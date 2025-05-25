use serde::{Serialize, Deserialize};
use crate::register_spec::RegisterSpec;
use crate::endian::Endian;
use crate::access_proc::AccessProcSpec;
use crate::trait_member::TraitMember;
use crate::struct_spec::StructSpec;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PeripheralSpec {
    pub name: String,
    pub address_len: u8,
    pub byte_order: Endian,
    pub registers: Vec<RegisterSpec>,
    // Map from name for an enum variant for AccessProc to fully qualified function name taking
    // the named peripheral, reg address, and buffer
    pub non_standard_access_procs: Option<Vec<AccessProcSpec>>,
    // Extra mods that may need to be included for non_standard_procs
    pub extra_mods: Option<Vec<String>>,
    // Generate a member for the peripheral to hold and take as constructor argument.
    // It will be a generic with the given trait bound.
    // Useful for access procs
    // e.g. embedded_hal_async::delay::DelayNs
    pub trait_members: Option<Vec<TraitMember>>,
    pub struct_defns: Option<Vec<StructSpec>>,
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

    fn get_standard_access_proc_spec(&self) -> AccessProcSpec {
        AccessProcSpec {
            proc_name: String::from("Standard"),
            struct_path: String::from("StandardAccessProc"),
        }
    }

    pub fn get_access_proc_member_name(&self, access_proc_maybe: &Option<String>) -> String {
        if let Some(access_proc_name) = access_proc_maybe {
            let procs_map = self.get_access_procs_map();
            let Some(access_proc) = procs_map.iter().find(|x| x.proc_name.as_str() == access_proc_name) else {
                panic!("Got nonstandard access proc \'{access_proc_name}\' that does not match any access proc in peripheral proc list: {:?}", self.get_access_procs_map());
            };
            access_proc.member_name()
        } else {
            self.get_standard_access_proc_spec().member_name()
        }
    }

    fn get_access_procs_map(&self) -> Vec<AccessProcSpec> {
        let mut full_list = if let Some(ref list) = self.non_standard_access_procs {
            list.clone()
        } else {
            Vec::new()
        };
        full_list.push(self.get_standard_access_proc_spec());
        full_list
    }

    fn get_regcomms_trait_member(&self) -> TraitMember {
        TraitMember {
            name: "comms".to_string(),
            generic_type: "C".to_string(),
            trait_bound: format!("RegComms{}", self.regcomms_params()),
        }
    }

    fn get_trait_members_list(&self) -> Vec<TraitMember> {
        let mut full_list = if let Some(ref list) = self.trait_members {
            list.clone()
        } else {
            Vec::new()
        };
        full_list.push(self.get_regcomms_trait_member());
        full_list
    }

    pub fn get_generics_string(&self) -> String {
        itertools::join(
            self.get_trait_members_list().iter()
                .map(|t| format!("{}: {}", t.generic(), t.bound())),
        ", ")
    }

    pub fn get_boundfree_generics(&self) -> String {
        itertools::join(
            self.get_trait_members_list().iter()
                .map(|t| t.generic()),
        ", ")
    }

    pub fn get_constructor_args_list(&self) -> String {
        itertools::join(
            self.get_trait_members_list().iter()
                .map(|t| format!("{}: {}", t.member_name(), t.generic())),
        ", ")
    }
    
    pub fn get_parameterized_typename(&self) -> String {
        format!("{}<{}>", self.peripheral_struct_name(), self.get_boundfree_generics())
    }

    pub fn generate_librs(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("#![no_std]\n"));
        out.push_str(&format!("use core::result::Result;\n"));
        out.push_str(&format!("use core::default::Default;\n"));
        for register in self.registers.iter() {
            out.push_str(&format!("mod {};\n", register.reg_mod_name()));
        }
        for module in self.extra_mods.clone().unwrap_or(Vec::new()).iter() {
            out.push_str(&format!("mod {};\n", module));
        }
        out.push_str(&format!("use regcomms::{{RegComms, RegCommsError, RegCommsAccessProc}};\n"));
        out.push_str(&format!("use spin::once::Once;\n"));
        let standard = self.get_standard_access_proc_spec();
        out.push_str(&format!("#[derive(Default)]\n"));
        out.push_str(&format!("pub struct {};\n", standard.struct_path()));
        out.push_str(&format!("impl<{}> RegCommsAccessProc<{}, {}, {}> for {} {{\n", self.get_generics_string(), self.get_parameterized_typename(), self.address_word_size(), self.address_word_name(), standard.struct_path()));
        out.push_str(&format!("    fn proc_read(&self, peripheral: &mut {}, reg_address: {}, buf: &mut [u8]) -> Result<usize, RegCommsError> {{\n", self.get_parameterized_typename(), self.address_word_name()));
        out.push_str(&format!("        peripheral.comms.comms_read(reg_address, buf)\n"));
        out.push_str(&format!("    }}\n"));
        out.push_str(&format!("    async fn proc_read_async(&self, peripheral: &mut {}, reg_address: {}, buf: &mut [u8]) -> Result<usize, RegCommsError> {{\n", self.get_parameterized_typename(), self.address_word_name()));
        out.push_str(&format!("        peripheral.comms.comms_read_async(reg_address, buf).await\n"));
        out.push_str(&format!("    }}\n"));
        out.push_str(&format!("    fn proc_write(&self, peripheral: &mut {}, reg_address: {}, buf: &[u8]) -> Result<usize, RegCommsError> {{\n", self.get_parameterized_typename(), self.address_word_name()));
        out.push_str(&format!("        peripheral.comms.comms_write(reg_address, buf)\n"));
        out.push_str(&format!("    }}\n"));
        out.push_str(&format!("    async fn proc_write_async(&self, peripheral: &mut {}, reg_address: {}, buf: &[u8]) -> Result<usize, RegCommsError> {{\n", self.get_parameterized_typename(), self.address_word_name()));
        out.push_str(&format!("        peripheral.comms.comms_write_async(reg_address, buf).await\n"));
        out.push_str(&format!("    }}\n"));
        out.push_str(&format!("}}\n"));
        for proc in self.get_access_procs_map() {
            out.push_str(&format!("static {}: Once<{}> = Once::new();\n", proc.static_name(), proc.struct_path()));
        }
        out.push_str(&format!("pub struct {}<{}> {{\n", self.peripheral_struct_name(), self.get_generics_string()));
        for trait_member in self.get_trait_members_list() {
            out.push_str(&format!("    pub {}: {},\n", trait_member.member_name(), trait_member.generic()));
        }
        for proc in self.get_access_procs_map() {
            out.push_str(&format!("    pub {}: &'static {},\n", proc.member_name(), proc.struct_path()));
        }
        out.push_str(&format!("}}\n"));
        out.push_str(&format!("impl<{}> {}<{}> {{\n", self.get_generics_string(), self.peripheral_struct_name(), self.get_boundfree_generics()));
        out.push_str(&format!("    pub fn new({}) -> Self {{\n", self.get_constructor_args_list()));
        out.push_str(&format!("        Self {{\n"));
        for trait_member in self.get_trait_members_list() {
            out.push_str(&format!("             {},\n", trait_member.member_name()));
        }
        for proc in self.get_access_procs_map() {
            out.push_str(&format!("            {}: {}.call_once(|| Default::default()),\n", proc.member_name(), proc.static_name()));
        }
        out.push_str(&format!("        }}\n"));
        out.push_str(&format!("    }}\n"));
        for reg in self.registers.iter() {
            out.push_str(&format!("    pub fn {}<'a>(&'a mut self) -> {}::{}<'a, {}> {{\n", reg.reg_method_name(), reg.reg_mod_name(), reg.reg_struct_name(), self.get_boundfree_generics()));
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
