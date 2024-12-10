type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Graph {
    pub n: usize, // Number of unique nodes
    pub outedges: AdjacencyLists, // Adjacency List 
    pub id_to_node: Vec<usize>, // Mapping from index to NodeID
}
impl Graph {
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
    pub fn calculate_out_degree(&self) -> Vec<(usize, usize)> {
        self.outedges
            .iter()
            .enumerate()
            .map(|(node, edges)| (node, edges.len()))
            .collect()
    }
    pub fn calculate_density(&self) -> f64 {
        let total_edges: usize = self.outedges.iter().map(|edges| edges.len()).sum();
        let total_nodes = self.n; 
        if total_nodes == 0 {
            0.0
        } else {
            total_edges as f64 / total_nodes as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_graph() {
        // Generate empty graph 
        let graph = Graph {
            n: 0,
            outedges: Vec::new(),
            id_to_node: Vec::new(),
        };
        assert_eq!(graph.n, 0);
        assert!(graph.outedges.is_empty());
        assert!(graph.id_to_node.is_empty());
    }
    #[test]
    fn test_single_edge_graph() {
        // Generate graph with one edge
        let edges = vec![(0, 1)];
        let mut graph = Graph {
            n: 2,
            outedges: vec![Vec::new(); 2],
            id_to_node: vec![1, 2],
        };
        graph.add_directed_edges(&edges);
        assert_eq!(graph.outedges[0], vec![1]);
        assert_eq!(graph.outedges[1], vec![]);
    }
    #[test]
    fn test_outdegree(){
        let graph = Graph{
            n: 3,
            outedges: vec![
                vec![1, 2], // Node 0 has edges to nodes 1 and 2
                vec![2],    // Node 1 has an edge to node 2
                vec![],     // Node 2 has no outgoing edges
            ],
            id_to_node: vec![0, 1, 2], 
        };

        // Expected out-degree
        let expected_out_degree = vec![
            (0, 2), 
            (1, 1), 
            (2, 0),
        ];
        
        // Calculated out-degree
        let actual_out_degree = graph.calculate_out_degree();

        // Check if out-degrees match
        assert_eq!(actual_out_degree, expected_out_degree);
    }
    #[test]
    fn test_density_calculation() {
        // Calculate density of small graph with 4 nodes and 5 directed edges
        let graph = Graph {
            n: 4,
            outedges: vec![vec![1, 2], vec![0, 2], vec![3], vec![]],
            id_to_node: vec![0, 1, 2, 3], 
        };

        let expected_density = 5.0 / 4.0;

        // Checking if the calculated density matches the expected value
        assert_eq!(graph.calculate_density(), expected_density);
    }
}