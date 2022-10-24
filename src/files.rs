use anyhow::Result;
use std::{
    fs::{self, read_dir, File, OpenOptions},
    io::SeekFrom,
    io::{BufRead, BufReader, Read, Seek},
    path::Path,
    str::FromStr,
};

/// 创建文件,文件存在会打开,往文件追加内容
pub fn open_file(file_path: &str) -> File {
    create_pdir(file_path);
    OpenOptions::new()
        .read(true) // 可读
        .write(true) // 可写
        .append(true) // 追加内容
        .create(true) // 新建，若文件存在则打开这个文件
        .open(file_path)
        .unwrap()
}

/// 创建父文件夹
pub fn create_pdir(file_path: &str) {
    //创建父目录
    let path = Path::new(file_path);
    let prefix = path.parent().unwrap();
    if !prefix.exists() {
        std::fs::create_dir_all(prefix).unwrap();
    }
}
/// 创建新文件,文件存在会清空内容
pub fn create_file(file_path: &str) -> Result<File, std::io::Error> {
    //创建父目录
    create_pdir(file_path);
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

pub fn read_line<T>(path: &str, deal_line: fn(line: String) -> Option<T>) -> Vec<T> {
    let mut vec = Vec::new();
    match File::open(path) {
        Ok(input) => {
            let buffered = BufReader::new(input);
            for line in buffered.lines() {
                if let Ok(s) = line {
                    let value = deal_line(s);
                    match value {
                        Some(v) => {
                            vec.push(v);
                        }
                        None => {}
                    }
                }
            }
        }
        Err(err) => {
            println!("{err}");
        }
    }
    vec
}

/// 文件大小
pub fn file_size(path: &str) -> u64 {
    let meta = fs::symlink_metadata(path).unwrap();
    meta.len()
}

/// 读取文件最后一行
pub fn read_last_line(path: &str, buf_size: u64) -> Option<String> {
    let file_size = file_size(path);
    match File::open(path) {
        Ok(mut input) => {
            if file_size > buf_size {
                let start_idx = file_size - buf_size;
                input.seek(SeekFrom::Start(start_idx)).unwrap();
            }
            let bf = BufReader::new(input);
            match bf.lines().last() {
                Some(line) => match line {
                    Ok(l) => {
                        return Some(l);
                    }
                    Err(err) => {
                        println!("seek 失败 {err}");
                        None
                    }
                },
                None => None,
            }
        }
        Err(err) => {
            eprintln!("打开文件{path}失败,{err}");
            None
        }
    }
}

/// 获取文件行数
pub fn line_size(path: &str) -> usize {
    if let Ok(f) = File::open(path) {
        let file_size = file_size(path);
        println!("size={file_size}");
        if file_size > 0 {
            let buffered = BufReader::new(f);
            return buffered.lines().count();
        }
    }
    0
}

// 加载目录文件
pub fn load<T>(path: &str, filter: fn(path: &String) -> bool) -> Vec<T>
where
    T: FromStr + Default,
{
    let files = list_all_file(path, filter);
    let mut all = Vec::new();
    for ele in files {
        let mut items = load_file(&ele);
        all.append(&mut items);
    }
    all
}

// 加载文件
pub fn load_file<T>(path: &str) -> Vec<T>
where
    T: FromStr + Default,
{
    read_line(path, |s| match T::from_str(&s) {
        Ok(t) => Some(t),
        Err(_) => {
            println!("line {s} pase err");
            None
        }
    })
}

#[test]
fn test_size() {
    println!(
        "{}",
        line_size("E:/data/datacenter/test/cps_member_device/wx28.data")
    )
}

// 按行加载文件
pub fn load_file_by_line<T>(path: &str, t: &mut T, line_fn: fn(line: String, t: &mut T)) {
    match File::open(path) {
        Ok(input) => {
            let buffered = BufReader::new(input);
            for line in buffered.lines() {
                if let Ok(line) = line {
                    line_fn(line, t);
                }
            }
        }
        Err(err) => {
            println!("{err}");
        }
    }
}

// 加载目录文件
pub fn load_by_line<T>(
    //加载目录
    path: &str,
    //过滤器
    filter: fn(path: &String) -> bool,
    //数据
    t: &mut T,
    //处理行数据方法
    line_fn: fn(line: String, t: &mut T),
) {
    let files = list_all_file(path, filter);
    for ele in files {
        load_file_by_line(&ele, t, line_fn);
    }
}
