use std::collections::HashMap;

struct Graph {
    edges: Vec<(String, String)>,
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

fn bfs_path<'a>(graph: &'a Graph, orig: &'a str, dest: &'a str) -> Result<Vec<&'a str>, String> {
    if orig == dest {
        return Ok(vec![orig]);
    }

    let mut path: Vec<&str> = Vec::new();
    let mut parents: HashMap<&str, &str> = HashMap::new();
    let mut frontier: Vec<&str> = Vec::new();
    let mut reached: Vec<&str> = Vec::new();

    frontier.push(orig);
    reached.push(orig);

    while let Some(node) = frontier.pop() {
        for child in graph.neighbors(node) {
            if !reached.contains(&&child[..]) {
                reached.push(&child[..]);
                parents.insert(child, node);

                if child == &dest {
                    path.push(&child[..]);
                    let mut parent = Some(node);
                    while let Some(p) = parent {
                        path.push(p);
                        parent = parents.get(p).copied();
                    }
                    path.reverse();
                    return Ok(path);
                }
                frontier.push(&child[..]);
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

    match bfs_path(&graph, "a", "e") {
        Ok(path) => println!("{:?}", path),
        Err(msg) => println!("{}", msg),
    }
}
