use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dAuth Service", about = "An instance of the dAuth Service")]
pub struct DauthOpt {
    /// Yaml config file path
    #[structopt(parse(from_os_str))]
    pub config_path: PathBuf,
}
