mod register_spec;
mod field_spec;
mod peripheral_spec;
mod endian;
mod opts;

use opts::Opts;
use std::fs::File;
use std::io::{BufReader, Write};
use clap::Parser;
use peripheral_spec::PeripheralSpec;

/*
use register_spec::RegisterSpec;
use field_spec::{FieldSpec, FieldPost};
fn test_routine() {
    let peripheral_spec = PeripheralSpec {
        name: "MyPeripheral".to_string(),
        byte_order: Endian::Big,
        address_len: 2,
        registers: vec![
                RegisterSpec {
                name: "test_reg".to_string(),
                address: 0x0f,
                size: 1,
                readable: true,
                writable: false,
                reset_val: None,
                fields: vec![
                    FieldSpec {
                        name: "test_good".to_string(),
                        field_pos: FieldPos::Bit(0),
                    },
                    FieldSpec {
                        name: "num_tests".to_string(),
                        field_pos: FieldPos::Field(3, 1),
                    },
                ],
                access_proc: None,
            }
        ]
    };
    let yaml_string = serde_yaml::to_string(&peripheral_spec).unwrap();
    println!("yaml registers:\n{}", yaml_string);
    let parsed_back): PeripheralSpec = serde_yaml::from_str(&yaml_string).unwrap();
    println!("Parsed back: {:?}", parsed_back);
    let spec_string = r#"
name: MyPeripheral
byte_order: Big
registers:
- name: test_reg
  address: 15
  size: 1
  readable: true
  writable: true
  reset_val: null
  fields:
  - name: test_good
    field_pos: '0'
  - name: num_tests
    field_pos: '[3:1]'
  access_proc: null
"#;
    let parsed: PeripheralSpec = serde_yaml::from_str(spec_string).unwrap();
    let codegen = parsed.registers[0].generate_file(&parsed);
    println!("Code generated:\n{}", codegen);
    let librs = parsed.generate_librs();
    println!("Code generated:\n{}", librs);
}*/

fn main() {
    let opts = Opts::parse();
    let yaml_file = File::open(&opts.yamlfile).unwrap();
    let yaml_reader = BufReader::new(yaml_file);
    let peripheral_spec: PeripheralSpec = serde_yaml::from_reader(yaml_reader).unwrap();
    let peripheral_mod = peripheral_spec.generate_module();
    let _ = std::fs::remove_dir_all(&opts.output_directory);
    std::fs::create_dir_all(format!("{}/src", &opts.output_directory)).unwrap();
    for (filename, file_contents) in peripheral_mod {
        let mut outfile = File::create(format!("{}/src/{}", &opts.output_directory, filename)).unwrap();
        outfile.write_all(file_contents.as_bytes()).unwrap();
    }
    let mut outfile = File::create(format!("{}/Cargo.toml", &opts.output_directory)).unwrap();
    outfile.write_all(peripheral_spec.generate_cargo_toml(opts.reg_comms_path.clone()).as_bytes()).unwrap();
}
