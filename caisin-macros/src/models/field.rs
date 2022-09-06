use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Field {
    pub name: String,
    pub def_value: String,
    pub comment: String,
    pub auto_inc: bool,
    pub null_able: bool,
    pub is_pk: bool,
    pub is_idx: bool,
    pub db_type: String,
    pub size: i32,
}

impl Field {
    pub fn to_sql(&self) -> String {
        let mut s = String::from("\t");
        s.push_str(&self.name);
        s.push_str(" ");
        let db_typ = match self.db_type.as_str() {
            "i32" => "int",
            "i64" => "bigint",
            "DateTime" => "datetime",
            _ => "varchar",
        };
        s.push_str(db_typ);
        let size = match db_typ {
            "varchar" => 255,
            _ => self.size,
        };
        if size > 0 {
            s.push_str("(");
            s.push_str(size.to_string().as_str());
            s.push_str(")");
        }
        if !self.null_able {
            s.push_str(" not null");
        }
        if self.auto_inc {
            s.push_str(" auto_increment");
        }
        if !self.def_value.is_empty() {
            s.push_str(" default ");
            s.push_str(&self.def_value);
        }
        if !self.comment.is_empty() {
            s.push_str(" comment '");
            s.push_str(&self.comment);
            s.push_str("'");
        }
        s
    }
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
    pub fn create_table_sql(&self) -> String {
        /* create table test
        (
            a int not null auto_increment default 0 comment '',
            primary key (a),
            index (a)
        ) comment 'ces' */
        let mut s = String::from("create table ");
        s.push_str(&self.name);
        s.push_str(" (\n");
        let mut fields = Vec::new();
        for ele in &self.fields {
            let sql = ele.to_sql();
            fields.push(sql);
        }
        //主键
        let sql = self.get_pk_str();
        fields.push(sql);
        //索引
        for ele in self.get_idx() {
            fields.push(ele);
        }
        let fs = fields.join(",\n").to_string();
        s.push_str(fs.as_str());
        s.push_str("\n)\n");
        if !self.comment.is_empty() {
            s.push_str("\tcomment '");
            s.push_str(&self.comment);
            s.push_str("';\n")
        }
        s
    }

    pub fn get_pk_str(&self) -> String {
        let mut s = String::new();
        if self.pks.len() > 0 {
            s.push_str("\tprimary key (");

            let mut pk = Vec::new();
            for ele in &self.pks {
                pk.push(ele.name.as_str());
            }
            s.push_str(pk.join(",").to_string().as_str());
            s.push_str(")");
        }
        s
    }
    pub fn get_idx(&self) -> Vec<String> {
        let mut pk = Vec::new();
        for ele in &self.pks {
            let mut s = String::new();
            s.push_str("\tindex(");
            s.push_str(&ele.name);
            s.push_str(")");
            pk.push(s);
        }
        pk
    }
}
