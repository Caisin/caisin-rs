pub fn get_db_type_by_rs(rs_type: &str) -> &'static str {
    match rs_type {
        "bool" => "bool",
        "i8" => "tinyint",
        "i16" => "smallint",
        "i32" => "int",
        "i64" => "bigint",
        "DateTime" => "datetime",
        "FastDateTime" => "datetime",
        _ => "varchar",
    }
}

pub fn get_rs_type_by_db(db_type: &str) -> &'static str {
    match db_type {
        "float" => "f64",
        "bool" => "bool",
        "tinyint" => "i8",
        "smallint" => "i16",
        "int" => "i32",
        "bigint" => "i64",
        "datetime" => "FastDateTime",
        _ => "String",
    }
}
