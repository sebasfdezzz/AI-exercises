use std::collections::HashMap;

struct Graph {
    edges: Vec<(String, String)>,
    empty: bool,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            edges: Vec::new(),
            empty: true,
        }
    }

    fn add_node(mut self, node: &str) -> Graph {
        self.edges.push((node.to_string(), node.to_string()));
        self.empty = false;
        self
    }

    fn add_edge(mut self, node1: &str, node2: &str) -> Graph {
        if !self.edges.iter().any(|&(n1, n2)| (n1 == node1 && n2 == node1) || (n1 == node2 && n2 == node2)) {
            self.edges.push((node1.to_string(), node2.to_string()));
        }
        self
    }

    fn neighbors(&self, node: &str) -> Vec<&String> {
        self.edges
            .iter()
            .filter_map(|&(ref n1, ref n2)| if n1 == node && n1 != n2 { Some(n2) } else { None })
            .collect()
    }
}

fn BFSpath(g: &Graph, orig: &str, dest: &str) -> Result<Vec<&str>, String> {
    let mut path: Vec<&str> = Vec::new();
    let mut parents: HashMap<&str, &str> = HashMap::new();

    if orig == dest {
        return Ok(vec![orig]);
    }

    let mut frontier: Vec<&str> = Vec::new();
    frontier.push(orig);
    let mut reached: Vec<&str> = Vec::new();
    reached.push(orig);

    while !frontier.is_empty() {
        if let Some(node) = frontier.pop() {
            for child in g.neighbors(node) {
                if !reached.contains(child) {
                    reached.push(child);

                    if !parents.contains_key(child) {
                        parents.insert(child, node);
                    }

                    if child == &dest {
                        path.push(dest);
                        let mut parent = Some(node);
                        while let Some(p) = parent {
                            path.push(p);
                            parent = parents.get(p).copied();
                        }
                        path.reverse();
                        return Ok(path);
                    }
                }
            }
        }
    }
    Err(String::from("Nodes or path are non-existent"))
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

    match BFSpath(&graph, "a", "e") {
        Ok(path) => println!("{:?}", path),
        Err(msg) => println!("{}", msg),
    }
}
