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
            .flat_map(|(n1, n2)| {
                if n1 == node {
                    Some(n2)
                } else {
                    Some(n1)
                }
            })
            .collect()
    }

    fn edge_from(&self, from: &str, to: &str) -> Option<&u32> {
        match self.weights.get(&(from.to_string(), to.to_string())) {
            Some(weight) => Some(weight),
            None => self.weights.get(&(to.to_string(), from.to_string())),
        }
    }
    

    fn nodes(&self) -> Vec<String> {
        self.nodes.clone()
    }
}

fn rand_int(min:u32, max: u32) -> u32 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => (duration.as_nanos() as u32 % (max-min)) + min,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn create_heuristic(graph: &Graph, dest: &str) -> HashMap<String,u32>{
    let mut h: HashMap<String,u32> = HashMap::new();

    for node in &graph.nodes(){
        h.insert(node.clone(),rand_int(20,50));
    }
    h.insert(dest.to_string(),0);
    
    h
}

fn beam_search(graph: &Graph, origin: &str, destination: &str, beam_width: usize) -> Result<(Vec<String>, u32), String> {
    let h = create_heuristic(graph, destination);

    let mut beam = vec![(origin.to_string(), vec![origin.to_string()], 0)];

    while !beam.is_empty() {
        let mut new_beam = Vec::new();

        for (node, path, cost) in beam {
            if node == destination {
                return Ok((path, cost));
            }

            for neighbor in graph.neighbors(&node) {
                let new_path = [&path[..], &[neighbor.clone()]].concat();
                let new_cost = cost + *graph.edge_from(&node, neighbor).expect("edge not found in graph") + h[&neighbor[..]];

                new_beam.push((neighbor.clone(), new_path, new_cost));
            }
        }

        new_beam.sort_by_key(|&(_, _, cost)| cost);
        beam = new_beam.into_iter().take(beam_width).collect();
    }

    Err("Path not found".to_string())
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
    .add_w_edge("c", "i", 1)
    .add_w_edge("d", "h", 6)
    .add_w_edge("e", "i", 5)
    .add_w_edge("a", "j", 7);

    match beam_search(&graph, "a","i",3) {
        Ok(path_cost) => println!("{:?}", path_cost),
        Err(msg) => println!("{}", msg),
    }

}