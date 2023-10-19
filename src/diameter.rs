use crate::{GraphAdjList, Vertex};
use std::collections::{HashMap, VecDeque};

fn bfs(graph: &GraphAdjList, start: Vertex) -> HashMap<Vertex, usize> {
    let mut distances: HashMap<Vertex, usize> = HashMap::new();
    let mut queue = VecDeque::new();
    distances.insert(start, 0);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.adj_list.get(&current) {
            for &neighbor in neighbors {
                if !distances.contains_key(&neighbor) {
                    distances.insert(neighbor, *distances.get(&current).unwrap() + 1);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    distances
}

pub fn diameter(graph: &GraphAdjList) -> usize {
    let mut diameter = 0;
    for &vertex in graph.adj_list.keys() {
        let distances = bfs(&graph, vertex);
        if let Some(&max_distance) = distances.values().max() {
            diameter = diameter.max(max_distance);
        }
    }
    diameter
}

pub fn print(idx: usize, graph: &GraphAdjList) {
    let diameter = diameter(graph);
    if diameter != 0 {
        println!("Graph {idx} has diameter {diameter}.");
    } else {
        println!("Graph {idx} is disconnected.");
    }
}
