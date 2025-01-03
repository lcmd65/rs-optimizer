use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::{HashMap, HashSet};

/// Hàm DFS bằng Rust
#[pyfunction]
fn dfs(graph: HashMap<String, Vec<String>>, start: String) -> Vec<String> {
    let mut visited = HashSet::new();
    let mut stack = vec![start.clone()];
    let mut result = vec![];

    while let Some(node) = stack.pop() {
        if !visited.contains(&node) {
            visited.insert(node.clone());
            result.push(node.clone());
            if let Some(neighbors) = graph.get(&node) {
                for neighbor in neighbors.iter().rev() {
                    stack.push(neighbor.clone());
                }
            }
        }
    }

    result
}

/// Định nghĩa module Python
#[pymodule]
fn dfs_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dfs, m)?)?;
    Ok(())
}
