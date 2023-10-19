use crate::{GraphAdjList, Vertex};
use std::collections::{HashMap, HashSet, VecDeque};

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

fn find_connected_components(graph: &GraphAdjList) -> Vec<HashSet<Vertex>> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();

    for &vertex in graph.adj_list.keys() {
        if !visited.contains(&vertex) {
            let mut component = HashSet::new();
            let distances = bfs(graph, vertex);
            for vertex in distances.keys() {
                visited.insert(*vertex);
                component.insert(*vertex);
            }
            components.push(component);
        }
    }

    components
}

pub fn is_disconnected(graph: &GraphAdjList) -> bool {
    find_connected_components(graph).len() > 1
}

pub fn compute_diameter(graph: &GraphAdjList) -> usize {
    let mut max_diameter = 0;

    for component in find_connected_components(graph) {
        for &vertex in &component {
            let distances = bfs(graph, vertex);
            if let Some(&max_distance) = distances.values().max() {
                max_diameter = max_diameter.max(max_distance);
            }
        }
    }

    max_diameter
}

pub fn print(idx: usize, graph: &GraphAdjList) {
    let diameter = compute_diameter(graph);
    if !is_disconnected(graph) {
        println!("Graph {idx} has diameter {diameter}.");
    } else {
        println!("Graph {idx} is disconnected.");
    }
}
