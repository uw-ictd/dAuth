## dAuth Service

### Installation
- All dependencies are managed through cargo -- simply run `cargo build`.

### Running
- For the main service, run `cargo run <config path>`.
- For the cli, run `cargo run --bin cli <config path>`.
- Sample configs with documentation are available in `/configs`.

### Quick Info
- The main service requires a running instance of the directory service. Without it, there is no method for finding other instances of dAuth.
- The cli requires a running instance of dAuth. This is due in equal parts to database managament and the complexities of sharing new users across instances of dAuth.
- dAuth does not need a 4G/5G core to function correctly. While dAuth is meant to work on top of a core, the core itself has a "consumer" role and dAuth simply provides auth info on demand. All other functionalities (auth info distribution and management) can proceed normally with or without a core.
