use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{Result, BufReader, BufRead};

pub fn make_reverse_graph(file: File) -> HashMap<i32, HashSet<i32>> {   
    let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut reader = BufReader::new(file);
    let (mut tail_vertx, mut head_vertx) = (0, 0); // Compiler won't allow use of uninitialized variables
    
    for line in reader.lines() {
        let split_line: Vec<String> = line.unwrap().split_whitespace().map(|s| s.to_string()).collect();
        tail_vertx = split_line[0].parse::<i32>().unwrap();
        head_vertx = split_line[1].parse::<i32>().unwrap();

        println!("{} has an edge to {}", tail_vertx, head_vertx);
        
        if !map.contains_key(&head_vertx) {
            map.insert(head_vertx, HashSet::new());
        }
        map.get_mut(&head_vertx).unwrap().insert(tail_vertx);
    }
    map 
} 


















#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_graph_construction() {
        let file = File::open("/home/chris/Workspace/rust/kosarajuSCC/test_files/test1.txt").unwrap();
        let reverse_graph = make_reverse_graph(file);
        assert!(reverse_graph.get(&4).unwrap().contains(&2));
        assert!(reverse_graph.get(&2).unwrap().contains(&1));
        assert!(reverse_graph.get(&1).unwrap().contains(&3));
        assert!(reverse_graph.get(&3).unwrap().contains(&2));
    }
}
