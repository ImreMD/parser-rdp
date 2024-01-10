#[derive(Debug)]
struct Node {
    operation : String,
    left : Box<Option<Leaf>>,
    right : Box<Option<Leaf>>
}

type Precendence = Int;


fn enginneer(lexer_string: String) -> Node {

    // TODO: implement the engineer 
    // engineer will spawn node_builders
    // collect effect of node_builder work
    // build the AST Tree base on information 
    // from each node_builder 

}

fn node_builder(material: String)-> (Vec<Char>, Precedence) {

    //TODO: implement a node builder 
    // node_builder will receive a Vec<Char> i.e. ["3","+"]
    // and will produce a Node (left or right)
    // node_builder will have an identifier to enable the collection
    // of their respective work by the engineer
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
