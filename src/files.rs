use anyhow::Result;
use std::{
    fs::{self, read_dir, File, OpenOptions},
    io::SeekFrom,
    io::{BufRead, BufReader, Seek, Write},
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
                ret.push(ele.file_name().to_str().unwrap().to_string())
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
                ret.push(ele.file_name().to_str().unwrap().to_string())
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
/// 根据指定行数分割文件
pub fn splite_file_by_lines(path: &str, out_path: &str, splite_lines: i32) -> Vec<String> {
    let (_, name, ext) = file_path_attr(path);
    let mut new_files = vec![];
    match File::open(path) {
        Ok(input) => {
            let buffered = BufReader::new(input);
            let mut lines = 0;
            let mut str = String::new();
            for line in buffered.lines() {
                if let Ok(s) = line {
                    str.push_str(&s);
                    str.push_str("\n");
                    lines += 1;
                    if lines > 0 && lines % splite_lines == 0 {
                        let new_file_name =
                            format!("{out_path}/{name}_{}_{lines}.{ext}", lines - splite_lines);
                        create_file(&new_file_name)
                            .unwrap()
                            .write_all(str.as_bytes())
                            .unwrap();
                        str.clear();
                        new_files.push(new_file_name);
                    }
                }
            }
            if !str.is_empty() {
                let new_file_name =
                    format!("{out_path}/{name}_{}_{lines}.{ext}", lines - splite_lines);
                create_file(&new_file_name)
                    .unwrap()
                    .write_all(str.as_bytes())
                    .unwrap();
                new_files.push(new_file_name);
            }
        }
        Err(err) => {
            println!("{err}");
        }
    }
    new_files
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
    if exists(path) {
        let meta = fs::symlink_metadata(path).unwrap();
        meta.len()
    } else {
        0
    }
}

/// 读取文件最后一行
pub fn read_last_line(path: &str, buf_size: u64) -> Option<String> {
    match File::open(path) {
        Ok(mut input) => {
            let file_size = file_size(path);
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
        Err(_) => {
            // eprintln!("打开文件{path}失败,{err}");
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
/// 获取文件目录,文件名,拓展名
pub fn file_path_attr(file_path: &str) -> (&str, &str, &str) {
    let path = Path::new(file_path);
    let dir = path.parent().unwrap().to_str().unwrap();
    let name = path.file_stem().unwrap().to_str().unwrap();
    let ext = path.extension().unwrap().to_str().unwrap();
    (dir, name, ext)
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
