use std::io::Read;
use std::fs::{self, File};
use std::io;
use ndarray::Array2;
use serde::{Serialize, Deserialize};
use serde_json;
use rand::prelude::*;
use rand::seq::SliceRandom;

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    adjacency_matrix: Array2<usize>,
    nodelist: Vec<Node>
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

    pub fn add_nodes_from(mut self, nodes: Vec<Node>) -> Self {
        self.nodelist.extend(nodes);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    identifer: NodeIdentifier
}

#[derive(Debug, Serialize, Deserialize)]
enum NodeIdentifier {
    Int(usize)
}

impl Default for Graph {
    fn default() -> Graph {
        Graph {
            adjacency_matrix: Array2::zeros((0,0)),
            nodelist: vec![]
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

pub fn create_from_nodes()
{

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
    }

    fn nodes_and_edges_can_be_added_to_an_empty_graph() {
        let graph = crate::Graph::new();

    }
}