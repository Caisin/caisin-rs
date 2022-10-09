use std::collections::HashMap;
use std::hash::Hash;
/// map 转换
pub fn trans_map<K, V, T>(m: HashMap<K, V>, vf: fn(v: &V) -> T) -> HashMap<K, T>
where
    K: Eq + Hash,
{
    let mut ret: HashMap<K, T> = HashMap::new();
    for (k, v) in m {
        ret.insert(k, vf(&v));
    }
    ret
}
