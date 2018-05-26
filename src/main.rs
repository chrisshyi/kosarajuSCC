extern crate kosarajuSCC;
use std::fs::File;

use kosarajuSCC::{compute_scc};

fn main() {
    let scc_map = compute_scc("/home/chris/Workspace/rust/kosarajuSCC/SCC.txt");

    let borrowed_map = scc_map.borrow();

    for key in borrowed_map.keys() {
        println!("SCC {} has size {}", key, borrowed_map.get(key).unwrap());
    }
}
