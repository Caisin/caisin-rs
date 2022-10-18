use std::collections::HashMap;
use std::hash::Hash;

/// group slice分组
pub fn group<K, V>(list: Vec<V>, kf: fn(t: &V) -> K) -> HashMap<K, Vec<V>>
where
    K: Eq + Hash,
{
    let mut ret: HashMap<K, Vec<V>> = HashMap::new();
    for v in list {
        let k = kf(&v);
        if ret.contains_key(&k) {
            if let Some(l) = ret.get_mut(&k) {
                l.push(v);
            }
        } else {
            ret.insert(k, vec![v]);
        }
    }
    ret
}

/// group slice分组
pub fn to_map<K, V, T>(list: Vec<T>, kf: fn(t: &T) -> (K, V)) -> HashMap<K, V>
where
    K: Eq + Hash,
{
    let mut ret: HashMap<K, V> = HashMap::new();
    for t in list {
        let (k, v) = kf(&t);
        ret.insert(k, v);
    }
    ret
}
