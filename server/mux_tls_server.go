package main

import (
	"context"
	"crypto/tls"
	"log"
	"net"

	"github.com/soheilhy/cmux"
	"google.golang.org/grpc"

	pb "grpc-con/pb"
)

type tlsServer struct {
	pb.UnimplementedPingPongServiceServer
}

func (s *tlsServer) PingPong(ctx context.Context, in *pb.Ping) (*pb.Pong, error) {
	log.Printf("Received: %v", in.GetMessage())
	return &pb.Pong{Message: "Pong"}, nil
}

// Custom TLS GetConfigForClient function to log client info
func getConfigForClient(info *tls.ClientHelloInfo) (*tls.Config, error) {
	log.Printf("Client SNI: %v", info.ServerName)
	log.Printf("Client Requested Protocols: %v", info.SupportedProtos)
	return nil, nil
}

func main() {
	ctx := context.Background()
	//
	cer, err := GenerateX509KeyPair()
	if err != nil {
		panic(err)
	}

	tlsConfig := &tls.Config{
		Certificates:       []tls.Certificate{*cer},
		MinVersion:         tls.VersionTLS12,
		GetConfigForClient: getConfigForClient,
	}
	//creds := credentials.NewTLS(tlsConfig)

	lis, err := net.Listen("tcp", "localhost:50051")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	lis = tls.NewListener(lis, tlsConfig)
	tcpm := cmux.New(lis)
	grpcL := tcpm.Match(cmux.Any())

	go func() {
		s := grpc.NewServer()
		pb.RegisterPingPongServiceServer(s, &tlsServer{})
		//reflection.Register(s)

		log.Printf("Server listening at %v", lis.Addr())
		if err := s.Serve(lis); err != nil {
			log.Fatalf("failed to serve: %v", err)
		}
		_ = s.Serve(grpcL)
	}()
	go func() {
		_ = tcpm.Serve()
	}()

	<-ctx.Done()

}
