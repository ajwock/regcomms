use serde::{Serialize, Deserialize};
use crate::register_spec::RegisterSpec;
use crate::peripheral_spec::PeripheralSpec;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StructSpec {
    pub struct_name: String,
    // A struct is composed of registers which are at an offset within
    // the struct indicated by their address.
    pub sub_registers: Vec<RegisterSpec>,
    pub bufsize: usize,
}

impl StructSpec {
    pub fn generate_struct_file(&self, pspec: &PeripheralSpec) -> String {
        let mut out = String::new();
        out.push_str(&format!("use core::result::Result;\n"));
        out.push_str(&format!("use regcomms::{{RegCommsError, RegComms, RegCommsAccessProc}};\n"));
        out.push_str(&format!("use crate::{};\n", pspec.peripheral_struct_name()));
        out.push_str(&format!("pub struct {}([u8;{}]);\n", self.struct_name, self.bufsize));
        out.push_str(&format!("impl {} {{\n", self.struct_name));
        for reg in self.sub_registers.iter() {
            out.push_str(&format!("    pub fn {}<'a>(&'a mut self) -> {}<'a> {{\n", reg.reg_method_name(), reg.reg_struct_name()));
            out.push_str(&format!("        {}(self)\n", reg.reg_struct_name()));
            out.push_str(&format!("    }}\n"));
        }
        out.push_str(&format!("}}\n"));
        out
    }
}
