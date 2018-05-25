use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{Result, BufReader, BufRead};

pub fn make_reverse_graph(file: File) -> HashMap<i32, HashSet<i32>> {
    let mut reader = BufReader::new(file);
    
    for line in reader.lines() {
        let split_line: Vec<String> = line.unwrap().split_whitespace().map(|s| s.to_string()).collect();
        println!("{:?}", split_line);
    }
    let map: HashMap<i32, HashSet<i32>> = HashMap::new();
    map 
} 
