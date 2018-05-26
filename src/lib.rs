use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{BufReader, BufRead};
use std::cell::RefCell;


pub fn compute_scc(file_path: &str) -> RefCell<HashMap<i32, i32>> {

    let mut file = File::open(file_path).unwrap();

    let (reverse_graph, num_vertices) = make_reverse_graph(file);

    let finishing_times = dfs_outer(reverse_graph, true, num_vertices);

    file = File::open(file_path).unwrap();
    let relabelled_graph = relabel_graph(file, &*finishing_times.borrow());

    let scc_map = dfs_outer(relabelled_graph, false, num_vertices);
    scc_map
}

/// Creates the adjacency list representation of a directed acyclic graph, but
/// with all the edges reversed
/// 
/// Assumes the data file has the format:
/// [tail vertex] [head vertex]
/// 
/// example:
/// 2 5 indicates there an edge from vertex 2 to vertex 5
fn make_reverse_graph(file: File) -> (HashMap<i32, HashSet<i32>>, i32) {   
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    let reader = BufReader::new(file);
    let (mut tail_vertx, mut head_vertx) = (0, 0); // Compiler won't allow use of uninitialized variables

    let mut largest_vertex = 1;    
    for line in reader.lines() {
        let split_line: Vec<String> = line.unwrap().split_whitespace().map(|s| s.to_string()).collect();
        tail_vertx = split_line[0].parse::<i32>().unwrap();
        head_vertx = split_line[1].parse::<i32>().unwrap();

        if tail_vertx > largest_vertex {
            largest_vertex = tail_vertx;
        }
        if head_vertx > largest_vertex {
            largest_vertex = head_vertx
        }
        // println!("{} has an edge to {}", tail_vertx, head_vertx);

        let neighbors = graph.entry(head_vertx).or_insert(HashSet::new());
        neighbors.insert(tail_vertx); 
    }
    (graph, largest_vertex) 
} 

/// Outer function for depth first search, keeps track of either
/// the leader variable or the finishing times and passes them to
/// the inner DFS call
/// 
/// Args:
///     graph: adjacency list of a graph (or reversed graph)
///     first_pass: true if this is the first pass DFS subroutine in Kosaraju's algorithm
///     num_vertices: the number of vertices in this graph
/// 
/// Returns:
///     A mapping of vertices to their finishing times (first pass), or a mapping of leader variables 
///     to their SCC sizes
fn dfs_outer(graph: HashMap<i32, HashSet<i32>>, first_pass: bool, num_vertices: i32) -> RefCell<HashMap<i32, i32>> {
    let mut explored_nodes: RefCell<HashSet<i32>> = RefCell::new(HashSet::new());
    let mut result_map: RefCell<HashMap<i32, i32>> = RefCell::new(HashMap::new());

    let finishing_time = RefCell::new(0);
    let leader_val = RefCell::new(0);
    for vertex in (1..num_vertices + 1).rev() {
        if first_pass {
            println!("First pass, {} vertices left", vertex);
        } else {
            println!("Second pass, {} vertices left", vertex);
        }
        if !explored_nodes.borrow_mut().contains(&vertex) {
            if first_pass {
                dfs_inner(&graph, vertex, first_pass, &finishing_time, &result_map, &explored_nodes);
            } else {
                *leader_val.borrow_mut() = vertex;
                dfs_inner(&graph, vertex, first_pass, &leader_val, &result_map, &explored_nodes);
            }
        }  
    }
    result_map
} 

/// Inner function for depth first search. Responsible for the 
/// actual graph search. Computes the finishing time of 
/// each vertex or the leader variable on the second pass of DFS
/// in Kosaraju's SCC algorithm
/// 
/// Args:
///     graph: adjacency list representation of a directed acyclic graph
///     start_vertex: the starting vertex from which DFS will begin
///     first_pass: true if this is the first pass of DFS in Kosaraju's algorithm
///     leader_or_ftime: if first pass, this is the current finishing time, else, this is the value of the leader
///     leader_or_ft_map: if first pass, maps vertices to their finishing times, else, maps each leader to the size of their SCCs
///     explored_nodes: set of explored nodes
fn dfs_inner(graph: &HashMap<i32, HashSet<i32>>, start_vertex: i32, first_pass: bool, 
leader_or_ftime: &RefCell<i32>, leader_or_ft_map: &RefCell<HashMap<i32, i32>>, explored_nodes: &RefCell<HashSet<i32>>) {

    // introduce a block here so that the mutable borrow will be dropped at the end of the block
    {
        explored_nodes.borrow_mut().insert(start_vertex);
    }

    if graph.contains_key(&start_vertex) {
        for neighbor in graph.get(&start_vertex).unwrap().iter() {
            if !explored_nodes.borrow().contains(neighbor) {
                dfs_inner(graph, *neighbor, first_pass, leader_or_ftime, leader_or_ft_map, explored_nodes);
            }
        }
    } else {
        return;
    }

    if first_pass {
        *leader_or_ftime.borrow_mut() += 1;
        // println!("Finishing time for {} is {}", start_vertex, *leader_or_ftime.borrow());
        leader_or_ft_map.borrow_mut().insert(start_vertex, *leader_or_ftime.borrow());
    } else {
        let mut borrowed_map = (*leader_or_ft_map).borrow_mut();
        let scc_size = borrowed_map.entry(*leader_or_ftime.borrow()).or_insert(0);
        *scc_size += 1;
    }
}

/// Relabels the vertices of a graph using their finishing times computed by 
/// the first pass DFS of Kosaraju's algorithm
fn relabel_graph(file: File, finishing_times: &HashMap<i32, i32>) -> HashMap<i32, HashSet<i32>> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    let reader = BufReader::new(file);
    let (mut tail_vertx, mut head_vertx) = (0, 0); // Compiler won't allow use of uninitialized variables

    for line in reader.lines() {
        let split_line: Vec<String> = line.unwrap().split_whitespace().map(|s| s.to_string()).collect();
        tail_vertx = split_line[0].parse::<i32>().unwrap();
        head_vertx = split_line[1].parse::<i32>().unwrap();

        let relabelled_tail = finishing_times.get(&tail_vertx).unwrap();
        let relabelled_head = finishing_times.get(&head_vertx).unwrap();
        
        let neighbors = graph.entry(*relabelled_tail).or_insert(HashSet::new());
        neighbors.insert(*relabelled_head);
    } 
    graph
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_graph_construction() {
        let file = File::open("/home/chris/Workspace/rust/kosarajuSCC/test_files/test1.txt").unwrap();
        let (reverse_graph, largest_vertex) = make_reverse_graph(file);
        assert!(reverse_graph.get(&4).unwrap().contains(&2));
        assert!(reverse_graph.get(&2).unwrap().contains(&1));
        assert!(reverse_graph.get(&1).unwrap().contains(&3));
        assert!(reverse_graph.get(&3).unwrap().contains(&2));
    }

    #[test]
    fn test_input_file2() {
        let scc_map = compute_scc("/home/chris/Workspace/rust/kosarajuSCC/test_files/test2.txt");
        assert!(!scc_map.borrow().is_empty());

        assert_eq!(*(scc_map.borrow().get(&6).unwrap()), 3);
        assert_eq!(*(scc_map.borrow().get(&7).unwrap()), 1);
        assert_eq!(*(scc_map.borrow().get(&4).unwrap()), 3);
    }


    #[test]
    fn test_input_file3() {
        let scc_map = compute_scc("/home/chris/Workspace/rust/kosarajuSCC/test_files/test3.txt");
        assert!(!scc_map.borrow().is_empty());

        assert_eq!(*(scc_map.borrow().get(&6).unwrap()), 4);
        assert_eq!(*(scc_map.borrow().get(&2).unwrap()), 1);
        assert_eq!(*(scc_map.borrow().get(&1).unwrap()), 1);
    }
}
