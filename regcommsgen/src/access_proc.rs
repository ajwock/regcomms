use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessProcSpec {
    pub proc_name: String,
    pub struct_path: String,
}

impl AccessProcSpec {
    pub fn member_name(&self) -> String {
        stringcase::snake_case(&self.proc_name)
    }

    pub fn struct_path(&self) -> &str {
        &self.struct_path
    }

    pub fn static_name(&self) -> String {
        stringcase::macro_case(&self.proc_name)
    }
}
