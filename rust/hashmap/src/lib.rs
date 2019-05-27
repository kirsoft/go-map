#![feature(test)]
use std::collections::HashMap;

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
