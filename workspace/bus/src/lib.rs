mod handlers;
pub mod server;
mod test;
mod users;

pub mod common {
    include!(concat!(env!("OUT_DIR"), "/example.common.v1.rs"));
}
