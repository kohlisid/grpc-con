use std::io::ErrorKind;
use std::sync::Arc;
use std::time::Duration;
use bytes::Bytes;
use hyper_rustls::HttpsConnector;
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client},
    rt::TokioExecutor,
};
use mvtxdaemon::mono_vertex_daemon_service_client::MonoVertexDaemonServiceClient;
use tonic::transport::Channel;
use rustls::{self, pki_types::CertificateDer, ClientConfig};
use std::net::SocketAddr;
use tonic::Status;
use rustclient::mvtxdaemon;

// // Generated module from protobuf definitions
pub mod pingpong {
    tonic::include_proto!("pingpong"); // The string specified here must match the proto package name
}

#[derive(Debug)]
struct SkipServerVerification;

// TLS server certificate verifier to accept self-signed certs when using rustls
impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::client::danger::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA1,
            rustls::SignatureScheme::ECDSA_SHA1_Legacy,
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::RSA_PKCS1_SHA384,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rustls::SignatureScheme::RSA_PKCS1_SHA512,
            rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
            rustls::SignatureScheme::RSA_PSS_SHA256,
            rustls::SignatureScheme::RSA_PSS_SHA384,
            rustls::SignatureScheme::RSA_PSS_SHA512,
            rustls::SignatureScheme::ED25519,
            rustls::SignatureScheme::ED448,
        ]
    }
}

type HTTPSClient = Client<
    HttpsConnector<HttpConnector>,
    http_body_util::combinators::UnsyncBoxBody<Bytes, Status>,
>;

// Creates an HTTPS client that can be used to connect to
pub fn new_https_client() -> Result<HTTPSClient, Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let tls = ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(SkipServerVerification::new())
        .with_no_client_auth();

    let mut http = HttpConnector::new();
    http.enforce_http(false);

    let connector = tower::ServiceBuilder::new()
        .layer_fn(move |s| {
            let tls = tls.clone();

            hyper_rustls::HttpsConnectorBuilder::new()
                .with_tls_config(tls)
                .https_only()
                .enable_http2()
                .wrap_connector(s)
        })
        .service(http);

    let client = hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build(connector);
    Ok(client)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let client = new_https_client().unwrap();
    let addr = tokio::net::lookup_host("localhost:4327")
        .await
        .unwrap()
        .find_map(|addr| {
            if let SocketAddr::V4(ip) = addr {
                Some(ip.to_string())
            } else {
                None
            }
        })
        .unwrap();

    // Wait for the service to be ready
    loop {
        match tokio::net::TcpStream::connect(&addr).await {
            Ok(_) => break,
            Err(err) => {
                if err.kind() == ErrorKind::ConnectionRefused {
                    println!("{} {}", addr, "Monovertex deamon server is not ready yet");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                } else {
                    panic!("{err:?}")
                }
            }
        }
    }

    let uri = hyper::Uri::builder()
        .authority(addr)
        .scheme("https")
        .path_and_query("/")
        .build()
        .unwrap();

    println!("Connecting to Server {}",uri);
    // let mut daemon_client = pingpong::ping_pong_service_client::PingPongServiceClient::with_origin(client, uri);
    // // Prepare a Ping request to send
    // let request = tonic::Request::new(pingpong::Ping {
    //     message: "Ping".into(),
    // });
    //
    // // Send the request and wait for a response
    // let response = daemon_client.ping_pong(request).await?;
    //
    // // Extract the message from Pong response and print it
    // println!("RESPONSE={:?}", response.into_inner().message);

    let mut daemon_client = MonoVertexDaemonServiceClient::with_origin(client, uri);

    let res = daemon_client
        .get_mono_vertex_status(tonic::Request::new(()))
        .await
        .expect("daemon should be up");
    let status = res.into_inner().status.expect("cannot be none").status;
    println!("mvtx daemon status {}", status);


    Ok(())

}