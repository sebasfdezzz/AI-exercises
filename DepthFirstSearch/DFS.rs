use std::collections::HashMap;
use std::time::SystemTime;

struct Graph {
    nodes: Vec<String>,
    edges: Vec<(String, String)>,
    weights: HashMap<(String, String),u32>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            weights: HashMap::new(),
        }
    }

    fn add_node(mut self, node: &str) -> Graph {
        self.edges.push((node.to_string(), node.to_string()));
        self.nodes.push(node.to_string());
        self
    }

    // fn add_edge(mut self, node1: &str, node2: &str) -> Graph {
    //     if !self.edges.iter().any(|(n1, n2)| (n1 == node1 && n2 == node2) || (n1 == node2 && n2 == node1)) {
    //         self.edges.push((node1.to_string(), node2.to_string()));
    //         self.weights.insert((node1.to_string(), node2.to_string()),1);
    //     }
    //     self
    // }

    fn add_w_edge(mut self, node1: &str, node2: &str, w: u32) -> Graph {
        if !self.edges.iter().any(|(n1, n2)| (n1 == node1 && n2 == node2) || (n1 == node2 && n2 == node1)) {
            self.edges.push((node1.to_string(), node2.to_string()));
            self.weights.insert((node1.to_string(), node2.to_string()),w);
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

    fn edge_from(&self, from: &str, to: &str) -> Option<&u32> {
        self.weights.get(&(from.to_string(), to.to_string()))
    }

    fn nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }
}

fn dfs_path<'a>(graph: &'a Graph, node: &'a str, dest: &'a str, visited: &mut Vec<&str>) -> Result<Vec<&'a str>, String> {

    visited.push(node);

    if node == dest{
        return vec![node];
    }

    if !graph.neighbors(node).is_empty(){
        for child in graph.neighbors(node){
            if !visited.contains(child){
                if let Some(path) = dfs_path(graph,child,dest,visited){
                    Some(vec![node].extend(path))
                }
                
            }
        }
    }
    None
}

fn dfs_path_limited<'a>(graph: &'a Graph, orig: &'a str, dest: &'a str) -> Result<Vec<&'a str>, String> {
    
}

fn dfs_path_rnd<'a>(graph: &'a Graph, orig: &'a str, dest: &'a str) -> Result<Vec<&'a str>, String> {
    
}

fn dfs_path_limited_rnd<'a>(graph: &'a Graph, orig: &'a str, dest: &'a str) -> Result<Vec<&'a str>, String> {
    
}

fn random_index(max: usize) -> usize {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos() as usize % max,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn main() {
    let graph = Graph::new()
    .add_node("a")
    .add_node("b")
    .add_node("c")
    .add_node("d")
    .add_node("e")
    .add_node("f")
    .add_node("g")
    .add_node("h")
    .add_node("i")
    .add_node("j")
    .add_w_edge("a", "b", 5)
    .add_w_edge("a", "c", 8)
    .add_w_edge("b", "d", 10)
    .add_w_edge("b", "e", 15)
    .add_w_edge("c", "f", 12)
    .add_w_edge("c", "g", 7)
    .add_w_edge("d", "h", 4)
    .add_w_edge("e", "i", 9)
    .add_w_edge("f", "j", 6)
    .add_w_edge("g", "j", 3)
    .add_w_edge("c", "d", 8)
    .add_w_edge("i", "e", 5)
    .add_w_edge("h", "f", 2)
    .add_w_edge("c", "g", 1)
    .add_w_edge("d", "h", 6)
    .add_w_edge("e", "i", 5)
    .add_w_edge("a", "j", 7);

    match dfs_path(&graph, "a", "e", &mut Vec::new()) {
        Ok(path) => println!("{:?}", path),
        Err(msg) => println!("{}", msg),
    }
}
