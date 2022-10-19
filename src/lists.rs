use std::collections::HashMap;
use std::hash::Hash;

use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use rbs::to_value;

/// group slice分组
pub fn group<K, V>(list: &Vec<V>, kf: fn(t: &V) -> K) -> HashMap<K, Vec<V>>
where
    K: Eq + Hash,
    V: Clone,
{
    let mut ret: HashMap<K, Vec<V>> = HashMap::new();
    for v in list {
        let k = kf(v);
        if ret.contains_key(&k) {
            if let Some(l) = ret.get_mut(&k) {
                l.push(v.clone());
            }
        } else {
            ret.insert(k, vec![v.clone()]);
        }
    }
    ret
}

/// group slice分组
pub fn to_map<K, V, T>(list: &Vec<T>, kf: fn(t: &T) -> (K, V)) -> HashMap<K, V>
where
    K: Eq + Hash,
{
    let mut ret: HashMap<K, V> = HashMap::new();
    for t in list {
        let (k, v) = kf(t);
        ret.insert(k, v);
    }
    ret
}

#[test]
fn testc() {
   
}
