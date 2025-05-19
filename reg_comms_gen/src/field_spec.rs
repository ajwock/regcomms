use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FieldSpec {
    pub name: String,
    pub field_pos: FieldPos,
    // readable
    // writable
    // aliasable
}

#[derive(Copy, Clone, Debug)]
pub enum FieldPos {
    Bit(u8),
    Field(u8, u8),
}

impl FieldPos {
    pub fn fieldpos_word(self) -> &'static str {
        match self {
            FieldPos::Bit(_) => "u8",
            FieldPos::Field(high, low) => {
                assert!(high >= low);
                let field_len = high - low + 1;
                if field_len <= 8 {
                    "u8"
                } else if field_len <= 16 {
                    "u16"
                } else if field_len <= 32 {
                    "u32"
                } else if field_len <= 64 {
                    "u64"
                } else {
                    panic!("Unsupported field len longer than 64: {field_len}, based on {high}:{low}")
                }
            }
        }
    }
}

impl<'de> Deserialize<'de> for FieldPos {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FieldPosVisitor)
    }
}

struct FieldPosVisitor;

impl<'de> Visitor<'de> for FieldPosVisitor {
    type Value = FieldPos;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a bit index like \"4\" or a range like \"[4:6]\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<FieldPos, E>
    where
        E: de::Error,
    {
        if let Ok(single) = v.parse::<u8>() {
            return Ok(FieldPos::Bit(single));
        }

        if let Some(stripped) = v.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            let parts: Vec<&str> = stripped.split(':').collect();
            if parts.len() == 2 {
                let from = parts[0].parse::<u8>().map_err(de::Error::custom)?;
                let to = parts[1].parse::<u8>().map_err(de::Error::custom)?;
                if to > from {
                    return Err(de::Error::custom(format!("Bitfield spec in [from:to], 'from' must be greater than 'to', got [{}:{}]", from, to)))
                }
                return Ok(FieldPos::Field(from, to))
            }
        }

        Err(de::Error::custom(format!("invalid FieldPos format: {}", v)))
    }
}

impl Serialize for FieldPos {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            FieldPos::Bit(bit) => serializer.serialize_str(&format!("{}", bit)),
            FieldPos::Field(from, to) => serializer.serialize_str(&format!("[{}:{}]", from, to)),
        }
    }
}

impl FieldSpec {
    pub fn method_name(&self) -> String {
        stringcase::snake_case(&self.name)
    }

    pub fn struct_name(&self) -> String {
        stringcase::pascal_case(&self.name)
    }
}
