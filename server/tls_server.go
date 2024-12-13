package main

import (
	"context"
	"crypto/tls"
	"log"
	"net"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"

	pb "grpc-con/pb"
)

type muxtlsServer struct {
	pb.UnimplementedPingPongServiceServer
}

func (s *muxtlsServer) PingPong(ctx context.Context, in *pb.Ping) (*pb.Pong, error) {
	log.Printf("Received: %v", in.GetMessage())
	return &pb.Pong{Message: "Pong"}, nil
}

// Custom TLS GetConfigForClient function to log client info
func getConfigForClient2(info *tls.ClientHelloInfo) (*tls.Config, error) {
	log.Printf("Client SNI: %v", info)
	log.Printf("Client Requested Protocols: %v", info.SupportedProtos)
	return nil, nil
}

func main() {
	cer, err := GenerateX509KeyPair()
	if err != nil {
		panic(err)
	}

	creds := credentials.NewTLS(&tls.Config{
		Certificates:       []tls.Certificate{*cer},
		MinVersion:         tls.VersionTLS12,
		GetConfigForClient: getConfigForClient2,
	})
	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	s := grpc.NewServer(grpc.Creds(creds))
	pb.RegisterPingPongServiceServer(s, &muxtlsServer{})
	//reflection.Register(s)

	log.Printf("Server listening at %v", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
