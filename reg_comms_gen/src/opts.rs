use clap::Parser;

#[derive(Parser)]
pub struct Opts {
    pub yamlfile: String,
    pub output_directory: String,
    #[arg(short, long)]
    pub reg_comms_path: Option<String>,
    #[arg(short, long, default_value_t = false)]
    pub src_only: bool,
}
