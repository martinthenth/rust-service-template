use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &["../../protos/example/users/v1/events/user_created.proto"],
        &["../../protos"],
    )?;
    prost_build::compile_protos(
        &["../../protos/example/common/v1/envelope.proto"],
        &["../../protos"],
    )?;

    Ok(())
}
