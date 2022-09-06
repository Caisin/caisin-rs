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
    pub fn create_table_sql(&self) -> &str {
        /* create table test
        (
            a int not null auto_increment default 0 comment '',
            primary key (a),
            index (a)
        ) comment 'ces' */
        let mut s = String::from("create table ");
        s.push_str(&self.name);
        s.push_str(" (\n");
        for ele in &self.fields {
            
        }

        ""
    }
    pub fn get_pk_str(&self) -> &str {
        let s = String::new();
        for ele in &self.pks {}
        ""
    }
}
