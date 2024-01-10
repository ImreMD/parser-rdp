mod helper_functions;

use std::collections::HashMap;

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

#[derive(Debug)]
struct Node {
    operation : String,
    left : Box<Option<Leaf>>,
    right : Box<Option<Leaf>>
}

type Precedence = i32;

enum Build_Instruction {

    build_left,
    build_right

}
#[derive(PartialEq, Eq, Hash)]
enum Material {

    operation,
    numeric,
    variable

}

type MaterialMap = HashMap<Material, String>;

//fn enginneer(lexer_string: Vec<String>) -> Node {

    // TODO: implement the engineer 
    // engineer will spawn node_builders
    // collect effect of node_builder work
    // build the AST Tree base on information 
    // from each node_builder 

//}

fn node_builder(material: HashMap<Material, String>, instruction: Build_Instruction)-> (Node, Precedence) {

    //TODO: implement a node builder return type (Vec<char>, Precedence)
    // node_builder will receive a Vec<Char> i.e. ["3","+"]
    // and will produce a Node (left or right)
    // node_builder will have an identifier to enable the collection
    // of their respective work by the engineer
    let op = material.get(&Material::operation).unwrap();
    let prec = 4;
    let mut m:Node = Node { operation: "null".to_string(), left: Box::new(None), right: Box::new(None) };
    match instruction {
        Build_Instruction::build_left =>  m = Node { operation: op.to_string(), left: Box::new(Some(Leaf::Variable("+".to_string()))), right: Box::new(None) },
        _ => ()}
    
return (m, prec);
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
