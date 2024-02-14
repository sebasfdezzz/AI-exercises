use std::collections::HashMap;

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

    fn add_edge(mut self, node1: &str, node2: &str) -> Graph {
        if !self.edges.iter().any(|(n1, n2)| (n1 == node1 && n2 == node2) || (n1 == node2 && n2 == node1)) {
            self.edges.push((node1.to_string(), node2.to_string()));
            self.weights.insert((node1.to_string(), node2.to_string()),1);
        }
        self
    }

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

    fn nodes(&self) -> &Vec<&str> {
        &self.nodes.iter().map(|&x| x).collect()
    }
}

fn dijkstra<'a>(graph: &'a Graph, source: &'a str) -> Result<HashMap<&'a str, u32>, String> {
    let mut dict_distances: HashMap<&str, u32> = HashMap::new();
    dict_distances.insert(source, 0);

    let mut to_check_queue: Vec<&str> = graph.nodes().iter().map(|&x| x).collect();

    while let Some(node) = to_check_queue.pop() {
        for child in graph.neighbors(node) {
            let dist = match dict_distances.get(child) {
                Some(curr_dist) => {
                    if dict_distances[node] + graph.edge_from(node, child)? < *curr_dist {
                        dict_distances[node] + graph.edge_from(node, child)?
                    } else {
                        *curr_dist
                    }
                }
                None => dict_distances[node] + graph.edge_from(node, child).unwrap_or(&u32::MAX),
            };
            dict_distances.insert(child, dist);
        }
    }
    Ok(dict_distances)
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

    // for _ in 0..10 {
    //     let nodes = graph.nodes();
    //     let node1 = nodes[rand::random::<usize>() % 10];
    //     let node2 = nodes[rand::random::<usize>() % 10];
    //     let weight = rand::random::<u32>() % 20 + 1;
    //     graph.add_w_edge(node1.to_string(), node2.to_string(), weight);
    // }

    match dijkstra(&graph, "a") {
        Ok(distance_map) => println!("{:?}", distance_map),
        Err(msg) => println!("{}", msg),
    }
}