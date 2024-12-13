package main

import (
	"context"
	"log"
	"net"

	"google.golang.org/grpc"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/reflection"

	pb "grpc-con/pb"
)

type server struct {
	pb.UnimplementedPingPongServiceServer
}

func (s *server) PingPong(ctx context.Context, in *pb.Ping) (*pb.Pong, error) {
	log.Printf("Received: %v", in.GetMessage())
	return &pb.Pong{Message: "Pong"}, nil
}

// serverInterceptor logs the protocol from metadata
func serverInterceptor(ctx context.Context, req interface{}, info *grpc.UnaryServerInfo, handler grpc.UnaryHandler) (interface{}, error) {
	if md, ok := metadata.FromIncomingContext(ctx); ok {
		log.Println("Metadata:", md)
		if val, exists := md[":scheme"]; exists {
			log.Printf("Protocol used: %v", val[0])
		}
	}
	return handler(ctx, req)
}

func main() {
	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	opts := []grpc.ServerOption{
		grpc.UnaryInterceptor(serverInterceptor),
	}
	s := grpc.NewServer(opts...)
	pb.RegisterPingPongServiceServer(s, &server{})
	reflection.Register(s)
	log.Printf("Server listening at %v", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
