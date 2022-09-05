use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Filed {
    #[serde(rename = "Field")]
    pub field: String,
    #[serde(rename = "Type")]
    pub typ: String,
    #[serde(rename = "Collation")]
    pub collation: Option<String>,
    #[serde(rename = "Null")]
    pub null: String,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Default")]
    pub default: Option<String>,
    #[serde(rename = "Extra")]
    pub extra: String,
    #[serde(rename = "Privileges")]
    pub privileges: String,
    #[serde(rename = "Comment")]
    pub comment: String,
}