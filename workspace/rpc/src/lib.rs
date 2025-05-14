pub mod handlers;
pub mod server;
mod test;

pub mod users {
    pub mod rpc {
        tonic::include_proto!("example.users.v1.rpc");
    }

    pub mod types {
        tonic::include_proto!("example.users.v1.types");
    }
}
