pub mod clients;
mod handler;
pub mod server;

pub mod dauth {
    // It seems like as of 2021-11-27 the include_proto macro can only be
    // called once per module, which makes sense for keeping the imported
    // protos in separate namespaces.
    pub mod common {
        tonic::include_proto!("d_auth");
    }
    pub mod local {
        tonic::include_proto!("dauth_local");
    }
    pub mod remote {
        tonic::include_proto!("dauth_remote");
    }

}
