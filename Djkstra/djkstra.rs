use std::collections::HashMap;

struct Graph {
    nodes: Vec<String>,
    edges: Vec<(String, String)>,
    weights: HashMap<(&str, &str),u32>,
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

    fn add_edge(mut self, node1: &str, node2: &str) -> Graph {
        // Use tuple destructuring without references to avoid moving out of a shared reference
        if !self.edges.iter().any(|(n1, n2)| (n1 == node1 && n2 == node2) || (n1 == node2 && n2 == node1)) {
            self.edges.push((node1.to_string(), node2.to_string()));
            self.weights.insert((node1.to_string(), node2.to_string()),1);
        }
        self
    }

    fn add_w_edge(mut self, node1: &str, node2: &str, w: u32) -> Graph {
        // Use tuple destructuring without references to avoid moving out of a shared reference
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

    fn nodes(&self) -> &Vec<String> {
        &self.nodes
    }
}

fn dijkstra<'a>(graph: &'a Graph, source: &'a str) -> Result<Vec<(&'a str, u32)>, String> {
    
}

fn main(){
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
    .add_weighted_edge("a".to_string(), "b".to_string(), 5)
    .add_weighted_edge("a".to_string(), "c".to_string(), 8)
    .add_weighted_edge("b".to_string(), "d".to_string(), 10)
    .add_weighted_edge("b".to_string(), "e".to_string(), 15)
    .add_weighted_edge("c".to_string(), "f".to_string(), 12)
    .add_weighted_edge("c".to_string(), "g".to_string(), 7)
    .add_weighted_edge("d".to_string(), "h".to_string(), 4)
    .add_weighted_edge("e".to_string(), "i".to_string(), 9)
    .add_weighted_edge("f".to_string(), "j".to_string(), 6)
    .add_weighted_edge("g".to_string(), "j".to_string(), 3)
    .add_weighted_edge("c".to_string(), "d".to_string(), 8)
    .add_weighted_edge("i".to_string(), "e".to_string(), 5)
    .add_weighted_edge("h".to_string(), "f".to_string(), 2)
    .add_weighted_edge("c".to_string(), "g".to_string(), 1)
    .add_weighted_edge("d".to_string(), "h".to_string(), 6)
    .add_weighted_edge("e".to_string(), "i".to_string(), 5)
    .add_weighted_edge("a".to_string(), "j".to_string(), 7);

    // for _ in 0..10 {
    //     let nodes = graph.nodes();
    //     let node1 = nodes[rand::random::<usize>() % 10];
    //     let node2 = nodes[rand::random::<usize>() % 10];
    //     let weight = rand::random::<u32>() % 20 + 1;
    //     graph.add_weighted_edge(node1.to_string(), node2.to_string(), weight);
    // }

    match bfs_path(&graph, "a", "e") {
        Ok(path) => println!("{:?}", path),
        Err(msg) => println!("{}", msg),
    }
}