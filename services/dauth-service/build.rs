fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(
            &[
                "../../protos/remote_authentication.proto",
                "../../protos/local_authentication.proto",
                "../../protos/management.proto",
                "../../protos/directory.proto",
            ],
            &["../../protos"],
        )?;
    Ok(())
}
