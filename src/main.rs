// Modified from Mrjillhace's version

extern crate rayon;
extern crate fnv;
extern crate concurrent_hashmap;

use std::io::*;
use std::collections::*;
use std::sync::*;
use std::hash::BuildHasherDefault;
use rayon::prelude::*;
use concurrent_hashmap::*;
use fnv::FnvHasher;

const ENCODING_TABLE: [u8; 117] =
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3];
const TASK_SIZE: usize = 5000000;
const SEQ_LENS: [usize; 7] = [1, 2, 3, 4, 6, 12, 18];
const LOOKUPS: [&'static str; 5] = ["GGT", "GGTA", "GGTATT", "GGTATTTTAATT", "GGTATTTTAATTTATAGT"];

type Table = ConcHashMap<u64, i32, BuildHasherDefault<FnvHasher>>;

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
    data.iter().map(|&byte| ENCODING_TABLE[byte as usize]).collect()
}

fn encode_str(s: &str) -> u64 {
    s.as_bytes().iter().fold(0, |acc, &x| (acc << 2) + ENCODING_TABLE[x as usize] as u64)
}

fn decode(mut v: u64, len: usize) -> String {
    let mut s = String::new();
    for _ in 0..len {
        let digit = v % 4;
        match digit {
            0 => s.push('A'),
            1 => s.push('C'),
            2 => s.push('G'),
            3 => s.push('T'),
            _ => {}
        };
        v /= 4;
    }
    s.chars().rev().collect()
}

struct Buffer {
    value: u64,
    size: usize,
}

impl Buffer {
    fn push(&mut self, c: u8) {
        self.value = (self.value * (1 << 2)) % (1 << (2 * self.size)) + (c as u64);
    }
}

fn parse(mut input: &[u8], len: usize, table: Arc<Table>) {
    let mut buffer = Buffer {
        value: 0,
        size: len,
    };
    if input.len() < len {
        return;
    }
    for _ in 0..len - 1 {
        buffer.push(input[0]);
        input = &input[1..];
    }
    while input.len() != 0 {
        buffer.push(input[0]);
        input = &input[1..];
        table.upsert(buffer.value, 1, &|x| *x += 1);
    }
}

fn report(table: &Table, len: usize) {
    let mut vec = Vec::new();

    for entry in table.iter() {
        vec.push((decode(*entry.0, len), *entry.1));
    }
    vec.sort_by(|a, b| b.1.cmp(&a.1));
    let sum = vec.iter().fold(0, |acc, i| acc + i.1);
    for seq in vec {
        println!("{} {:.3}", seq.0, (seq.1 * 100) as f32 / sum as f32);
    }
    println!("");
}

fn main() {
    let data = read_input();
    let tables: HashMap<usize, Arc<Table>> = SEQ_LENS.iter()
        .map(|&len| {
            (len,Arc::new(ConcHashMap::<u64, i32, BuildHasherDefault<FnvHasher>>::with_options(Options::default())))
        })
        .collect();
    let len = data.len();
    let mut tasks = Vec::new();
    for &i in &SEQ_LENS {
        let mut head = 0;
        while head < len - i + 1 {
            let tail = std::cmp::min(head + TASK_SIZE, len);
            tasks.push((i, head, tail));
            head = tail - i + 1;
        }
    }
    tasks.par_iter()
        .for_each(|&(len, head, tail)| {
            parse(&data[head..tail], len, tables.get(&len).unwrap().clone())
        });
    report(tables.get(&1).unwrap(), SEQ_LENS[0]);
    report(tables.get(&2).unwrap(), SEQ_LENS[1]);
    for pattern in LOOKUPS.iter() {
        let table = tables.get(&pattern.len()).unwrap();
        println!("{}\t{}",
                 match table.find(&encode_str(pattern)) {
                     Some(val) => *val.get(),
                     None => 0,
                 },
                 pattern);
    }
}