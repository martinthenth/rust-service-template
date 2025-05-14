use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure().build_server(true).compile_protos(
        &["../../protos/example/users/v1/rpc/users.proto"],
        &["../../protos"],
    )?;

    Ok(())
}
