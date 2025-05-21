mod register_spec;
mod field_spec;
mod peripheral_spec;
mod access_proc;
mod endian;

use std::fs::File;
use std::io::{BufReader, Write};
use peripheral_spec::PeripheralSpec;
use std::convert::AsRef;
use std::path::Path;
use std::fs;

pub fn generate_src_dir<P: AsRef<Path>>(pspec: &PeripheralSpec, src_dir: P) {
    let dir_path = src_dir.as_ref().to_path_buf();
    if !fs::metadata(&dir_path)
        .expect(&format!("Failed to get fs metadata for 'src' dir path: {:?}", dir_path))
        .is_dir() {
        panic!("Cannot generate peripheral module: Got bad 'src' dir path {:?}", dir_path);
    }
    let peripheral_module = pspec.generate_module();
    for (filename, contents) in peripheral_module {
        let mut outfile_path = dir_path.clone();
        outfile_path.push(&filename);
        let mut outfile = File::create(&outfile_path).expect(&format!("generate_srcdir: Failed to create file at path: {:?}", outfile_path));
        outfile.write_all(contents.as_bytes()).expect(&format!("generate_srcdir: Failed to write all contents for file at path: {:?}", outfile_path));
    }
}

pub fn generate_cargo_toml<P: AsRef<Path>>(pspec: &PeripheralSpec, crate_dir: P, reg_comms_override: Option<String>) {
    let dir_path = crate_dir.as_ref().to_path_buf();
    if !fs::metadata(&dir_path)
        .expect(&format!("Failed to get fs metadata for 'crate' dir path: {:?}", dir_path))
        .is_dir() {
        panic!("Cannot generate Cargo.toml: got bad 'crate' dir path {:?}", dir_path);
    }
    let mut outfile_path = dir_path;
    outfile_path.push("Cargo.toml");
    let mut outfile = File::create(&outfile_path).expect(&format!("generate_cargo_toml: Failed to create file at path: {:?}", outfile_path));
    let contents = pspec.generate_cargo_toml(reg_comms_override);
    outfile.write_all(contents.as_bytes()).expect(&format!("generate_cargo_toml: Failed to write all contents for file at path: {:?}", outfile_path));
}

pub fn read_peripheral_spec<P: AsRef<Path>>(pspec_path: P) -> PeripheralSpec {
    let yaml_path = pspec_path.as_ref();
    let yaml_file = BufReader::new(File::open(yaml_path).expect(&format!("Failed to open peripheral spec file at path: {:?}", yaml_path)));
    let peripheral_spec: PeripheralSpec = serde_yaml::from_reader(yaml_file).expect(&format!("Failed to parse peripheral_spec as yaml: {:?}", yaml_path));
    peripheral_spec
}

pub fn generate_crate<Path0: AsRef<Path>, Path1: AsRef<Path>>(spec_path: Path0, crate_path: Path1, reg_comms_override: Option<String>) {
    let crate_p = crate_path.as_ref();
    if !fs::metadata(&crate_p)
        .expect(&format!("generate_crate: Failed to get fs metadata for 'crate' dir path: {:?}", crate_p))
        .is_dir() {
        panic!("Cannot generate Cargo.toml: got bad 'crate' dir path {:?}", crate_p);
    }

    let peripheral_spec = read_peripheral_spec(spec_path);
    let mut src_path = crate_p.to_path_buf();
    src_path.push("src");
    let _ = std::fs::create_dir(&src_path);
    generate_src_dir(&peripheral_spec, src_path);
    generate_cargo_toml(&peripheral_spec, crate_path, reg_comms_override);
}
