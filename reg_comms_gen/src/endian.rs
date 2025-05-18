use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Endian {
    Big,
    Little,
}

impl Endian {
    pub fn abbrev(self) -> &'static str {
        match self {
            Self::Big => "be",
            Self::Little => "le",
        }
    }
}
