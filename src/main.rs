use std::{collections::HashMap, io::stdin};

#[cfg(test)]
pub mod unit_test;

pub mod girth;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vertex(pub usize);

#[derive(Debug)]
pub struct GraphAdjList {
    pub adj_list: HashMap<Vertex, Vec<Vertex>>,
}

fn read_graph() -> Option<(GraphAdjList, usize)> {
    let mut num_nodes = String::new();
    stdin()
        .read_line(&mut num_nodes)
        .expect("Failed to read line");
    let num_nodes: usize = num_nodes
        .trim()
        .parse()
        .expect("Failed to parse num_nodes into i32");

    if num_nodes == 0 {
        return None;
    }

    let mut graph = GraphAdjList {
        adj_list: HashMap::new(),
    };
    for i in 0..num_nodes {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Failed to read line");
        let line: Vec<Vertex> = line
            .trim()
            .split_whitespace()
            .map(|num_str| {
                Vertex(
                    num_str
                        .parse::<usize>()
                        .expect("Failed to parse num_str into usize"),
                )
            })
            .collect();
        graph.adj_list.entry(Vertex(i)).or_insert(line);
    }

    Some((graph, num_nodes))
}

fn main() {
    let mut idx = 1;
    while let Some((mut graph, num_nodes)) = read_graph() {
        let g = girth::compute_girth(&mut graph, num_nodes);
        println!("Graph {} has girth {}.", idx, g);
        idx += 1;
    }
}
