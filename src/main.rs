use std::collections::HashMap;
mod graph_subroutines;
use graph_subroutines::Graph;

fn main() {
    let file_name = "Cit-HepTh.txt";

    // Generate the citation graph
    let mut citation_graph = Graph::read_file(file_name);
    let denser_graph = denser_subgraph(&mut citation_graph);

    let mut component = vec![None;denser_graph.n];
    let mut component_count = 0;

    for v in 0..denser_graph.n {
        if let None = component[v] {
            component_count += 1;
            mark_component_dfs(v,&denser_graph,&mut component,component_count);
        }
    };
    
    // Find the top 10 densest components
    let top_components = find_densest_components(&denser_graph, &component);

    // Print the results
    println!("Top 10 Densest Components:");
    for (comp_id, density, vertices) in top_components {
        println!("Component {}: Density {:.4}, Vertices: {:?}", comp_id, density, vertices);
    }
    
    // Calculate the average density
    let avg_density = calculate_average_density(&denser_graph, &component);
    println!("Average Density Across All Components: {:.4}", avg_density);
}

fn calculate_average_density(graph: &Graph, component: &Vec<Option<usize>>) -> f64 {
    let mut vertex_count: HashMap<usize, Vec<usize>> = HashMap::new();

    // Group vertices by component
    for (vertex, &comp_id) in component.iter().enumerate() {
        if let Some(comp_id) = comp_id {
            vertex_count.entry(comp_id).or_insert_with(Vec::new).push(vertex);
        }
    }

    let mut total_density = 0.0;

    // Calculate density for each component
    for (_, vertices) in vertex_count.iter() {
        let mut subgraph = Graph {
            n: vertices.len(),
            outedges: vec![Vec::new(); vertices.len()],
            id_to_node: vertices.clone(),
        };

        let vertex_map: HashMap<usize, usize> = vertices
            .iter()
            .enumerate()
            .map(|(index, &v)| (v, index))
            .collect();

            // Build subgraph for the component
        for &v in vertices {
            let v_new = vertex_map[&v];
            for &neighbor in &graph.outedges[v] {
                if let Some(&neighbor_new) = vertex_map.get(&neighbor) {
                    subgraph.outedges[v_new].push(neighbor_new);
                }
            }
        }

         // Calculate density for the subgraph
        let density = subgraph.calculate_density();
        total_density += density;
        }

    // Calculate average density
    total_density / vertex_count.len() as f64
}


type Vertex = usize;
type Component = usize;

fn mark_component_dfs(vertex:Vertex, graph:&Graph, component:&mut Vec<Option<Component>>, component_no:Component){
    component[vertex] = Some(component_no);
    for w in graph.outedges[vertex].iter(){
        if let None = component[*w] {
            mark_component_dfs(*w, graph, component, component_no);
        }
    }
}

fn find_densest_components(graph: &Graph, component:&Vec<Option<Component>>) 
    -> Vec<(usize, f64, Vec<usize>)>{
        let mut vertex_count: HashMap<usize, Vec<usize>> = HashMap::new();

        // Group vertices by component
        for (vertex, &comp_id) in component.iter().enumerate() {
            if let Some(comp_id) = comp_id {
                vertex_count.entry(comp_id).or_insert_with(Vec::new).push(vertex);
            }
        }

        let mut densities = Vec::new();

        // Calculate density of each component
        for (comp_id, vertices) in vertex_count.iter(){
            if vertices.len() < 50{
                let mut subgraph = Graph {
                    n: vertices.len(),
                    outedges: vec![Vec::new(); vertices.len()],
                    id_to_node: vertices.clone(),
                };
                let vertex_map: HashMap<usize, usize> = vertices
                .iter()
                .enumerate()
                .map(|(index, &v)| (v, index))
                .collect();
    
                // Build subgraph for the component
                for &v in vertices {
                    let v_new = vertex_map[&v];
                    for &neighbor in &graph.outedges[v] {
                        if let Some(&neighbor_new) = vertex_map.get(&neighbor) {
                            subgraph.outedges[v_new].push(neighbor_new);
                        }
                    }
                }
                // Calculate density for the subgraph
                let density = subgraph.calculate_density();
                densities.push((*comp_id, density, vertices.clone()));
            }
        }
        // Sort by densest and return the top 10
        densities.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
        densities.into_iter().take(10).collect()
}

fn denser_subgraph(graph: &mut Graph) -> Graph{
    let mut degrees = graph.calculate_out_degree();
    degrees.sort_by_key(|&(_, degree)| degree); // Sort by out-degree (ascending)

    let total_nodes = graph.n;
    let num_to_keep = total_nodes - total_nodes / 4; // Remove 25% least connected 
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_densest_components() {
        let graph = Graph {
            n: 6,
            outedges: vec![
                vec![1],    // Node 0 -> Node 1
                vec![],    // Node 1 has no outgoing edges
                vec![3, 4], // Node 2 -> Nodes 3, 4
                vec![2],    // Node 3 -> Node 2
                vec![],     // Node 4 has no outgoing edges
                vec![],     // Node 5 is isolated
            ],
            id_to_node: vec![0, 1, 2, 3, 4, 5],
        };
    
        let component = vec![Some(1), Some(1), Some(2), Some(2), Some(2), None];
    
        let expected = vec![
            (2, 1.0, vec![2, 3, 4]), // Component 2: Density = 3 / 3 = 1.0
            (1, 0.5, vec![0, 1]),    // Component 1: Density = 1 / 2 = 0.5
        ];
    
        let result = find_densest_components(&graph, &component);
    
        assert_eq!(result, expected);
    }
}

