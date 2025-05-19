mod opts;

use opts::Opts;
use clap::Parser;
use regcommsgen::{
    generate_src_dir,
    generate_cargo_toml,
    generate_crate,
    read_peripheral_spec,
};

fn main() {
    let opts = Opts::parse();
    if opts.src_only {
        let pspec = read_peripheral_spec(&opts.pspec_yaml);
        let mut src_dir_path = opts.crate_directory.clone();
        src_dir_path.push("src");
        generate_src_dir(&pspec, &src_dir_path);
    } else if opts.cargo_only {
        let pspec = read_peripheral_spec(&opts.pspec_yaml);
        generate_cargo_toml(&pspec, &opts.crate_directory, opts.reg_comms_override);
    } else {
        generate_crate(&opts.pspec_yaml, &opts.crate_directory, opts.reg_comms_override);
    }
}
