# 🦀 Rust Service Template

<!-- TODO: Review and improve README.md with ChatGPT -->

## Introduction

A modern template for building production-ready **Rust microservices**. Clone this project to kickstart services with:

- **GraphQL over HTTP** using `axum` (`web`)
- **gRPC with Protobuf** using `tonic` (`rpc`)
- **Kafka consumer** using `rdkafka` and `sea-streamer` (`bus`)
- **Kafka producer** using `debezium` using the **Outbox Pattern**
- **PostgreSQL** with `sqlx` and `sea-query`
- **Observability** with `opentelemetry` and Jaeger
- **Async runtime** powered by `tokio`

Ideal for software engineers looking to quickly scaffold scalable and testable microservices in Rust.

## 🚀 Getting Started

### Pre-requisites:

Install the following tools:

1. [asdf](https://asdf-vm.com) version manager
2. Required `asdf` plugins:
   - [rust](https://github.com/code-lever/asdf-rust.git)
   - [protoc](https://github.com/paxosglobal/asdf-protoc.git)
   - [task](https://github.com/particledecay/asdf-task.git)
   - [buf](https://github.com/truepay/asdf-buf.git) (optional for testing)
   - [grpcurl](https://github.com/asdf-community/asdf-grpcurl) (optional for testing)
3. [Docker](https://www.docker.com) for running dependencies (Postgres, Kafka, Jaeger, etc.)

### Setup

Run the following:

```sh
docker-compose up -d           # Start database, Kafka, Jaeger, etc.
asdf install                   # Install tool versions from .tool-versions
task install                   # Install dependencies and tooling
task migrate                   # Run database migrations
./register-debezium.sh         # Register Debezium for CDC (optional)
```

If you see an error about `sqlx` not found, run:

```sh
asdf reshim
```

## 🧪 Running Services

Start individual services using Cargo:

- Web Server:
  ```sh
  cargo run -p web
  ```
- RPC Server:
  ```sh
  cargo run -p rpc
  ```
- Bus Server:
  ```sh
  cargo run -p bus
  ```

## 🔍 Manual Testing

### Web Server (GraphQL)

Send a mutation to the GraphQL endpoint:

```sh
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

This will trigger both a database write and a message to Kafka (via the outbox table).

### RPC Server (gRPC)

Call the gRPC GetUser method using grpcurl:

```sh
grpcurl -plaintext \
  -import-path ./protos \
  -proto protos/example/users/v1/rpc/users.proto \
  -d '{ "id": "<YOUR USER ID>" }' \
  0.0.0.0:50051 \
  example.users.v1.rpc.Users/GetUser
```

### Bus Server (Kafka Consumer)

To test the Kafka consumer flow:

1. Run the Debezium registration script:
   ```sh
   ./register-debezium.sh
   ```
2. Start the bus server:
   ```sh
   cargo run -p bus
   ```
3. Trigger a GraphQL mutation (as shown above under Web Server).

The Bus Server will consume the message published by Debezium and process it.

## ⚙️ Continuous Integration

A basic GitHub Actions workflow is available in `.github/workflows/ci.yml`. It includes steps for:

- Formatting and linting
- Running tests

## 🧱 Project Structure

```sh
.
├── workspace/
│   ├── base/             # Business logic
│   ├── web/              # GraphQL server
│   ├── rpc/              # gRPC server
│   ├── bus/              # Kafka consumer
│   └── meta/             # Test macros
├── protos/               # Protobuf definitions
├── migrations/           # Database migrations
├── docs/                 # Service documentation
├── buf.yaml              # Protobuf linter
├── docker-compose.yaml   # Docker containers
├── register-debezium.sh  # Debezium connector
├── taskfile.yaml         # Task runner commands
├── .github               # GitHub Actions
└── .tool-versions        # asdf tool versions
```

## 🤝 Contributing

This project is a template and not meant for external PRs.
Feel free to fork it and make it your own!
