extern crate kosarajuSCC;
use std::fs::File;

use kosarajuSCC::make_reverse_graph;

fn main() {
    println!("Hello, world!");
    let file = File::open("../SCC.txt").unwrap();
    make_reverse_graph(file);
    
}
