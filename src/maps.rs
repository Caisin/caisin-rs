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

pub fn group_by_value<K, V>(m: &HashMap<K, V>) -> HashMap<V, Vec<K>>
where
    V: Eq + Hash + Clone,
    K: Clone,
{
    let mut ret: HashMap<V, Vec<K>> = HashMap::new();
    for (k, v) in m {
        if ret.contains_key(v) {
            let keys = ret.get_mut(v).unwrap();
            keys.push(k.clone());
        } else {
            ret.insert(v.clone(), vec![k.clone()]);
        }
    }
    ret
}
