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

fn dijkstra<'a>(graph: &'a Graph, source: &'a str) -> Result<HashMap<&'a str, u32>, String> {
    let mut dict_distances: HashMap<&str, u32> = HashMap::new();
    dict_distances.insert(source, 0);

    let mut to_check_queue: Vec<String> = vec![source.to_string()];
    to_check_queue.extend(graph.nodes().into_iter().filter(|s| s.as_str() != source).map(|s| s.to_string()));
    to_check_queue.reverse();

    while let Some(node) = min_pop(&mut to_check_queue,&dict_distances) {
        for child in graph.neighbors(&node[..]) {
            if !to_check_queue.contains(child){
                continue;
            } 
            let dist = match dict_distances.get(&child[..]) {
                Some(curr_dist) => {
                    if dict_distances[&node[..]] + graph.edge_from(&node[..], &child[..]).unwrap_or(&u32::MAX) < *curr_dist {
                        dict_distances[&node[..]] + graph.edge_from(&node[..], &child[..]).unwrap_or(&u32::MAX)
                    } else {
                        *curr_dist
                    }
                }
                None => dict_distances[&node[..]] + graph.edge_from(&node[..], &child[..]).unwrap_or(&u32::MAX),
            };
            dict_distances.insert(&child[..], dist);
        }
    }

    

    Ok(dict_distances)
}

fn dijkstra_path<'a>(graph: &'a Graph, source: &'a str) -> Result<HashMap<&'a str, Vec<&'a str>>, String> {
    let mut dict_distances: HashMap<&str, u32> = HashMap::new();
    let mut dict_prev: HashMap<&str, &str> = HashMap::new();

    dict_distances.insert(source, 0);

    let mut to_check_queue: Vec<String> = vec![source.to_string()];
    to_check_queue.extend(graph.nodes().into_iter().filter(|s| s.as_str() != source).map(|s| s.to_string()));
    to_check_queue.reverse();

    while let Some(node) = min_pop(&mut to_check_queue,&dict_distances) {
        for child in graph.neighbors(&node[..]) {
            if !to_check_queue.contains(child){
                continue;
            } 
            let dist = match dict_distances.get(&child[..]) {
                Some(curr_dist) => {
                    if dict_distances[&node[..]] + graph.edge_from(&node[..], &child[..]).unwrap_or(&u32::MAX) < *curr_dist {
                        dict_distances[&node[..]] + graph.edge_from(&node[..], &child[..]).unwrap_or(&u32::MAX)
                    } else {
                        *curr_dist
                    }
                }
                None => dict_distances[&node[..]] + graph.edge_from(&node[..], &child[..]).unwrap_or(&u32::MAX),
            };
            dict_distances.insert(&child[..], dist);
            dict_prev.insert(&child[..],&node[..]);
        }
    }

    let mut dict_paths: HashMap<&str,Vec<&str>> = HashMap::new();
    for node in graph.nodes().iter(){
        let mut temp_vec: Vec<&str> = vec![&node[..]];
        if let mut parent: &str = dict_prev.get();
        temp_vec.push(parent);
        while parent{
            
            parent = dict_prev[parent]
        }

        temp_vec.reverse();
        dict_paths.insert(node,temp_vec.clone());
        
    }

    
    Ok(dict_distances)
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
        Some(min_node)
    } else {
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
    .add_w_edge("c", "g", 1)
    .add_w_edge("d", "h", 6)
    .add_w_edge("e", "i", 5)
    .add_w_edge("a", "j", 7);

    match dijkstra(&graph, "a") {
        Ok(distance_map) => println!("{:?}", distance_map),
        Err(msg) => println!("{}", msg),
    }

}