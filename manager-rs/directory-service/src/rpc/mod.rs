pub mod handler;
pub mod server;

pub mod directory_service {
    tonic::include_proto!("dauth_directory");
}
