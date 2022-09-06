use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Field {
    pub name: String,
    pub comment: String,
    pub is_pk: bool,
    pub is_idx: bool,
    pub db_type: String,
    pub size: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TableInfo {
    pub name: String,
    pub comment: String,
    pub idxs: Vec<Field>,
    pub pks: Vec<Field>,
    pub fields: Vec<Field>,
}

impl TableInfo {
    pub fn add_field(&mut self, f: Field) {
        self.fields.push(f.clone());
        if f.is_pk {
            self.pks.push(f.clone());
        }
        if f.is_idx {
            self.idxs.push(f);
        }
    }
}
