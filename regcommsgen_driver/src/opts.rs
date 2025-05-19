use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Opts {
    pub pspec_yaml: PathBuf,
    pub crate_directory: PathBuf,
    #[arg(short, long)]
    pub reg_comms_override: Option<String>,
    #[arg(short, long, default_value_t = false)]
    pub src_only: bool,
    #[arg(short, long, default_value_t = false)]
    pub cargo_only: bool,
}
