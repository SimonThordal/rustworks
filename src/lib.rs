use std::io::Read;
use std::fs::{self, File};
use std::io;
use std::convert::From;
use std::collections::HashMap;
use ndarray::Array2;
use serde::{Serialize, Deserialize};
use serde_json;
use rand::prelude::*;
use rand::seq::SliceRandom;


#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    adjacency_matrix: Array2<usize>,
    nodes: HashMap<NodeIdentifier, Node>,
    edges: Vec<Edge>
}

impl Graph {
    pub fn print(&self) -> () {
        println!("{}", &self.adjacency_matrix);
    }

    pub fn save(&self, filename: &str) -> io::Result<()> {
        let serialized = serde_json::to_string(&self).unwrap();
        fs::write(filename, serialized)
    }

    pub fn new() -> Self {
        Self {
            ..Self::default()
        }
    }

    pub fn add_nodes_from(&mut self, nodes: Vec<impl AddsToGraph>) {
        for node in nodes.into_iter() {
            node.add_to_graph(self)
        }
    }

    pub fn add_edges_from(&mut self, edges: Vec<Edge>) {
        let nodes: Vec<NodeIdentifier> = edges.iter().map(|edge| [edge.source.clone(), edge.target.clone()]).flatten().collect();
        self.edges.extend(edges);
        self.add_nodes_from(nodes);
    }
}

pub trait AddsToGraph {
    fn add_to_graph(self, graph: &mut Graph);
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
enum NodeIdentifier {
    Int(usize),
    Text(String)
}

impl AddsToGraph for NodeIdentifier {
    fn add_to_graph(self, graph: &mut Graph) {
        if !graph.nodes.contains_key(&self) {
            let node = Node {
                identifier: self.clone()
            };
            graph.nodes.insert(self.clone(), node);
        }
    }
}

impl AddsToGraph for Node {
    fn add_to_graph(self, graph: &mut Graph) {
        graph.nodes.insert(self.identifier.clone(), self);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    identifier: NodeIdentifier
}

impl From<usize> for Node {
    fn from(identifer: usize) -> Self {
        Node {
            identifier: NodeIdentifier::Int(identifer)
        }
    }
}

impl From<String> for Node {
    fn from(identifier: String) -> Self {
        Node {
            identifier: NodeIdentifier::Text(identifier)
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    source: NodeIdentifier,
    target: NodeIdentifier
}

impl From<(Node, Node)> for Edge {
    fn from(tuple: (Node, Node)) -> Self {
        Edge {
            source: tuple.0.identifier.clone(),
            target: tuple.1.identifier.clone()
        }
    }
}

impl Default for Graph {
    fn default() -> Graph {
        Graph {
            adjacency_matrix: Array2::zeros((0,0)),
            nodes: HashMap::new(),
            edges: Vec::new()
        }
    }
}

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
        // We can nodes with integer identifiers to it
        let nodes: Vec<crate::Node> = vec![1,2,3].into_iter().map(|val| crate::Node::from(val)).collect();
        graph.add_nodes_from(nodes);
        assert_eq!(3, graph.nodes.len());
        // And we can keep adding nodes with different identifiers to it without problems
        let nodes: Vec<crate::Node> = vec![
            String::from("a"),
            String::from("b"),
            String::from("c")
        ].into_iter().map(|val| crate::Node::from(val)).collect();
        graph.add_nodes_from(nodes);
        assert_eq!(6, graph.nodes.len());
    }

    #[test]
    fn edges_can_be_added_between_nodes() {
        // When we have an empty graph
        let mut graph = crate::Graph::new();
        let edges: Vec<crate::Edge> = vec![(1,2),(2,3), (1,3)].into_iter().map(|val| 
            crate::Edge::from((crate::Node::from(val.0), crate::Node::from(val.1)))
        ).collect();
        graph.add_edges_from(edges);
        assert_eq!(3, graph.edges.len());
        assert_eq!(3, graph.nodes.len());
    }
}