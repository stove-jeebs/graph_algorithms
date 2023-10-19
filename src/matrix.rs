use crate::GraphAdjList;

#[derive(Debug)]
pub enum GraphConversionError {
    OutOfBoundsVertex(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub struct GraphAdjMat {
    adj_mat: Vec<Vec<u8>>,
}

impl GraphAdjMat {
    pub fn new() -> Self {
        Self {
            adj_mat: Vec::new(),
        }
    }

    pub fn new_from_size(num_nodes: usize) -> Self {
        Self {
            adj_mat: vec![vec![0; num_nodes]; num_nodes],
        }
    }

    pub fn add_col(&mut self, col: Vec<u8>) {
        self.adj_mat.push(col);
    }

    pub fn set_edge(&mut self, from: usize, to: usize) {
        self.adj_mat[from][to] = 1;
    }

    pub fn size(&self) -> usize {
        self.adj_mat.len()
    }
}

pub fn adj_list_to_mat(graph: &GraphAdjList) -> Result<GraphAdjMat, GraphConversionError> {
    let mut matrix = GraphAdjMat::new_from_size(graph.size());

    for (node, neighbours) in graph.adj_list.iter() {
        if node.0 >= matrix.adj_mat.len() {
            return Err(GraphConversionError::OutOfBoundsVertex(node.0));
        }
        for vet in neighbours {
            if vet.0 >= matrix.adj_mat[node.0].len() {
                return Err(GraphConversionError::OutOfBoundsVertex(vet.0));
            }
            matrix.adj_mat[node.0][vet.0] = 1;
        }
    }

    Ok(matrix)
}

pub fn print(graph: &GraphAdjList) {
    // converts an adjacency list into a matrix
    let matrix: GraphAdjMat = match adj_list_to_mat(&graph) {
        Ok(graph_mat) => graph_mat,
        Err(err) => panic!("GraphConversionError: {err:?}"),
    };
    let mut builder = String::from(format!("{}", matrix.size()));
    // for each column in the matrix join the values into a string separated by spaces between
    for col in matrix.adj_mat.iter() {
        builder.push_str(&format!(
            "\n{}",
            col.iter()
                .map(|&n| n.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        ));
    }

    println!("{builder}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vertex;

    #[test]
    fn test_conversion() {
        let mut graph = GraphAdjList::new();
        graph.add_adj_list(Vertex(0), vec![Vertex(1)]);
        graph.add_adj_list(Vertex(1), vec![Vertex(0), Vertex(2)]);
        graph.add_adj_list(Vertex(2), vec![Vertex(1)]);

        let expected_matrix = GraphAdjMat {
            adj_mat: vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]],
        };

        assert_eq!(graph.size(), 3);
        assert_eq!(adj_list_to_mat(&graph).unwrap(), expected_matrix);
    }

    #[test]
    fn test_print_matrix() {
        let mut graph = GraphAdjList::new();
        graph.add_adj_list(Vertex(0), vec![Vertex(1)]);
        graph.add_adj_list(Vertex(1), vec![Vertex(0), Vertex(2)]);
        graph.add_adj_list(Vertex(2), vec![Vertex(1)]);

        assert_eq!(1, 1);
    }
}
