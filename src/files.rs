use std::{
    fs::{read_dir, File, OpenOptions},
    path::Path,
};

/// 创建文件,文件存在会打开,往文件追加内容
pub fn open_file(file_path: &str) -> File {
    OpenOptions::new()
        .read(true) // 可读
        .write(true) // 可写
        .append(true) // 追加内容
        .create(true) // 新建，若文件存在则打开这个文件
        .open(file_path)
        .unwrap()
}

/// 创建新文件,文件存在会清空内容
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

/// 列出目录下所有文件
pub fn list_all_file(dir_path: &str, filter: fn(path: &String) -> bool) -> Vec<String> {
    let mut ret = Vec::new();
    if exists(dir_path) {
        for ele in read_dir(dir_path).unwrap() {
            let ele = ele.unwrap();
            let metadata = ele.metadata().unwrap();
            let path = ele.path().display().to_string();
            let path = path.replace("\\", "/");
            if metadata.is_dir() {
                let mut files = list_all_file(path.as_str(), filter);
                ret.append(&mut files);
            }
            if metadata.is_file() {
                if filter(&path) {
                    ret.push(path)
                }
            }
        }
    }
    ret
}
