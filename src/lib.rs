use ndarray::Array2;
use std::io::Read;
use std::fs::{File};
use serde_json;
use rand::prelude::*;
use rand::seq::SliceRandom;

mod graphs;
use graphs::{Graph, Edge, Node};

pub fn generate_graph(n: usize) -> Graph {
    let mut graph = Graph {
        adjacency_matrix: Array2::zeros((n,n)),
        ..Graph::default()
    };
    let mut rng = rand::thread_rng();
    let potential_targets: Vec<usize> = (0..n).collect();
    for i in 0..n {
        let n_edges = rng.gen_range(0..n);
        let targets: Vec<usize> = potential_targets.choose_multiple(&mut rng, n_edges).cloned().collect();
        for j in targets {
            graph.adjacency_matrix[[i,j]] = 1;
            graph.adjacency_matrix[[j,i]] = 1;
        }
    }
    graph
}

pub fn load_graph(filename: &str) -> Graph {
    let mut file = File::open(filename).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    serde_json::from_str(&s).unwrap()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_creates_graphs_of_right_size() {
    	let graph = crate::generate_graph(2);
        assert_eq!(graph.adjacency_matrix.shape(), [2,2]);
        let graph = crate::generate_graph(4);
        assert_eq!(graph.adjacency_matrix.shape(), [4,4]);
    }

    #[test]
    fn it_can_create_an_empty_graph() {
        let graph = crate::Graph::new();
        assert_eq!(graph.adjacency_matrix.shape(), [0,0]);
        assert_eq!(0, graph.nodes.len());
    }

    #[test]
    fn nodes_can_be_added_to_a_graph() {
        // When we have an empty graph
        let mut graph = crate::Graph::new();
        // We can add nodes with integer identifiers to it
        let nodes: Vec<crate::Node> = vec![1,2,3].into_iter().map(|val| crate::Node::from(val)).collect();
        graph.add_nodes_from(nodes);
        assert_eq!(3, graph.nodes.len());
        // And we can keep adding nodes with different identifiers to it
        let nodes: Vec<crate::Node> = vec![
            String::from("a"),
            String::from("b"),
            String::from("c")
        ].into_iter().map(|val| crate::Node::from(val)).collect();
        graph.add_nodes_from(nodes);
        assert_eq!(6, graph.nodes.len());
        // A single node can be added to the graph
        graph.add_node(crate::Node::from("foo"));
        assert_eq!(7, graph.nodes.len());
    }

    #[test]
    fn edges_can_be_added_between_nodes() {
        // When we have an empty graph
        let mut graph = crate::Graph::new();
        // And a collection of edges
        let edges: Vec<crate::Edge> = vec![(1,2),(2,3), (1,3)].into_iter().map(|val| 
            crate::Edge::from((crate::Node::from(val.0), crate::Node::from(val.1)))
        ).collect();
        // We can add the edges to the graph
        graph.add_edges_from(edges);
        assert_eq!(3, graph.edges.len());
        // And any nodes in the edges will be added as well
        assert_eq!(3, graph.nodes.len());
    }

    #[test]
    fn an_edge_can_be_added_between_nodes() {
        let source = crate::Node::from("foo");
        let target = crate::Node::from("bar");
        source.add_edge_to(&target);
        assert_eq!(1, source.edges.len());
        assert_eq!(1, target.edges.len());

    }
}