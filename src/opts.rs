use clap::Parser;

#[derive(Parser)]
pub struct Opts {
    pub yamlfile: String,
    pub output_directory: String,
}
