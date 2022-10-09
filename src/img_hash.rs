use std::num::ParseIntError;



pub fn get_hash_from_str(hash_str: &str) -> Result<u64, ParseIntError> {
    let mut str = hash_str;
    if hash_str.contains(":") {
        str = &hash_str[2..hash_str.len()];
    }
    let from_str_radix = u64::from_str_radix(str, 16);
    println!("{:?}", from_str_radix);
    from_str_radix
}

fn popcnt(mut x: u64) -> u64 {
    let mut diff = 0;
    while x != 0 {
        diff += x & 1;
        x >>= 1;
    }
    diff
}

pub fn distance_from_str(lstr: &str, rstr: &str) -> u64 {
    let l = get_hash_from_str(lstr);
    let r = get_hash_from_str(rstr);
    distance(l.unwrap(), r.unwrap())
}

fn distance(l: u64, r: u64) -> u64 {
    let hamming = l ^ r;
    popcnt(hamming)
}
