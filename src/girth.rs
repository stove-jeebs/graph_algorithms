use crate::{GraphAdjList, Vertex};
use std::collections::{HashMap, VecDeque};

fn vet_cycle_len_bfs(graph: &GraphAdjList, start_vet: Vertex) -> usize {
    let mut que = VecDeque::new();
    let mut visited: HashMap<Vertex, Option<Vertex>> = HashMap::new();
    let mut depths = HashMap::new();

    que.push_back((start_vet, 0));
    visited.insert(start_vet, None);
    depths.insert(start_vet, 0);

    while let Some((vet, depth)) = que.pop_front() {
        if let Some(adj_vets) = graph.adj_list.get(&vet) {
            for &adj_vet in adj_vets {
                if visited.contains_key(&adj_vet) {
                    if let Some(parent) = visited.get(&vet) {
                        if Some(&adj_vet) != parent.as_ref() {
                            return depth
                                + depths.get(&adj_vet).expect("adj_vet doesn't exist")
                                + 1;
                        }
                    }
                } else {
                    visited.insert(adj_vet, Some(vet));
                    depths.insert(adj_vet, depth + 1);
                    que.push_back((adj_vet, depth + 1));
                }
            }
        }
    }
    usize::MAX
}

pub fn compute_girth(graph: &GraphAdjList, num_nodes: usize) -> String {
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

pub fn print(idx: usize, graph: &GraphAdjList, num_nodes: usize) {
    let girth = compute_girth(graph, num_nodes);
    println!("Graph {idx} has girth {girth}.");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vet_cycle_len_bfs_no_cycle() {
        let mut graph = GraphAdjList::default();
        graph.add_adj_list(Vertex(0), vec![Vertex(1)]);
        graph.add_adj_list(Vertex(1), vec![Vertex(0), Vertex(2)]);
        graph.add_adj_list(Vertex(2), vec![Vertex(1)]);

        assert_eq!(vet_cycle_len_bfs(&mut graph, Vertex(0)), usize::MAX);
    }

    #[test]
    fn test_vet_cycle_len_bfs_with_cycle() {
        let mut graph = GraphAdjList::default();
        graph.add_adj_list(Vertex(0), vec![Vertex(1)]);
        graph.add_adj_list(Vertex(1), vec![Vertex(0), Vertex(2)]);
        graph.add_adj_list(Vertex(2), vec![Vertex(1), Vertex(3)]);
        graph.add_adj_list(Vertex(3), vec![Vertex(2), Vertex(0)]);

        assert_eq!(vet_cycle_len_bfs(&mut graph, Vertex(0)), 4);
    }

    #[test]
    fn test_compute_girth_no_cycle() {
        let mut graph = GraphAdjList::default();
        graph.add_adj_list(Vertex(0), vec![Vertex(1)]);
        graph.add_adj_list(Vertex(1), vec![Vertex(0), Vertex(2)]);
        graph.add_adj_list(Vertex(2), vec![Vertex(1)]);

        assert_eq!(compute_girth(&mut graph, 3), "infinity".to_string());
    }

    #[test]
    fn test_compute_girth_with_cycle() {
        let mut graph = GraphAdjList::default();
        graph.add_adj_list(Vertex(0), vec![Vertex(1)]);
        graph.add_adj_list(Vertex(1), vec![Vertex(0), Vertex(2)]);
        graph.add_adj_list(Vertex(2), vec![Vertex(1), Vertex(3)]);
        graph.add_adj_list(Vertex(3), vec![Vertex(2), Vertex(0)]);

        assert_eq!(compute_girth(&mut graph, 4), "4".to_string());
    }
}
