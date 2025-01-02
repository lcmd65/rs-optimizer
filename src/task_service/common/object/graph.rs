use std::collections::{HashMap, HashSet};


struct Graph {
    adjacency_list: HashMap<i32, HashSet<i32>>,
}

impl Graph {
    fn new() -> Self{
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: i32, v: i32) {
        self.adjacency_list.entry(u).or_insert_with(HashSet::new).insert(v);
        self.adjacency_list.entry(v).or_insert_with(HashSet::new).insert(u);
    }

    fn display(&self) {
        for (node, neighbors) in &self.adjacency_list {
            println!("{} -> {:?}", node, neighbors);
        }
    }
}