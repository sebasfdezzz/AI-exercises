use std::collections::HashMap;
use std::time::SystemTime;

struct Graph {
    nodes: Vec<String>,
    edges: Vec<(String, String)>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn add_node(mut self, node: &str) -> Graph {
        self.edges.push((node.to_string(), node.to_string()));
        self.nodes.push(node.to_string());
        self
    }

    fn add_edge(mut self, node1: &str, node2: &str) -> Graph {
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

    fn nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }
}

fn rand_int(max: u32) -> u32 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos() as u32 % max,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn create_heuristic(graph: &Graph, dest: &str) -> HashMap<String,u32>{
    let mut h: HashMap<String,u32> = HashMap::new();

    for node in &graph.nodes(){
        h.insert(node.clone(),rand_int(50));
    }
    h.insert(dest.to_string(),0);
    h
}

fn greedy_search<'a>(graph: &'a Graph,orig: &'a str, dest: &'a str) -> Result<String, String> {

    let h: HashMap<String,u32> = create_heuristic(graph,dest);

    let mut to_check_queue: Vec<&str> = vec![orig];

    let mut visited: Vec<&str> = Vec::new();

    while let node = to_check_queue.remove(0) {

        if visited.contains(&node){
            if to_check_queue.is_empty(){
                return Err("Not Found".to_string());
            }
            continue;
        }
        println!("{}",node);
        visited.push(node);

        if dest == node{
            return Ok("Success".to_string());
        }

        to_check_queue.extend(graph.neighbors(&node[..]).iter().map(|x| x.as_str()));
        to_check_queue.sort_by_key(|node| h[*node]);
    }

    Err("Not Found".to_string())
}

fn main() {
    let graph = Graph::new()
        .add_node("a")
        .add_node("b")
        .add_node("c")
        .add_node("d")
        .add_node("e")
        .add_edge("a", "c")
        .add_edge("a", "d")
        .add_edge("a", "b")
        .add_edge("c", "d")
        .add_edge("d", "e");

    match greedy_search(&graph,"a", "e") {
        Ok(msg) => println!("Yey {}", msg),
        Err(msg) => println!("Error {}", msg),
    }
}
