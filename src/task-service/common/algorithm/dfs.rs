use object::{Graph};
fn dfs(graph: Graph, start: i32, visited: &mut HashMap<i32, bool>) {
    visited.insert(start, true);
    println!("Visited: {}", start);

    if let Some(neighbors) = graph.get(&start) {
        for &neighbor in neighbors {
            if !visited.get(&neighbor).unwrap_or(&false) {
                dfs(graph, neighbor, visited);
            }
        }
    }
}