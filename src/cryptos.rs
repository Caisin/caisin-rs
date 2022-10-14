use crypto::{digest::Digest, md5::Md5};

pub fn md5(str: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(str);
    hasher.result_str()
}

