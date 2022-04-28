use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Directory Service",
    about = "An instance of the Directory Service"
)]
pub struct DirectoryOpt {
    /// Yaml config file path
    #[structopt(parse(from_os_str))]
    pub config_path: PathBuf,
}
