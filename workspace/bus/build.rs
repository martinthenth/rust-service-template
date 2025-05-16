use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &["../../protos/example/users/v1/events/user_created.proto"],
        &["../../protos"],
    )?;

    Ok(())
}
