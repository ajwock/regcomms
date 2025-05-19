use serde::{Serialize, Deserialize};
use crate::register_spec::RegisterSpec;
use crate::endian::Endian;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PeripheralSpec {
    pub name: String,
    pub address_len: u8,
    pub byte_order: Endian,
    pub registers: Vec<RegisterSpec>,
}

impl PeripheralSpec {
    pub fn peripheral_struct_name(&self) -> String {
        stringcase::pascal_case(&self.name)
    }

    pub fn peripheral_mod_name(&self) -> String {
        stringcase::snake_case(&self.name)
    }

    pub fn endian(&self) -> Endian {
        self.byte_order
    }

    pub fn address_word_size(&self) -> u8 {
        match self.address_len {
            1 => 1,
            2 => 2,
            4 => 4,
            8 => 8,
            _ => panic!("Invalid word size from address_word_size"),
        }

    }

    pub fn address_word_name(&self) -> &'static str {
        match self.address_word_size() {
            1 => "u8",
            2 => "u16",
            4 => "u32",
            8 => "u64",
            _ => panic!("Invalid word size from address_word_size"),
        }
    }

    pub fn regcomms_params(&self) -> String {
        format!("<{}, {}>", self.address_word_size(), self.address_word_name())
    }

    pub fn generate_librs(&self) -> String {
        let mut out = String::new();
        for register in self.registers.iter() {
            out.push_str(&format!("mod {};\n", register.reg_mod_name()));
        }
        out.push_str(&format!("use regcomms::{{RegComms, RegCommsError}};\n"));
        out.push_str(&format!("pub enum AccessProc {{\n"));
        out.push_str(&format!("    Standard,\n"));
        out.push_str(&format!("}}\n"));
        out.push_str(&format!("pub struct {}<C: RegComms<{}, {}>>(pub C);\n", self.peripheral_struct_name(), self.address_word_size(), self.address_word_name()));
        out.push_str(&format!("impl<C: RegComms{}> {}<C> {{\n", self.regcomms_params(), self.peripheral_struct_name()));
        out.push_str(&format!("    pub fn comms_read(&mut self, reg_address: {}, buf: &mut [u8], _access_proc: AccessProc) -> Result<(), RegCommsError> {{\n", self.address_word_name()));
        out.push_str(&format!("        self.0.comms_read(reg_address, buf)\n"));
        out.push_str(&format!("    }}\n"));
        out.push_str(&format!("    pub fn comms_write(&mut self, reg_address: {}, buf: &[u8], _access_proc: AccessProc) -> Result<(), RegCommsError> {{\n", self.address_word_name()));
        out.push_str(&format!("        self.0.comms_write(reg_address, buf)\n"));
        out.push_str(&format!("    }}\n"));
        out.push_str(&format!("    pub async fn comms_read_async(&mut self, reg_address: {}, buf: &mut [u8], _access_proc: AccessProc) -> Result<(), RegCommsError> {{\n", self.address_word_name()));
        out.push_str(&format!("        self.0.comms_read_async(reg_address, buf).await\n"));
        out.push_str(&format!("    }}\n"));
        out.push_str(&format!("    pub async fn comms_write_async(&mut self, reg_address: {}, buf: &[u8], _access_proc: AccessProc) -> Result<(), RegCommsError> {{\n", self.address_word_name()));
        out.push_str(&format!("        self.0.comms_write_async(reg_address, buf).await\n"));
        out.push_str(&format!("    }}\n"));

        for reg in self.registers.iter() {
            out.push_str(&format!("    pub fn {}<'a>(&'a mut self) -> {}::{}<'a, C> {{\n", reg.reg_method_name(), reg.reg_mod_name(), reg.reg_struct_name()));
            out.push_str(&format!("        {}::{}(self)\n", reg.reg_mod_name(), reg.reg_struct_name()));
            out.push_str(&format!("    }}\n"));
        }
        out.push_str(&format!("}}\n"));
        out
    }

    pub fn generate_module(&self) -> Vec<(String, String)> {
        let mut out = Vec::new();
        out.push((String::from("lib.rs"), self.generate_librs()));
        for register in self.registers.iter() {
            let register_source = register.generate_file(&self);
            let register_source_name = format!("{}.rs", register.reg_mod_name());
            out.push((register_source_name, register_source));
        }
        out
    }

    pub fn generate_cargo_toml(&self, regcomms_override: Option<String>) -> String {
        let mut out = String::new();
        out.push_str(&format!("[package]\n"));
        out.push_str(&format!("name = \"{}\"\n", self.peripheral_mod_name()));
        out.push_str(&format!("edition = \"2024\"\n"));
        out.push_str(&format!("version = \"0.1.0\"\n\n"));
        out.push_str(&format!("[dependencies]\n"));
        let rc_configs = regcomms_override.unwrap_or("{{ }}".to_string());
        out.push_str(&format!("regcomms = {}\n", rc_configs));
        out
    }
}
