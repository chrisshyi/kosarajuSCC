extern crate kosarajuSCC;
use std::fs::File;

use kosarajuSCC::{make_reverse_graph, dfs_outer, relabel_graph};

fn main() {
    println!("Hello, world!");
    let mut file = File::open("/home/chris/Workspace/rust/kosarajuSCC/SCC.txt").unwrap();
    let (reverse_graph, num_vertices) = make_reverse_graph(file);

    let finishing_times = dfs_outer(reverse_graph, true, num_vertices);

    file = File::open("/home/chris/Workspace/rust/kosarajuSCC/SCC.txt").unwrap();
    let relabelled_graph = relabel_graph(file, finishing_times);

    let scc_map = dfs_outer(relabelled_graph, false, num_vertices);

    for key in scc_map.keys() {
        println!("SCC {} has size {}", key, scc_map.get(key).unwrap());
    }
}
