use std::fs::File;
use std::io::{self, BufRead};
use petgraph::graph::{DiGraph};
use petgraph::visit::Bfs;

fn read_graph_from_file(filename: &str) -> DiGraph<(), ()> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut graph = DiGraph::<(), ()>::new();
    let mut node_indices = std::collections::HashMap::new();

    
    for (line_number, line) in reader.lines().enumerate() {
        if let Ok(edge) = line {
            let nodes: Vec<&str> = edge.split_whitespace().collect();

            // Check if the line contains two nodes
            if nodes.len() == 2 {
                let source_index = nodes[0].parse::<usize>().expect(&format!("Invalid source node at line {}", line_number + 1));
                let target_index = nodes[1].parse::<usize>().expect(&format!("Invalid target node at line {}", line_number + 1));

                // Add nodes to the graph 
                let source = *node_indices.entry(source_index).or_insert_with(|| graph.add_node(()));
                let target = *node_indices.entry(target_index).or_insert_with(|| graph.add_node(()));

                // Add edge to the graph
                graph.add_edge(source, target, ());
            } else {
                eprintln!("Invalid edge at line {}: {}", line_number + 1, edge);
            }
        } else {
            eprintln!("Error reading line {}", line_number + 1);
        }
    }

    graph
}

fn average_degree_of_separation(graph: &DiGraph<(), ()>) -> f64 {
    let mut total_degree = 0;
    let mut total_pairs = 0;

    for start_node in graph.node_indices() {
        let mut bfs = Bfs::new(graph, start_node);
        let mut depth_map = std::collections::HashMap::new();
        depth_map.insert(start_node, 0);

        // Perform Breadth-First Search
        while let Some(node) = bfs.next(graph) {
            let depth = *depth_map.get(&node).unwrap();
            total_degree += depth;
            total_pairs += 1;

            for neighbor in graph.neighbors(node) {
                if !depth_map.contains_key(&neighbor) {
                    depth_map.insert(neighbor, depth + 1);
                }
            }
        }
    }

    // Calculate and return the average degree of separation
    if total_pairs > 0 {
        total_degree as f64 / total_pairs as f64
    } else {
        0.0
    }
}

fn largest_degree_of_separation(graph: &DiGraph<(), ()>) -> usize {
    let mut max_degree = 0;

    for start_node in graph.node_indices() {
        let mut bfs = Bfs::new(graph, start_node);
        let mut depth_map = std::collections::HashMap::new();
        depth_map.insert(start_node, 0);

        // Perform Breadth-First Search
        while let Some(node) = bfs.next(graph) {
            let depth = *depth_map.get(&node).unwrap();
            max_degree = max_degree.max(depth);

            // Update depth map for neighbors
            for neighbor in graph.neighbors(node) {
                if !depth_map.contains_key(&neighbor) {
                    depth_map.insert(neighbor, depth + 1);
                }
            }
        }
    }

    max_degree
}

fn print_graph_statistics(graph: &DiGraph<(), ()>) {
    let num_nodes = graph.node_count();
    let num_edges = graph.edge_count();

    println!("Number of Nodes: {}", num_nodes);
    println!("Number of Edges: {}", num_edges);
}

fn clustering_coefficient(graph: &DiGraph<(), ()>) -> f64 {
    let mut total_coefficient = 0.0;

    for node in graph.node_indices() {
        let neighbors: Vec<_> = graph.neighbors(node).collect();
        let num_neighbors = neighbors.len();

        if num_neighbors >= 2 {
            let mut edges_within_neighbors = 0;

            // Count edges between neighbors
            for i in 0..num_neighbors {
                for j in (i + 1)..num_neighbors {
                    if graph.contains_edge(neighbors[i], neighbors[j]) || graph.contains_edge(neighbors[j], neighbors[i]) {
                        edges_within_neighbors += 1;
                    }
                }
            }

            // Calculate clustering coefficient for the current node
            let coefficient = 2.0 * edges_within_neighbors as f64 / (num_neighbors * (num_neighbors - 1)) as f64;
            total_coefficient += coefficient;
        }
    }

    // Calculate and return the average clustering coefficient
    if graph.node_count() > 0 {
        total_coefficient / graph.node_count() as f64
    } else {
        0.0
    }
}

fn print_extreme_nodes(graph: &DiGraph<(), ()>) {
    let highest_degree_node = graph
        .node_indices()
        .max_by_key(|&node| graph.neighbors(node).count())
        .unwrap();

    let lowest_degree_node = graph
        .node_indices()
        .min_by_key(|&node| graph.neighbors(node).count())
        .unwrap();

    println!("Node with Highest Degree: {:?}", highest_degree_node.index());
    println!("Node with Lowest Degree: {:?}", lowest_degree_node.index());
}

fn main() {
    let mut filename = String::new();
 
    io::stdin().read_line(&mut filename).expect("failed to readline");
    let filename = filename.trim(); 

    let graph = read_graph_from_file(filename);
    println!("Graph Read Successfully!\n\nProcessing Data...\n");

    let avg_degree = average_degree_of_separation(&graph);
    let max_degree = largest_degree_of_separation(&graph);
    let clustering_coeff = clustering_coefficient(&graph);

    println!("Average Degree of Separation: {:.2}", avg_degree);
    println!("Largest Degree of Separation: {}", max_degree);
    println!("Clustering Coefficient: {:.4}", clustering_coeff);

    print_graph_statistics(&graph);
    print_extreme_nodes(&graph);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_graph_from_file() {
        let graph = read_graph_from_file("test_graph.txt");
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 4);
    }

    #[test]
    fn test_average_degree_of_separation() {
        let  graph = read_graph_from_file("test_graph.txt");
        let avg_degree = average_degree_of_separation(&graph);
        assert_eq!(avg_degree, 1.5);
    }

    #[test]
    fn test_largest_degree_of_separation() {
        let  graph = read_graph_from_file("test_graph.txt");
        let max_degree = largest_degree_of_separation(&graph);
        assert_eq!(max_degree, 3);
    }

    #[test]
    fn test_clustering_coefficient() {
        let  graph = read_graph_from_file("test_graph.txt");
        let coeff = clustering_coefficient(&graph);
        assert_eq!(coeff, 0.0);
    }
}
