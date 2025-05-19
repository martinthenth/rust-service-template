# Rust Service Template

<!-- TODO: Review and improve README.md with ChatGPT -->

## Introduction

This is a Rust Service template that you can copy to implement Microservices. It contains modern tools like:

- HTTP with GraphQL (a.k.a. Web Server)
- gRPC with Protobuf (a.k.a. RPC Server)
- Message Consumer with Kafka (a.k.a. Bus Server)
- Database with Postgres as Primary Datastore
- OpenTelemetry with Jaeger for Instrumentation

## Installation

Pre-requisites:

1. [asdf](https://asdf-vm.com) version manager for languages and tools.
<!-- TODO: Look up URLs -->
2. [rust](), [protoc](), [task](), [buf]() plugins for asdf.
3. [docker](https://www.docker.com) to start containers

Then run:

```zsh
asdf install
docker-compose up -d
task install
task migrate
./register-debezium.sh
```

If you get an error that command `sqlx` cannot be found, run: `asdf reshim`.

## How to run

To start the Web Server:

```zsh
cargo run -p web
```

To start the RPC Server:

```zsh
cargo run -p rpc
```

To start the Bus Server:

```zsh
cargo run -p bus
```

## Manual testing

To test the Web Server along with Bus Server:

```zsh
curl -X POST http://localhost:4000/graph \
  -H "Content-Type: application/json" \
  -d '{
    "query": "mutation CreateUser($input: CreateUserInput!) { createUser(input: $input) { id firstName lastName createdAt } }",
    "variables": {
      "input": {
        "firstName": "John",
        "lastName": "Doe"
      }
    }
  }'
```

To test the RPC Server:

```zsh
grpcurl -plaintext -import-path ./protos -proto protos/example/users/v1/rpc/users_service.proto '0.0.0.0:50051' example.users.v1.rpc.UsersService/GetUser
```

## Continuous integration

A basic GitHub Action Workflow is present to help you set up your own CI Runner.
