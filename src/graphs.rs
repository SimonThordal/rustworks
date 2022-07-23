
use std::fs::{self};
use std::io;
use std::convert::From;
use std::collections::HashMap;
use ndarray::Array2;
use serde::{Serialize, Deserialize};
use serde_json;


/// A struct containing the nodes and edges that make up a graph
/// ```rust
/// let g = Graph::new();
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    pub adjacency_matrix: Array2<usize>,
    pub nodes: HashMap<NodeIdentifier, Node>,
    pub edges: Vec<Edge>
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

    /// Add a node to the graph
    ///
    pub fn add_node(&mut self, node: impl AddsToGraph) {
        node.add_to_graph(self)
    }

    // Add multiple nodes to the graph
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
pub enum NodeIdentifier {
    Int(usize),
    Text(String)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum EdgeIdentifier {
    Int(usize),
    Test(String)
}

impl AddsToGraph for NodeIdentifier {
    fn add_to_graph(self, graph: &mut Graph) {
        if !graph.nodes.contains_key(&self) {
            let node = Node {
                identifier: self.clone(),
                edges: vec![]
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
    pub identifier: NodeIdentifier,
    pub edges: Vec<HashMap<NodeIdentifier, EdgeIdentifier>>
}

impl Node {
    pub fn add_edge_to(&self, _target: &Node) {

    } 
}

impl From<usize> for Node {
    fn from(identifer: usize) -> Self {
        Node {
            identifier: NodeIdentifier::Int(identifer),
            edges:vec![]
        }
    }
}

impl From<String> for Node {
    fn from(identifier: String) -> Self {
        Node {
            identifier: NodeIdentifier::Text(identifier),
            edges:vec![]
        }
    }
}

impl From<&str> for Node {
    fn from(identifier: &str) -> Self {
        Node {
            identifier: NodeIdentifier::Text(identifier.to_string()),
            edges:vec![]
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    source: NodeIdentifier,
    target: NodeIdentifier
}

// Edges
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