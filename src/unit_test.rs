use super::girth;
use super::{GraphAdjList, Vertex};
use std::collections::HashMap;

#[test]
fn test_vet_cycle_len_bfs_no_cycle() {
    let mut graph = GraphAdjList {
        adj_list: HashMap::new(),
    };
    graph.adj_list.insert(Vertex(0), vec![Vertex(1)]);
    graph.adj_list.insert(Vertex(1), vec![Vertex(0), Vertex(2)]);
    graph.adj_list.insert(Vertex(2), vec![Vertex(1)]);

    assert_eq!(girth::vet_cycle_len_bfs(&mut graph, Vertex(0)), usize::MAX);
}

#[test]
fn test_vet_cycle_len_bfs_with_cycle() {
    let mut graph = GraphAdjList {
        adj_list: HashMap::new(),
    };
    graph.adj_list.insert(Vertex(0), vec![Vertex(1)]);
    graph.adj_list.insert(Vertex(1), vec![Vertex(0), Vertex(2)]);
    graph.adj_list.insert(Vertex(2), vec![Vertex(1), Vertex(3)]);
    graph.adj_list.insert(Vertex(3), vec![Vertex(2), Vertex(0)]);

    assert_eq!(girth::vet_cycle_len_bfs(&mut graph, Vertex(0)), 4);
}

#[test]
fn test_compute_girth_no_cycle() {
    let mut graph = GraphAdjList {
        adj_list: HashMap::new(),
    };
    graph.adj_list.insert(Vertex(0), vec![Vertex(1)]);
    graph.adj_list.insert(Vertex(1), vec![Vertex(0), Vertex(2)]);
    graph.adj_list.insert(Vertex(2), vec![Vertex(1)]);

    assert_eq!(girth::compute_girth(&mut graph, 3), "infinity".to_string());
}

#[test]
fn test_compute_girth_with_cycle() {
    let mut graph = GraphAdjList {
        adj_list: HashMap::new(),
    };
    graph.adj_list.insert(Vertex(0), vec![Vertex(1)]);
    graph.adj_list.insert(Vertex(1), vec![Vertex(0), Vertex(2)]);
    graph.adj_list.insert(Vertex(2), vec![Vertex(1), Vertex(3)]);
    graph.adj_list.insert(Vertex(3), vec![Vertex(2), Vertex(0)]);

    assert_eq!(girth::compute_girth(&mut graph, 4), "4".to_string());
}
