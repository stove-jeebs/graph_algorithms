use crate::GraphAdjList;

pub fn get_size(graph: &GraphAdjList) -> usize {
    let count: usize = graph
        .adj_list
        .values()
        .map(|neighbours| neighbours.len())
        .sum();
    count / 2
}

pub fn print(idx: usize, graph: &GraphAdjList) {
    let g = get_size(graph);
    println!("Graph {idx} has size {g}.");
}
