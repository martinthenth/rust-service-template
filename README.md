# rust-service-template

## Introduction

This is a Rust Service template that you can copy to implement Microservices. It contains modern tools like:

- HTTP with GraphQL (a.k.a. Web Server)
- gRPC with Protobuf (a.k.a. RPC Server)
- Message Consumer with Kafka (a.k.a. Bus Server)
- Database with Postgres as Primary Datastore
- OpenTelemetry with Jaeger for Instrumentation

## Installation

1. Install `asdf` with `buf`, `protoc`, `rust`, and `task` plugins
2. Install `docker`, preferably Docker Desktop
3. Run `asdf install` to install languages and tools
4. Run `docker-compose up -d` to start datastores and telemetry
5. Run `task install` to install Rust tools
6. Run `task reset` to create and migrate databases
7. (Optional): Run `./register-debezium.sh` to start Change Data Capture with Kafka

## How to run

- Run `cargo run -p web` to start the Web Server
- Run `cargo run -p rpc` to start the RPC Server
- Run `cargo run -p bus` to start the Bus Server

## Manual testing

- Run `grpcurl -plaintext -import-path ./protos -proto protos/example/users/v1/rpc/users_service.proto '0.0.0.0:50051' example.users.v1.rpc.UsersService/GetUser` from the repository root

## Continuous integration

A basic GitHub Action Workflow is present to help you set up your own CI Runner.
