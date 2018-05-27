extern crate kosarajuSCC;
use std::fs::File;
use std::thread;
use kosarajuSCC::{compute_scc};

fn main() {

    let builder = thread::Builder::new().stack_size(100 * 1024 * 1024); // 100 MB of stack 

    let handle = builder.spawn(|| {

        let scc_map = compute_scc("/home/chris/Workspace/rust/kosarajuSCC/SCC.txt");

        let borrowed_map = scc_map.borrow();

        let mut scc_sizes = borrowed_map.values().collect::<Vec<&i32>>();

        scc_sizes.sort();
        scc_sizes.reverse();
        let mut size_iterator = scc_sizes.iter();

        println!("The five largest SCCs have sizes:");
        for _i in 0..5 {
            println!("{}", size_iterator.next().unwrap());
        }

    }).unwrap();

    handle.join().unwrap();
}
