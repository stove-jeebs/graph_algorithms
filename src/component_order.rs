use crate::{GraphAdjList, Vertex};
use std::collections::{HashSet, VecDeque};

fn dfs(vet: Vertex, graph: &GraphAdjList, visited: &mut HashSet<Vertex>) -> usize {
    if visited.contains(&vet) {
        return 0;
    }
    let mut res = vec![];
    let mut temp_visited = HashSet::new();
    temp_visited.insert(vet);
    let mut que = VecDeque::new();
    que.push_back(vet);
    while !que.is_empty() {
        let vet = que.pop_front().unwrap();
        res.push(vet);
        if let Some(adj_vets) = graph.adj_list.get(&vet) {
            for &adj_vet in adj_vets {
                if temp_visited.contains(&adj_vet) {
                    continue;
                }
                que.push_back(adj_vet);
                temp_visited.insert(adj_vet);
            }
        }
    }
    visited.extend(temp_visited);
    res.len()
}

pub fn largest_component_order(graph: &GraphAdjList) -> usize {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut max_order = 0;

    for (node, _) in graph.adj_list.iter() {
        let order = dfs(*node, &graph, &mut visited);
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
        assert_eq!(dfs(vet, &graph, &mut visited), 0);
    }

    #[test]
    fn test_dfs_unvisited_vet() {
        let graph = GraphAdjList::new();
        let vet = Vertex(1);
        let mut visited = HashSet::from([Vertex(0), Vertex(2)]);
        assert_eq!(dfs(vet, &graph, &mut visited), 1);
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
