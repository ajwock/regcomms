use serde::{Serialize, Deserialize};
use crate::register_spec::RegisterSpec;
use crate::endian::Endian;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PeripheralSpec {
    pub name: String,
    pub byte_order: Endian,
    pub registers: Vec<RegisterSpec>,
}

impl PeripheralSpec {
    pub fn peripheral_struct_name(&self) -> String {
        stringcase::pascal_case(&self.name)
    }

    pub fn endian(&self) -> Endian {
        self.byte_order
    }
}
