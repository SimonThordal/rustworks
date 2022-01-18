fn main() {
    let graph = rustworks::generate_graph(100);
    graph.print();
    graph.save("graph.json").expect("Failed to save graph");
    rustworks::load_graph("graph.json");
    println!("Succesfully generated, serialized and loaded graph");
}
