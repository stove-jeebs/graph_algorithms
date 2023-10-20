use std::{collections::HashMap, io::stdin};

pub mod component_order;
pub mod diameter;
pub mod girth;
pub mod matrix;
pub mod size;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vertex(pub usize);

#[derive(Debug)]
pub struct GraphAdjList {
    pub adj_list: HashMap<Vertex, Vec<Vertex>>,
}

impl Default for GraphAdjList {
    fn default() -> Self {
        Self {
            adj_list: HashMap::new(),
        }
    }
}

impl GraphAdjList {
    pub fn add_adj_list(&mut self, vet: Vertex, adj: Vec<Vertex>) {
        self.adj_list
            .entry(vet)
            .and_modify(|_| panic!("line already exists"))
            .or_insert(adj);
    }

    pub fn size(&self) -> usize {
        self.adj_list.len()
    }
}

fn read_graph() -> Option<(GraphAdjList, usize)> {
    // read input and initialize an adjacency list
    let mut num_nodes = String::new();
    stdin()
        .read_line(&mut num_nodes)
        .expect("Failed to read line");
    let num_nodes: usize = num_nodes
        .trim()
        .parse()
        .expect("Failed to parse num_nodes into usize");

    if num_nodes == 0 {
        return None;
    }

    let mut graph = GraphAdjList::default();
    for i in 0..num_nodes {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Failed to read line");
        let line: Vec<Vertex> = line
            .split_whitespace()
            .map(|num_str| {
                Vertex(
                    num_str
                        .parse::<usize>()
                        .expect("Failed to parse num_str into usize"),
                )
            })
            .collect();
        graph.add_adj_list(Vertex(i), line);
    }

    Some((graph, num_nodes))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut idx = 1;
    while let Some((graph, _num_nodes)) = read_graph() {
        if args.contains(&"--size".to_owned()) {
            // task 1
            size::print(idx, &graph);
        } else if args.contains(&"--component_order".to_owned()) {
            // task 2
            component_order::print(idx, &graph);
        } else if args.contains(&"--matrix".to_owned()) {
            // task 3
            matrix::print(&graph);
        } else if args.contains(&"--diameter".to_owned()) {
            // task 4
            diameter::print(idx, &graph);
        } else if args.contains(&"--girth".to_owned()) {
            // task 5
            girth::print(idx, &graph, _num_nodes);
        } else {
            println!("Please choose either --size or --girth");
        }
        idx += 1;
    }
}
