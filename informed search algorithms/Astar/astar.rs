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

fn astar<'a>(graph: &'a Graph, source: &'a str, dest: &'a str) -> Result<(Vec<String>, u32), String>  {

    let h = create_heuristic(graph,dest);

    let mut dict_distances: HashMap<&str, u32> = HashMap::new();
    let mut dict_prev: HashMap<String, String> = HashMap::new();
    let mut dict_predicts: HashMap<&str, u32> = HashMap::new();

    dict_distances.insert(source, 0);
    dict_predicts.insert(source, (0 as f32 + 1.3 * h[source] as f32).floor() as u32);

    let mut to_check_queue: Vec<String> = vec![source.to_string()];

    while let Some(node) = min_pop(&mut to_check_queue, &dict_predicts) {

        if &node[..] == dest{
            let mut temp_vec: Vec<String> = vec![dest.to_string()];

            let mut current_node = dest.to_string();
            while let Some(parent) = dict_prev.get(&current_node) {
                temp_vec.push(parent.clone());
                current_node = parent.to_string();
            }
    
            temp_vec.reverse();
    
            return Ok((temp_vec, dict_distances[dest]));
        }

        //println!("Current neighbors of {} are {:?}",node,graph.neighbors(&node[..]));

        for child in graph.neighbors(&node[..]) {
            //println!("Calculating for nodes {} and {}",child,node);
            let temp_dist = dict_distances[&node[..]] + graph.edge_from(&node[..], &child[..]).expect("edge not found in graph!");
            if temp_dist < *dict_distances.get(&child[..]).unwrap_or(&u32::MAX){
                dict_distances.insert(&child[..], temp_dist);
                dict_prev.insert(child.clone(), node.clone());
                dict_predicts.insert(&child[..], (dict_distances[&node[..]] as f32 + 1.3 * h[&child[..]] as f32).floor() as u32);
                to_check_queue.push(child.clone());
            }
        }
    }
    
    Err("Path not found".to_string())
}



fn min_pop(q: &mut Vec<String>, dict: &HashMap<&str, u32>) -> Option<String> {
    if q.is_empty() {
        return None;
    }
    
    let mut min_value = u32::MAX;
    let mut min_node = String::new();
    
    for node in &*q {
        if let Some(&temp_dist) = dict.get(node.as_str()) {
            if temp_dist < min_value {
                min_value = temp_dist;
                min_node = node.clone();
            }
        }
    }
    
    if let Some(index) = q.iter().position(|x| x == &min_node) {
        q.remove(index);
        println!("popped {}",min_node);
        Some(min_node)
    } else {
        println!("popped nothing");
        None
    }

    
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

    match astar(&graph, "a","j") {
        Ok(path_cost) => println!("{:?}", path_cost),
        Err(msg) => println!("{}", msg),
    }

}