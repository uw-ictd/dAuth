pub mod clients;
pub mod handlers;
pub mod server;

pub mod dauth {
    // It seems like as of 2021-11-27 the include_proto macro can only be
    // called once per module, which makes sense for keeping the imported
    // protos in separate namespaces.

    // The common module is an alias the d_auth proto package for clarity for
    // outside users.
    pub mod common {
        pub use super::d_auth::*;
    }
    mod d_auth {
        tonic::include_proto!("d_auth");
    }
    pub mod local {
        tonic::include_proto!("dauth_local");
    }
    pub mod remote {
        tonic::include_proto!("dauth_remote");
    }
}
