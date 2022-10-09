use std::{
    fs::{read_dir, File},
    path::Path,
};

use anyhow::{anyhow, Result};
/// 创建文件
pub fn create_file(file_path: &str) -> Result<File, std::io::Error> {
    //创建父目录
    let path = Path::new(file_path);
    let prefix = path.parent().unwrap();
    if !prefix.exists() {
        std::fs::create_dir_all(prefix).unwrap();
    }
    File::create(file_path)
}

/// 判断文件是否存在
pub fn exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// 创建文件夹
pub fn create_dir_all(dir_path: &str) {
    if !exists(dir_path) {
        std::fs::create_dir_all(dir_path).unwrap();
    }
}

/// 列出文件夹名称
pub fn list_dir_name(dir_path: &str) -> Vec<String> {
    let mut ret = vec![];
    if exists(dir_path) {
        for ele in read_dir(dir_path).unwrap() {
            let ele = ele.unwrap();
            if ele.metadata().unwrap().is_dir() {
                let file_name = format!("{:?}", ele.file_name());
                let len = &file_name.len();
                let var_name = file_name[1..len - 1].to_string();
                ret.push(var_name)
            }
        }
    }
    ret
}

/// 列出文件名称
pub fn list_file_name(dir_path: &str) -> Vec<String> {
    let mut ret = Vec::new();
    if exists(dir_path) {
        for ele in read_dir(dir_path).unwrap() {
            let ele = ele.unwrap();
            if ele.metadata().unwrap().is_file() {
                let file_name = format!("{:?}", ele.file_name());
                let var_name = file_name[1..file_name.len() - 1].to_string();
                ret.push(var_name)
            }
        }
    }
    ret
}
