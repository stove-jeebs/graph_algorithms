#![allow(dead_code)]
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vertex(usize);

#[derive(Debug)]
struct GraphAdjList {
    pub adj_list: HashMap<Vertex, Vec<Vertex>>,
}

impl GraphAdjList {
    fn new(edges: Vec<[Vertex; 2]>) -> Self {
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

    fn add_edge(&mut self, vet1: Vertex, vet2: Vertex) {
        // add edge or panic if entry doesn't exists
        self.adj_list
            .entry(vet1)
            .and_modify(|vec| vec.push(vet2))
            .or_insert_with(|| panic!("Failed to add edge: Entry {:?} not found", vet1));
        self.adj_list
            .entry(vet2)
            .and_modify(|vec| vec.push(vet1))
            .or_insert_with(|| panic!("Failed to add edge: Entry {:?} not found", vet2));
    }

    fn add_vertex(&mut self, vet: Vertex) {
        // add vertex or panic if vertex exists
        self.adj_list
            .entry(vet)
            .and_modify(|_| panic!("Failed to add vertex: {:?} already exists", vet))
            .or_insert(vec![]);
    }

    fn cycle_length_bfs(&mut self, start_vet: Vertex) -> usize {
        let mut visited = HashSet::new();
        let mut que = VecDeque::new();
        let mut depths = HashMap::new();

        visited.insert((start_vet, start_vet));
        que.push_back((start_vet, 0));
        depths.insert(start_vet, 0);

        while let Some((vet, depth)) = que.pop_front() {
            if let Some(adj_vets) = self.adj_list.get(&vet) {
                for &adj_vet in adj_vets {
                    if visited.contains(&(adj_vet, vet)) {
                        continue; // skip the parent vertex
                    } else if visited.iter().any(|&(v, _)| v == adj_vet) {
                        // a cycle is detected if adj_vet is visited but not from vet
                        return depth + depths.get(&adj_vet).unwrap_or(&0) + 1;
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

    fn compute_girth(&mut self, num_nodes: usize) -> String {
        let mut girth = usize::MAX;
        for i in 0..num_nodes {
            let node = self.cycle_length_bfs(Vertex(i));
            girth = if node < girth { node } else { girth };
            if girth == 3 {
                return girth.to_string();
            }
        }

        if girth == usize::MAX {
            "infinity".to_string()
        } else {
            girth.to_string()
        }
    }
}

fn process_input() {}

fn main() {
    let mut idx = 1;
    loop {
        idx += 1;
    }
}
