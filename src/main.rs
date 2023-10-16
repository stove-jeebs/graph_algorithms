#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::stdin,
};

pub mod graph {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Vertex(pub usize);

    #[derive(Debug)]
    pub struct GraphAdjList {
        pub adj_list: HashMap<Vertex, Vec<Vertex>>,
    }

    impl GraphAdjList {
        pub fn new(edges: Vec<[Vertex; 2]>) -> Self {
            let mut graph = GraphAdjList {
                adj_list: HashMap::new(),
            };

            // populate graph
            for edge in edges {
                graph.add_vertex(edge[0]);
                graph.add_vertex(edge[1]);
                graph.add_edge(edge[0], edge[1]);
            }

            graph
        }

        pub fn add_edge(&mut self, vet1: Vertex, vet2: Vertex) {
            // add edge or panic if entry doesn't exists
            self.adj_list
                .entry(vet1)
                .and_modify(|vec| vec.push(vet2))
                .or_insert_with(|| panic!("Failed to add edge: Entry {vet1:?} not found"));
            self.adj_list
                .entry(vet2)
                .and_modify(|vec| vec.push(vet1))
                .or_insert_with(|| panic!("Failed to add edge: Entry {vet2:?} not found"));
        }

        pub fn add_vertex(&mut self, vet: Vertex) {
            // add vertex if it doesn't exist
            self.adj_list.entry(vet).or_insert(vec![]);
        }
    }
}

pub mod girth {
    use super::*;
    use graph::{GraphAdjList, Vertex};

    fn vet_cycle_len_bfs(graph: &mut GraphAdjList, start_vet: Vertex) -> usize {
        // return the cycle length for start_vet
        let mut visited = HashSet::new();
        let mut que = VecDeque::new();
        let mut depths = HashMap::new();

        visited.insert((start_vet, start_vet));
        que.push_back((start_vet, 0));
        depths.insert(start_vet, 0);

        while let Some((vet, depth)) = que.pop_front() {
            if let Some(adj_vets) = graph.adj_list.get(&vet) {
                for &adj_vet in adj_vets {
                    if visited.contains(&(adj_vet, vet)) {
                        continue; // skip the parent vertex
                    } else if visited.iter().any(|&(v, _)| v == adj_vet) {
                        // a cycle is detected if adj_vet is visited but not from vet
                        return depth + depths.get(&adj_vet).expect("adj_vet doesn't exist") + 1;
                    } else {
                        visited.insert((adj_vet, vet));
                        que.push_back((adj_vet, depth + 1));
                        depths.insert(adj_vet, depth + 1);
                    }
                }
            }
        }
        return usize::MAX;
    }

    pub fn compute_girth(graph: &mut GraphAdjList, num_nodes: usize) -> String {
        let mut girth = usize::MAX;
        for i in 0..num_nodes {
            let node = vet_cycle_len_bfs(graph, Vertex(i));
            girth = if node < girth { node } else { girth };
            if girth == 3 {
                return girth.to_string();
            }
        }

        if girth == usize::MAX {
            "infinity".to_owned()
        } else {
            girth.to_string()
        }
    }
}

fn main() {
    use graph::{GraphAdjList, Vertex};

    let mut idx = 1;
    loop {
        let mut num_nodes = String::new();
        stdin()
            .read_line(&mut num_nodes)
            .expect("Failed to read line");
        let num_nodes: usize = num_nodes
            .trim()
            .parse()
            .expect("Failed to parse num_nodes into i32");

        if num_nodes == 0 {
            break;
        }

        let mut graph = GraphAdjList {
            adj_list: HashMap::new(),
        };
        for i in 0..num_nodes {
            let mut line = String::new();
            stdin().read_line(&mut line).expect("Failed to read line");
            // turn a string of numbers separated by spaces into Vec<Vertex(usize)>
            let line: Vec<Vertex> = line
                .trim()
                .split_whitespace()
                .map(|num_str| {
                    Vertex(
                        num_str
                            .parse::<usize>()
                            .expect("Failed to parse into a usize"),
                    )
                })
                .collect();
            // insert line ito adjacency list if entry doesn't exist
            graph.adj_list.entry(Vertex(i)).or_insert(line);
        }

        let g = girth::compute_girth(&mut graph, num_nodes);
        println!("Graph {idx} has girth {g}.");
        idx += 1;
    }
}
