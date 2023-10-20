use crate::{GraphAdjList, Vertex};
use std::collections::{HashSet, VecDeque};

// find the order of a component
fn bfs(start_vet: &Vertex, graph: &GraphAdjList, visited: &mut HashSet<Vertex>) -> usize {
    if visited.contains(start_vet) {
        return 0;
    }
    let mut order = 0;
    visited.insert(*start_vet);
    let mut que = VecDeque::new();
    que.push_back(*start_vet);
    while let Some(vet) = que.pop_front() {
        order += 1;
        if let Some(adj_vets) = graph.adj_list.get(&vet) {
            for &neighbour in adj_vets {
                if !visited.contains(&neighbour) {
                    que.push_back(neighbour);
                    visited.insert(neighbour);
                }
            }
        }
    }
    order
}

pub fn largest_component_order(graph: &GraphAdjList) -> usize {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut max_order = 0;

    for (node, _) in graph.adj_list.iter() {
        let order = bfs(node, graph, &mut visited);
        if order > max_order {
            max_order = order;
        }
    }
    max_order
}

pub fn print(idx: usize, graph: &GraphAdjList) {
    println!(
        "Graph {} has a component of order {}.",
        idx,
        largest_component_order(graph)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_visited_vet() {
        let graph = GraphAdjList::new();
        let vet = Vertex(1);
        let mut visited = HashSet::from([Vertex(0), Vertex(1)]);
        assert_eq!(bfs(&vet, &graph, &mut visited), 0);
    }

    #[test]
    fn test_dfs_unvisited_vet() {
        let graph = GraphAdjList::new();
        let vet = Vertex(1);
        let mut visited = HashSet::from([Vertex(0), Vertex(2)]);
        assert_eq!(bfs(&vet, &graph, &mut visited), 1);
    }

    #[test]
    fn test_larges_component_order() {
        /*
         * has 3 components:
         * 1. vertices 0, 1, 2
         * 2. vertices 3, 4
         * 3. vertex 5
         *
         * largest component order is 3
         * */
        let mut graph = GraphAdjList::new();
        graph.add_adj_list(Vertex(0), vec![Vertex(1), Vertex(2)]);
        graph.add_adj_list(Vertex(1), vec![Vertex(0)]);
        graph.add_adj_list(Vertex(2), vec![Vertex(0)]);
        graph.add_adj_list(Vertex(3), vec![Vertex(4)]);
        graph.add_adj_list(Vertex(4), vec![Vertex(3)]);
        graph.add_adj_list(Vertex(5), vec![]);
        assert_eq!(largest_component_order(&graph), 3);
    }
}
