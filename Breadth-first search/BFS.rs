struct Node {
    content: String,
    parent: Option<Box<Node>>,
    children: Vec<Node>,
    children_w: Vec<i32>,
}

impl Node {
    fn new(value: &str) -> Node {
        Node {
            content: value.to_string(),
            parent: None,              
            children: Vec::new(),
            children_w: Vec::new(),
        }
    }
}

struct Grahp{
    nodes: Vec<Node>,
    empty: bool
}

impl Graph{
    
}

fn main(){
    
}