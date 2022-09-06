use std::collections::HashSet;

use crate::models::Filed;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub comment: String,
    pub uses: Option<HashSet<String>>,
    pub fields: Option<Vec<Filed>>,
    pub fields_ident: Option<String>,
}

impl Table {
    pub fn pre_gen(&mut self) {
        if let Some(fields) = &self.fields {
            let mut s = String::new();
            for ele in fields {
                let mut attr = String::new();
                if !ele.comment.is_empty() {
                    attr.push_str("comment = \"");
                    attr.push_str(ele.comment.as_str());
                    attr.push_str("\"");
                }
                // 索引主键处理
                if !ele.key.is_empty() {
                    match ele.key.as_str() {
                        "PRI" => {
                            if !attr.is_empty() {
                                attr.push_str(", pk");
                            } else {
                                attr.push_str("pk");
                            }
                        }
                        "MUL" => {
                            if !attr.is_empty() {
                                attr.push_str(", index");
                            } else {
                                attr.push_str("index");
                            }
                        }
                        _ => {}
                    }
                }

                if !attr.is_empty() {
                    s.push_str("\t#[caisin(");
                    s.push_str(attr.as_str());
                    s.push_str(")]\n");
                }

                let mut f_name = ele.field.clone();
                if f_name == "type" {
                    f_name = format!("r#{}", f_name);
                }
                s.push_str("\tpub ");
                s.push_str(f_name.as_str());
                let rs_type = ele.get_rs_type_str();
                s.push_str(": ");
                s.push_str(rs_type.as_str());
                s.push_str(",\n")
            }
            self.fields_ident = Some(s);
        }
    }
}
