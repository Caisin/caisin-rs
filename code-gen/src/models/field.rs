use caisin::get_rs_type_by_db;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl Filed {
    pub fn get_rs_type_str(&self) -> String {
        let rs_type = get_rs_type_by_db(&self.get_db_type());

        if let "YES" = self.null.as_str() {
            let mut s = String::from("Option<");
            s.push_str(rs_type);
            s.push_str(">");
            s
        } else {
            rs_type.to_owned()
        }
    }
    pub fn get_db_type(&self) -> String {
        let mut db_type = self.typ.clone();
        if self.typ.contains("(") {
            match self.typ.find("(") {
                Some(idx) => {
                    db_type = self.typ[0..idx].to_string();
                }
                None => {}
            }
        }
        db_type
    }
    pub fn get_db_size(&self) -> String {
        if self.typ.contains("(") {
            if let Some(st) = self.typ.find("(") {
                if let Some(ed) = self.typ.find(")") {
                    return self.typ[(st + 1)..(ed - 1)].to_owned();
                }
            }
        }
        "".to_string()
    }
}
