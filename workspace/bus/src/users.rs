pub mod events {
    include!(concat!(env!("OUT_DIR"), "/example.users.v1.events.rs"));
}

pub mod types {
    include!(concat!(env!("OUT_DIR"), "/example.users.v1.types.rs"));
}
