use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TraitMember {
    pub name: String,
    pub generic_type: String,
    pub trait_bound: String,
}

impl TraitMember {
    pub fn member_name(&self) -> String {
        stringcase::snake_case(&self.name)
    }

    pub fn generic(&self) -> String {
        stringcase::pascal_case(&self.name)
    }

    pub fn bound(&self) -> &str {
        &self.trait_bound
    }
}
