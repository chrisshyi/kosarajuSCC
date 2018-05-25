extern crate kosarajuSCC;
use std::fs::File;

use kosarajuSCC::make_reverse_graph;

fn main() {
    println!("Hello, world!");
    let file = File::open("/home/chris/Workspace/rust/kosarajuSCC/SCC.txt").unwrap();
    make_reverse_graph(file);
    
}
