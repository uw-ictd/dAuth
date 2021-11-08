pub mod clients;
mod handler;
pub mod server;

pub mod d_auth {
    tonic::include_proto!("d_auth");
}
