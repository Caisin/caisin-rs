use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
/// map 转换
pub fn trans_map<K, V, T>(m: &HashMap<K, V>, vf: fn(v: &V) -> T) -> HashMap<K, T>
where
    K: Eq + Hash + Clone,
{
    let mut ret: HashMap<K, T> = HashMap::new();
    for (k, v) in m {
        ret.insert(k.clone(), vf(v));
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

pub fn group_by_key<K, V, T>(m: &HashMap<K, V>, kf: fn(k: &K) -> T) -> HashMap<T, HashMap<K, V>>
where
    T: Eq + Hash + Clone,
    K: Eq + Hash + Clone,
    V: Clone,
{
    let mut ret: HashMap<T, HashMap<K, V>> = HashMap::new();
    for (k, v) in m {
        let t = kf(k);
        if ret.contains_key(&t) {
            let keys = ret.get_mut(&t).unwrap();
            keys.insert(k.clone(), v.clone());
        } else {
            let mut rv = HashMap::new();
            rv.insert(k.clone(), v.clone());
            ret.insert(t, rv);
        }
    }
    ret
}

pub fn arc_map<K, V>() -> Arc<Mutex<HashMap<K, V>>> {
    Arc::new(Mutex::new(HashMap::new()))
}
