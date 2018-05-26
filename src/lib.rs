use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{BufReader, BufRead};

/// Creates the adjacency list representation of a directed acyclic graph, but
/// with all the edges reversed
/// 
/// Assumes the data file has the format:
/// [tail vertex] [head vertex]
/// 
/// example:
/// 2 5 indicates there an edge from vertex 2 to vertex 5
pub fn make_reverse_graph(file: File) -> HashMap<i32, HashSet<i32>> {   
    let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
    let reader = BufReader::new(file);
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
pub fn dfs_outer(graph: HashMap<i32, HashSet<i32>>, first_pass: bool, num_vertices: i32) -> HashMap<i32, i32> {
    let mut explored_nodes: HashSet<i32> = HashSet::new();
    let mut result_map: HashMap<i32, i32> = HashMap::new();

    let mut finishing_time = Box::new(1);
    let mut leader_val = Box::new(0);
    for vertex in (1..num_vertices + 1).rev() {
        if !explored_nodes.contains(&vertex) {
            if first_pass {
                dfs_inner(&graph, vertex, first_pass, &mut finishing_time, &mut result_map, &mut explored_nodes);
            } else {
                *leader_val = vertex;
                dfs_inner(&graph, vertex, first_pass, &mut leader_val, &mut result_map, &mut explored_nodes);
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
pub fn dfs_inner(graph: &HashMap<i32, HashSet<i32>>, start_vertex: i32, first_pass: bool, 
leader_or_ftime: &mut Box<i32>, leader_or_ft_map: &mut HashMap<i32, i32>, explored_nodes: &mut HashSet<i32>) {
    let mut stack: Vec<i32> = Vec::new();

    stack.push(start_vertex);

    while !stack.is_empty() {
        let cur_vertx = stack.pop().unwrap();
        explored_nodes.insert(cur_vertx);

        for neighbor in graph.get(&cur_vertx).unwrap().iter() {
            if !explored_nodes.contains(neighbor) {
                stack.push(*neighbor);
            }
        }
        if first_pass {
            **leader_or_ftime += 1;
            leader_or_ft_map.insert(cur_vertx, **leader_or_ftime);
        } else {
            let scc_size = leader_or_ft_map.entry(**leader_or_ftime).or_insert(0);
            *scc_size += 1;
        }
    }                        
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
