use std::{fs::File, path::Path};
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
    if !Path::new(dir_path).exists() {
        std::fs::create_dir_all(dir_path).unwrap();
    }
}
