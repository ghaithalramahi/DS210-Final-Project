type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Graph {
    pub n: usize, // Number of unique nodes
    pub outedges: AdjacencyLists,
    pub id_to_node: Vec<usize>, // Mapping from index to NodeID
}

impl Graph {
    pub fn calculate_out_degree(&self) -> Vec<(usize, usize)> {
        self.outedges
            .iter()
            .enumerate()
            .map(|(node, edges)| (node, edges.len()))
            .collect()
    }
    pub fn remove_node(&mut self, node: usize) {
        // Clear all outgoing edges of the node
        self.outedges[node].clear();

        // Remove incoming edges to the node
        for neighbors in &mut self.outedges {
            neighbors.retain(|&neighbor| neighbor != node);
        }
    }
    pub fn calculate_density(&self) -> f64 {
        let total_edges: usize = self.outedges.iter().map(|edges| edges.len()).sum();
        let total_nodes = self.outedges.iter().filter(|edges| !edges.is_empty()).count();
        if total_nodes == 0 {
            0.0
        } else {
            total_edges as f64 / total_nodes as f64
        }
    }

    pub fn read_file(file_name: &str) -> Graph {
        let file = File::open(file_name).unwrap();
        let reader = io::BufReader::new(file);

        let mut node_map = HashMap::new(); // Maps NodeIDs to indices
        let mut id_to_node = Vec::new(); // Maps indices back to NodeIDs
        let mut edges = Vec::new();
        let mut current_index = 0;

        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            // Parse the citing and cited papers directly
            let citing_node: usize = parts[0].parse().unwrap();
            let cited_node: usize = parts[1].parse().unwrap();

            // Map both nodes and retrieve their indices
            let citing_index = Self::map_node(&mut node_map, &mut id_to_node, citing_node, &mut current_index);
            let cited_index = Self::map_node(&mut node_map, &mut id_to_node, cited_node, &mut current_index);

            edges.push((citing_index, cited_index));
        }

        // Create the graph with the mapped indices
        let n = id_to_node.len();
        let mut graph = Graph {
            n,
            outedges: vec![Vec::new(); n],
            id_to_node,
        };

        graph.add_directed_edges(&edges);
        graph.sort_graph_lists();
        graph
    }
    pub fn print_vertex(&self, vertex: usize) {
        if vertex >= self.n {
            println!("Vertex {} does not exist in the graph.", vertex);
            return;
        }

        // Map the internal index back to the original NodeID
        let original_node = self.id_to_node[vertex];

        // Retrieve the outgoing edges and map them back to NodeIDs
        let edges: Vec<_> = self.outedges[vertex]
            .iter()
            .map(|&v| self.id_to_node[v])
            .collect();

        println!("Vertex (NodeID) {}: Edges -> {:?}", original_node, edges);
    }
    fn map_node(
        node_map: &mut HashMap<usize, usize>,
        id_to_node: &mut Vec<usize>,
        node: usize,
        current_index: &mut usize,
    ) -> usize {
        *node_map.entry(node).or_insert_with(|| {
            id_to_node.push(node);
            let idx = *current_index;
            *current_index += 1;
            idx
        })
    }

    fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for &(u, v) in edges {
            self.outedges[u].push(v);
        }
    }

    fn sort_graph_lists(&mut self) {
        for neighbors in self.outedges.iter_mut() {
            neighbors.sort();
        }
    }
}