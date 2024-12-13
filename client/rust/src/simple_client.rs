use tonic::transport::Channel;
use tonic::Request;

// Generated module from protobuf definitions
pub mod pingpong {
    tonic::include_proto!("pingpong"); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the gRPC client
    let mut client = pingpong::ping_pong_service_client::PingPongServiceClient::connect("http://[::1]:50051").await?;

    // Prepare a Ping request to send
    let request = tonic::Request::new(pingpong::Ping {
        message: "Ping".into(),
    });

    // Send the request and wait for a response
    let response = client.ping_pong(request).await?;

    // Extract the message from Pong response and print it
    println!("RESPONSE={:?}", response.into_inner().message);

    Ok(())
}