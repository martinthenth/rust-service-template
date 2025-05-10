# rust-service-template

## Introduction

This is a Rust Service template that you can copy to implement Microservices. It contains modern tools like:

- HTTP with GraphQL (a.k.a. Web Server)
- gRPC with Protobuf (a.k.a. RPC Server)
- Message Consumer with Kafka (a.k.a. Bus Server)
- Database with Postgres as Primary Datastore
- OpenTelemetry with Jaeger for Instrumentation

## Installation

1. Install `asdf` with `buf`, `protoc`, and `rust` plugins
2. Run `asdf install` to install languages and tools
3. Install `docker`, preferably Docker Desktop
4. Run `docker-compose up -d` to start datastores and telemetry

## How to run

- Run `cargo run -p web` to start the Web Server
- Run `cargo run -p rpc` to start the RPC Server
- Run `cargo run -p bus` to start the Bus Server

## Manual testing

- Run `grpcurl -plaintext -import-path ./protos -proto protos/example/users/v1/rpc/users.proto '[::1]:50051' example.users.v1.rpc.Users/GetUser` from the repository root
