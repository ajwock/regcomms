use serde::{Serialize, Deserialize};
use crate::register_spec::RegisterSpec;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StructSpec {
    pub struct_name: String,
    // A struct is composed of registers which are at an offset within
    // the struct indicated by their address.
    pub fields: Vec<RegisterSpec>,
}
