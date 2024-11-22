use std::collections::HashMap;
mod graph_subroutines;
use graph_subroutines::Graph;

fn main() {
    let file_name = "Cit-HepTh.txt";

    // Generate the citation graph
    let mut citation_graph = Graph::read_file(file_name);
    let denser_graph = denser_subgraph(&mut citation_graph);

    println!("{:?}", denser_graph)
}

fn denser_subgraph(graph: &mut Graph) -> Graph{
    let mut degrees = graph.calculate_out_degree();
    degrees.sort_by_key(|&(_, degree)| degree); // Sort by out-degree (ascending)

    let total_nodes = graph.n;
    let num_to_keep = total_nodes - total_nodes / 4; // Top 75% of nodes
    let to_keep: Vec<usize> = degrees.iter().rev().take(num_to_keep).map(|&(node, _)| node).collect();

    // Create the denser graph
    let mut new_graph = Graph {
        n: num_to_keep,
        outedges: vec![Vec::new(); num_to_keep],
        id_to_node: to_keep.clone(),
    };

    // Build the new graph, keeping only edges between the retained nodes
    let node_map: HashMap<usize, usize> = to_keep
        .iter()
        .enumerate()
        .map(|(index, &node)| (node, index))
        .collect();

    for &node in &to_keep {
        let new_node = node_map[&node];
        for &neighbor in &graph.outedges[node] {
            if let Some(&new_neighbor) = node_map.get(&neighbor) {
                new_graph.outedges[new_node].push(new_neighbor);
            }
        }
    }
    new_graph
}
