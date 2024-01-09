#[derive(Debug)]
struct Node {
    operation : String,
    left : Box<Option<Leaf>>,
    right : Box<Option<Leaf>>
}

#[derive(Debug)]
enum Leaf {
    Variable(String),
    Numeric(i32),
    AnotherNode(Node)    

}

fn main() {
    
}

#[test]
fn build_AST_by_hand() {

    //3x build the AST by hand
    //y=3x
    //  (*)
    //  / \
    // x   3

let left_node = Box::new(Some(Leaf::Variable("x".to_string())));
let right_node = Box::new(Some(Leaf::Numeric(3)));
let node = Node {operation: "+".to_string(),left: left_node, right: right_node };
println!("node {:?}", node)

}