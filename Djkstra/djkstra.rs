use std::collections::HashMap;

struct Graph {
    edges: Vec<(String, String)>,
    weights: HashMap<(&str, &str),u32>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            edges: Vec::new(),
        }
    }

    fn add_node(mut self, node: &str) -> Graph {
        self.edges.push((node.to_string(), node.to_string()));
        self
    }

    fn add_edge(mut self, node1: &str, node2: &str) -> Graph {
        // Use tuple destructuring without references to avoid moving out of a shared reference
        if !self.edges.iter().any(|(n1, n2)| (n1 == node1 && n2 == node2) || (n1 == node2 && n2 == node1)) {
            self.edges.push((node1.to_string(), node2.to_string()));
        }
        self
    }

    fn neighbors(&self, node: &str) -> Vec<&String> {
        self.edges
            .iter()
            .filter(|(n1, n2)| (n1 == node && n1 != n2) || (n2 == node && n1 != n2))
            .map(|(_, n2)| n2)
            .collect()
    }
}