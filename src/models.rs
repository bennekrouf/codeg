use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

// Define an enum for the properties with strict validation
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Property {
    Mandatory,
    Number,
    String,
    Optional,
}

// Implement a custom deserializer for the Property enum
impl<'de> Deserialize<'de> for Property {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize the input to a string first
        let s: String = Deserialize::deserialize(deserializer)?;
        // Convert the input to lowercase and match it with enum variants
        match s.to_lowercase().as_str() {
            "mandatory" => Ok(Property::Mandatory),
            "number" => Ok(Property::Number),
            "string" => Ok(Property::String),
            "optional" => Ok(Property::Optional),
            _ => Err(serde::de::Error::unknown_variant(
                &s,
                &["mandatory", "number", "string", "optional"],
            )),
        }
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Define the data structures
#[derive(Debug, Deserialize, Serialize)]
pub struct Field {
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Endpoint {
    pub path: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entity {
    pub endpoints: Vec<Endpoint>,
    pub fields: Option<Vec<Field>>,
}
