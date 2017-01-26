extern crate rayon;
extern crate fnv;
extern crate concurrent_hashmap;

use std::io::*;
use std::collections::*;
use rayon::prelude::*;
use concurrent_hashmap::*;
use std::sync::*;
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;

const ENCODING_TABLE: [u8; 117] =
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3];

const DECODING_TABLE: [u8; 4] = [65, 67, 71, 84];
const TASK_SIZE: usize = 5000000;

fn read_input() -> Vec<u8> {
    let stdin = stdin();
    let mut handle = stdin.lock();
    let mut data = Vec::new();
    while let Some(len) = handle.read_until(b'\n', &mut data).ok() {
        if len > 2 && data.starts_with(">TH".as_bytes()) {
            data.clear();
            break;
        }
        data.clear();
    }
    let data = handle.split(b'\n').fold(Vec::new(), |mut vec, line| {
        vec.extend_from_slice(line.unwrap().as_slice());
        vec
    });
    data
}

fn encode(slice: &[u8]) -> u64 {
    slice.iter().fold(0, |acc, &x| (acc << 2) + ENCODING_TABLE[x as usize] as u64)
}

fn decode(mut code: u64, len: usize) -> String {
    let mut result = Vec::new();
    for _ in 0..len {
        result.push(DECODING_TABLE[(code & 0x03) as usize]);
        code >>= 2;
    }
    result.reverse();
    String::from_utf8(result).unwrap()
}

fn update_hashmap(map: &ConcHashMap<u64, i32, BuildHasherDefault<FnvHasher>>, key: u64) {
    map.upsert(key, 1, &|x| *x += 1);
}

fn count(len: usize, data: &[u8], map: Arc<ConcHashMap<u64, i32, BuildHasherDefault<FnvHasher>>>) {
    for i in 0..(data.len() - len + 1) {
        update_hashmap(&map, encode(&data[i..(i + len)]));
    }
}

fn stat_all(len: usize, map: &ConcHashMap<u64, i32, BuildHasherDefault<FnvHasher>>) {
    let mut vec: Vec<(&u64, &i32)> = map.iter().collect();
    let sum: i32 = vec.iter().map(|&(_, &v)| v).sum();
    let sum = sum as f64;
    vec.sort_by(|&(&ka, &va), &(&kb, &vb)| {
        let cmp_val = vb.cmp(&va);
        if cmp_val == std::cmp::Ordering::Equal {
            ka.cmp(&kb)
        } else {
            cmp_val
        }
    });
    for (&k, &v) in vec {
        println!("{} {:.3}", decode(k, len), 100.0 * v as f64 / sum);
    }
    println!("");
}

fn main() {
    let data = read_input();
    let num_list: Vec<usize> = vec![1, 2, 3, 4, 6, 12, 18];
    let maps: HashMap<usize, Arc<ConcHashMap<u64, i32, BuildHasherDefault<FnvHasher>>>> = num_list.iter()
        .map(|&len| {
            (len,Arc::new(ConcHashMap::<u64, i32, BuildHasherDefault<FnvHasher>>::with_options(Options::default())))
        })
        .collect();
    let len = data.len();
    let mut tasks = Vec::new();
    for &i in &num_list {
        let mut head = 0;
        while head < len - i + 1 {
            let tail = std::cmp::min(head + TASK_SIZE, len - i + 1);
            tasks.push((i, head, tail));
            head = tail;
        }
    }
    tasks.par_iter()
        .for_each(|&(len, head, tail)| {
            count(len, &data[head..tail], maps.get(&len).unwrap().clone())
        });

    stat_all(num_list[0], maps.get(&1).unwrap());
    stat_all(num_list[1], maps.get(&2).unwrap());

    for pattern in ["GGT", "GGTA", "GGTATT", "GGTATTTTAATT", "GGTATTTTAATTTATAGT"].iter() {
        let map = maps.get(&pattern.len()).unwrap();
        println!("{}\t{}",
                 match map.find(&encode(pattern.as_bytes())) {
                     Some(val) => *val.get(),
                     None => 0,
                 },
                 pattern);
    }
}