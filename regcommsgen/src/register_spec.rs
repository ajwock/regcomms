use serde::{Serialize, Deserialize};
use crate::field_spec::{FieldSpec, FieldPos};
use crate::peripheral_spec::PeripheralSpec;
use crate::endian::Endian;
use crate::struct_spec::StructSpec;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SpecialRegType {
    DataPort,
    Struct(StructSpec),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterSpec {
    pub name: String,
    pub address: u64,
    // Size in bytes, up to 8 bytes supported
    pub size: u8,
    pub readable: bool,
    pub writable: bool,
    pub reset_val: Option<u64>,
    pub fields: Option<Vec<FieldSpec>>,
    pub access_proc: Option<String>,
    // aliasable
    pub special_type: Option<SpecialRegType>,
}

impl RegisterSpec {

    pub fn reg_mod_name(&self) -> String {
        stringcase::snake_case(&self.name)
    }

    pub fn reg_method_name(&self) -> String {
        stringcase::snake_case(&self.name)
    }

    pub fn reg_struct_name(&self) -> String {
        stringcase::pascal_case(&self.name)
    }

    pub fn regval_struct_name(&self) -> String {
        format!("{}Val", stringcase::pascal_case(&self.name))
    }
    
    pub fn is_data_port(&self) -> bool {
        match self.special_type {
            Some(SpecialRegType::DataPort) => true,
            _ => false,
        }
    }

    // Note:  This may not be used in struct special reg type
    pub fn regval_word_size(&self) -> u8 {
        let len = self.size;
        if len <= 2 {
            len as u8
        } else if len <= 4 {
            4
        } else if len <= 8 {
            8
        } else {
            panic!("Invalid word size")
        }
    }

    // Note:  This may not be used in struct special reg type
    pub fn regval_word_name(&self) -> &'static str {
        match self.regval_word_size() {
            1 => "u8",
            2 => "u16",
            4 => "u32",
            8 => "u64",
            _ => panic!("Invalid word size from regval_word_size"),
        }
    }

    // Commsbuf subscript based on word size and endianness
    // If the word size is the same as the actual register size, then
    // we have no subscript
    pub fn commsbuf_subscript(&self, endian: Endian) -> String {
        let word_size = self.regval_word_size();
        let padding_len = word_size - self.size;
        let (low, high) = if padding_len == 0 {
            return "".to_string()
        } else if matches!(endian, Endian::Big) {
            (padding_len, word_size)
        } else {
            (0, self.size)
        };
        format!("[{low}..{high}]")
    }

    pub fn generate_file(&self, pspec: &PeripheralSpec) -> String {
        let mut out = String::new();
        out.push_str(&format!("use core::result::Result;\n"));
        out.push_str(&format!("use regcomms::{{RegCommsError, RegComms, RegCommsAccessProc}};\n"));
        out.push_str(&format!("use crate::{};\n", pspec.peripheral_struct_name()));
        out.push_str(&format!("pub struct {}<'a, {}>(pub &'a mut {});\n", self.reg_struct_name(), pspec.get_generics_string(), pspec.get_parameterized_typename()));
        out.push_str(&format!("impl<'a, {}> {}<'a, {}> {{\n", pspec.get_generics_string(), self.reg_struct_name(), pspec.get_boundfree_generics()));
        let endian = pspec.endian();
        if self.readable {
            out.push_str(&format!("    pub fn read(&mut self) -> Result<{}, RegCommsError> {{\n", self.regval_struct_name()));
            out.push_str(&format!("        let mut buf = [0u8; {}];\n", self.regval_word_size()));
            out.push_str(&format!("        let proc = self.0.{};\n", pspec.get_access_proc_member_name(&self.access_proc))); 
            out.push_str(&format!("        proc.proc_read(&mut self.0, 0x{:x}, &mut buf{})?;\n", self.address, self.commsbuf_subscript(endian)));
            out.push_str(&format!("        let val = {}::from_{}_bytes(buf);\n", self.regval_word_name(), endian.abbrev()));
            out.push_str(&format!("        Ok({}(val))\n", self.regval_struct_name()));
           out.push_str(&format!("    }}\n"));
            out.push_str(&format!("    pub async fn read_async(&mut self) -> Result<{}, RegCommsError> {{\n", self.regval_struct_name()));
            out.push_str(&format!("        let mut buf = [0u8; {}];\n", self.regval_word_size()));
            out.push_str(&format!("        let proc = self.0.{};\n", pspec.get_access_proc_member_name(&self.access_proc))); 
            out.push_str(&format!("        proc.proc_read_async(&mut self.0, 0x{:x}, &mut buf{}).await?;\n", self.address, self.commsbuf_subscript(endian)));
            out.push_str(&format!("        let val = {}::from_{}_bytes(buf);\n", self.regval_word_name(), endian.abbrev()));
            out.push_str(&format!("        Ok({}(val))\n", self.regval_struct_name()));
            out.push_str(&format!("    }}\n"));
        }
        if self.writable {
            out.push_str(&format!("    pub fn write(&mut self, val: {}) -> Result<(), RegCommsError> {{\n", self.regval_struct_name()));
            out.push_str(&format!("        let buf = val.0.to_be_bytes();\n"));
            out.push_str(&format!("        let proc = self.0.{};\n", pspec.get_access_proc_member_name(&self.access_proc))); 
            out.push_str(&format!("        proc.proc_write(&mut self.0, 0x{:x}, &buf{})?;\n", self.address, self.commsbuf_subscript(endian)));
            out.push_str(&format!("        Ok(())\n"));
            out.push_str(&format!("    }}\n"));
            out.push_str(&format!("    pub fn write_raw(&mut self, raw_val: {}) -> Result<(), RegCommsError> {{\n", self.regval_word_name()));
            out.push_str(&format!("        self.write({}(raw_val))\n", self.regval_struct_name()));
            out.push_str(&format!("    }}\n"));
            out.push_str(&format!("    pub async fn write_async(&mut self, val: {}) -> Result<(), RegCommsError> {{\n", self.regval_struct_name()));
            out.push_str(&format!("        let buf = val.0.to_be_bytes();\n"));
            out.push_str(&format!("        let proc = self.0.{};\n", pspec.get_access_proc_member_name(&self.access_proc))); 
            out.push_str(&format!("        proc.proc_write_async(&mut self.0, 0x{:x}, &buf{}).await?;\n", self.address, self.commsbuf_subscript(endian)));
            out.push_str(&format!("        Ok(())\n"));
            out.push_str(&format!("    }}\n"));
            out.push_str(&format!("    pub async fn write_raw_async(&mut self, raw_val: {}) -> Result<(), RegCommsError> {{\n", self.regval_word_name()));
            out.push_str(&format!("        self.write_async({}(raw_val)).await\n", self.regval_struct_name()));
            out.push_str(&format!("    }}\n"));

        }
        if self.readable && self.writable {
            out.push_str(&format!("    pub fn modify<F: FnOnce({}) -> {}>(&mut self, f: F) -> Result<(), RegCommsError> {{\n", self.regval_struct_name(), self.regval_struct_name()));
            out.push_str(&format!("        let orig_val = self.read()?;\n"));
            out.push_str(&format!("        self.write(f(orig_val))\n"));
            out.push_str(&format!("    }}\n"));
            out.push_str(&format!("    pub async fn modify_async<F: FnOnce({}) -> {}>(&mut self, f: F) -> Result<(), RegCommsError> {{\n", self.regval_struct_name(), self.regval_struct_name()));
            out.push_str(&format!("        let orig_val = self.read_async().await?;\n"));
            out.push_str(&format!("        self.write_async(f(orig_val)).await\n"));
            out.push_str(&format!("    }}\n"));

        }
        if self.writable {
            if let Some(val) = self.reset_val {
                out.push_str(&format!("    pub fn reset(&mut self) -> Result<(), RegCommsError> {{\n"));
                out.push_str(&format!("        self.write({}(0x{:x}))\n", self.regval_struct_name(), val));
                out.push_str(&format!("    }}\n"));
                out.push_str(&format!("    pub async fn reset_async(&mut self) -> Result<(), RegCommsError> {{\n"));
                out.push_str(&format!("        self.write_async({}(0x{:x})).await\n", self.regval_struct_name(), val));
                out.push_str(&format!("    }}\n"));
            }
        }
        if self.is_data_port() && self.readable {
            if self.size != 1 {
                panic!("Data port only supported for size 1 registers now");
            }
            out.push_str(&format!("    pub fn data_port_read(&mut self, buf: &mut [u8]) -> Result<usize, RegCommsError> {{\n"));
            out.push_str(&format!("        let proc = self.0.{};\n", pspec.get_access_proc_member_name(&self.access_proc)));
            out.push_str(&format!("        proc.proc_read(&mut self.0, 0x{:x}, buf)\n", self.address));
            out.push_str(&format!("    }}\n"));
            out.push_str(&format!("    pub async fn data_port_read_async(&mut self, buf: &mut [u8]) -> Result<usize, RegCommsError> {{\n"));
            out.push_str(&format!("        let proc = self.0.{};\n", pspec.get_access_proc_member_name(&self.access_proc)));
            out.push_str(&format!("        proc.proc_read_async(&mut self.0, 0x{:x}, buf).await\n", self.address));
            out.push_str(&format!("    }}\n"));
        }
        if self.is_data_port() && self.writable {
            if self.size != 1 {
                panic!("Data port only supported for size 1 registers now");
            }
            out.push_str(&format!("    pub fn data_port_write(&mut self, buf: &[u8]) -> Result<usize, RegCommsError> {{\n"));
            out.push_str(&format!("        let proc = self.0.{};\n", pspec.get_access_proc_member_name(&self.access_proc)));
            out.push_str(&format!("        proc.proc_write(&mut self.0, 0x{:x}, buf)\n", self.address));
            out.push_str(&format!("    }}\n"));
            out.push_str(&format!("    pub async fn data_port_write_async(&mut self, buf: &[u8]) -> Result<usize, RegCommsError> {{\n"));
            out.push_str(&format!("        let proc = self.0.{};\n", pspec.get_access_proc_member_name(&self.access_proc)));
            out.push_str(&format!("        proc.proc_write_async(&mut self.0, 0x{:x}, buf).await\n", self.address));
            out.push_str(&format!("    }}\n"));
        }
        out.push_str(&format!("}}\n"));
        out.push_str(&self.generate_regval_struct());
        out
    }

    pub fn generate_regval_struct(&self) -> String {
        let mut out = String::new();
        // Regval struct generation
        out.push_str(&format!("pub struct {}(pub {});\n", self.regval_struct_name(), self.regval_word_name()));
        out.push_str(&format!("impl {} {{\n", self.regval_struct_name()));
        out.push_str(&format!("    pub fn get(&self) -> {} {{\n", self.regval_word_name()));
        out.push_str(&format!("        self.0\n"));
        out.push_str(&format!("    }}\n"));
        if self.writable {
            out.push_str(&format!("    pub fn zero() -> Self {{\n"));
            out.push_str(&format!("        Self(0)\n"));
            out.push_str(&format!("    }}\n"));
            out.push_str(&format!("    pub fn set(&mut self, val: {}) {{\n", self.regval_word_name()));
            out.push_str(&format!("        self.0 = val;\n"));
            out.push_str(&format!("    }}\n"));
        }
        if let Some(val) = self.reset_val {
            out.push_str(&format!("    pub fn reset_val() -> Self {{\n"));
            out.push_str(&format!("        Self(0x{:x})\n", val));
            out.push_str(&format!("    }}\n"));
        }
        for field in self.fields.as_ref().into_iter().flatten() {
            out.push_str(&format!("    pub fn {}<'a>(&'a mut self) -> {}<'a> {{\n", field.method_name(), field.struct_name()));
            out.push_str(&format!("        {}(self)\n", field.struct_name()));
            out.push_str(&format!("    }}\n"));
        }
        out.push_str(&format!("}}\n"));

        // Field struct generation
        for field in self.fields.as_ref().into_iter().flatten() {
            out.push_str(&format!("pub struct {}<'a>(pub &'a mut {});\n", field.struct_name(), self.regval_struct_name()));
            out.push_str(&format!("impl<'a> {}<'a> {{\n", field.struct_name()));
            match field.field_pos {
                FieldPos::Bit(bit_pos) => {
                    if self.readable {
                        out.push_str(&format!("    pub fn bit(&self) -> bool {{\n"));
                        out.push_str(&format!("        ((self.0.0 >> {}) & 1) != 0\n", bit_pos));
                        out.push_str(&format!("    }}\n"));
                        out.push_str(&format!("    pub fn bit_is_set(&self) -> bool {{\n"));
                        out.push_str(&format!("        self.bit()\n"));
                        out.push_str(&format!("    }}\n"));
                    }
                    if self.writable {
                        out.push_str(&format!("    pub fn assign(self, val: bool) -> &'a mut {} {{\n", self.regval_struct_name()));
                        out.push_str(&format!("        self.0.0 &= !(1 << {});\n", bit_pos));
                        out.push_str(&format!("        self.0.0 |= (val as {}) << {};\n", self.regval_word_name(), bit_pos));
                        out.push_str(&format!("        self.0\n"));
                        out.push_str(&format!("    }}\n"));
                        out.push_str(&format!("    pub fn set_bit(self) -> &'a mut {} {{\n", self.regval_struct_name()));
                        out.push_str(&format!("        self.assign(true)\n"));
                        out.push_str(&format!("    }}\n"));
                        out.push_str(&format!("    pub fn clear_bit(self) -> &'a mut {} {{\n", self.regval_struct_name()));
                        out.push_str(&format!("        self.assign(false)\n"));
                        out.push_str(&format!("    }}\n"));
                        if let Some(val) = self.reset_val {
                            out.push_str(&format!("    pub fn reset(self) -> &'a mut {} {{\n", self.regval_struct_name()));
                            out.push_str(&format!("        self.0.0 &= !(1 << {});\n", bit_pos));
                            out.push_str(&format!("        self.0.0 |= (1 << {}) & 0x{:x};\n", bit_pos, val));
                            out.push_str(&format!("        self.0\n"));
                            out.push_str(&format!("    }}\n"));
                        }
                    }
                }
                FieldPos::Field(high, low) => {
                    let field_len = high - low + 1;
                    if self.readable {
                        out.push_str(&format!("    pub fn bits(&self) -> {} {{\n", field.field_pos.fieldpos_word()));
                        if field_len == self.regval_word_size() * 8 {
                            out.push_str(&format!("        self.0.0\n"));
                        } else {
                            out.push_str(&format!("        ((self.0.0 >> {}) & !(!0 << {})) as {}\n", low, field_len, field.field_pos.fieldpos_word()));
                        }
                        out.push_str(&format!("    }}\n"));
                    }
                    if self.writable {
                        out.push_str(&format!("    pub fn set(self, val: {}) -> &'a mut {} {{\n", field.field_pos.fieldpos_word(), self.regval_struct_name()));
                        if field_len == self.regval_word_size() * 8 {
                            out.push_str(&format!("        self.0.0 = val;\n"));
                        } else {
                            out.push_str(&format!("        self.0.0 &= !(!(!0 << {}) << {});\n", field_len, low));
                            out.push_str(&format!("        self.0.0 |= ((val as {}) & !(!0 << {})) << {};\n", self.regval_word_name(), field_len, low));

                        }
                        out.push_str(&format!("        self.0\n"));
                        out.push_str(&format!("    }}\n"));
                        if let Some(reset_val) = self.reset_val {
                                out.push_str(&format!("    pub fn reset(self) -> &'a mut {} {{\n", self.regval_struct_name()));
                            if field_len == self.regval_word_size() * 8 {
                                out.push_str(&format!("        self.0.0 = 0x{:x};\n", reset_val));
                            } else {
                                out.push_str(&format!("        self.0.0 &= !(!(!0 << {}) << {});\n", field_len, low));
                                out.push_str(&format!("        self.0.0 |= 0x{:x} & (!(!0 << {}) << {});\n", reset_val, field_len, low));
                            }
                            out.push_str(&format!("        self.0\n"));
                            out.push_str(&format!("    }}\n"));
                        }
                    }
                }
            }
            out.push_str(&format!("}}\n"));
        }
        out
    }

}
