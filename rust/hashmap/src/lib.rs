#![feature(test)]
use std::collections::HashMap;
use rustc_hash::FxHashMap;
use hashbrown::HashMap as HbHashMap;


pub mod bench;
pub mod test;


const N: usize = 10000;

fn crete_hashmap() -> HashMap<String, String> {
    HashMap::with_capacity(N)
}


fn write(m: &mut HashMap<String, String>) -> &mut HashMap<String, String> {
    for i in 0..N {
        let mut k = i.to_string();
        k.push_str("-string");
        m.insert(k.clone(), k);
    }

    m
}

fn read(m: &mut HashMap<String, String>) -> &mut HashMap<String, String> {
    for i in 0..N {
        let mut k = i.to_string();
        k.push_str("-string");
        m.get(&k);
    }

    m
}

fn fx_crete_hashmap() -> FxHashMap<String, String> {
    FxHashMap::default()
}


fn fx_write(m: &mut FxHashMap<String, String>) -> &mut FxHashMap<String, String> {
    for i in 0..N {
        let mut k = i.to_string();
        k.push_str("-string");
        m.insert(k.clone(), k);
    }

    m
}

fn fx_read(m: &mut FxHashMap<String, String>) -> &mut FxHashMap<String, String> {
    for i in 0..N {
        let mut k = i.to_string();
        k.push_str("-string");
        m.get(&k);
    }

    m
}

fn hashbrown_crete_hashmap() -> HbHashMap<String, String> {
    HbHashMap::default()
}


fn hashbrown_write(m: &mut HbHashMap<String, String>) -> &mut HbHashMap<String, String> {
    for i in 0..N {
        let mut k = i.to_string();
        k.push_str("-string");
        m.insert(k.clone(), k);
    }

    m
}

fn hashbrown_read(m: &mut HbHashMap<String, String>) -> &mut HbHashMap<String, String> {
    for i in 0..N {
        let mut k = i.to_string();
        k.push_str("-string");
        m.get(&k);
    }

    m
}