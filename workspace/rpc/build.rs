fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().build_server(true).compile_protos(
        &["../../protos/example/users/v1/rpc/users_service.proto"],
        &["../../protos"],
    )?;

    Ok(())
}
