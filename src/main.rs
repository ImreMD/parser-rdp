#![warn(unused_variables)]
#![warn(non_snake_case)]
mod helper_functions;

use std::collections::HashMap;

#[derive(Debug)]
enum Leaf {
    Variable(String),
    Numeric(String),
    NoLeaf

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
#[derive(Debug)]
enum Material {

    operation,
    numeric,
    variable

}

type MaterialMap = HashMap<Material, String>;

fn enginneer(lexer_string: &Vec<&str>) -> () {

    let mut vec_operations:Vec<&str> = vec!["x","/","+","-"];
    vec_operations.sort();
    let vec_operations = vec_operations;

    let mut parsed_materials_first_node: HashMap<Material, &str> = HashMap::new();
    
    let str_last = &lexer_string[lexer_string.len()-1];
   
    let mut opr_first_node:Material;

   println!("ENGINEER STARTS");// testing code execution
   println!("LENGTH ---- : {}",lexer_string.len());
    // *****************************************************
   
   
    if let Ok(contents) = vec_operations.binary_search(str_last) {
        println!("CHECK X CLAUSE");  // testing code execution
         // *****************************************************
            let opr_first_node:Material = Material::variable;
            parsed_materials_first_node.insert(opr_first_node, str_last);
            leaf_builder(&parsed_materials_first_node);

        } else {

            println!("CHECK 2 CLAUSE");  // testing code execution
         // *****************************************************
            let opr_first_node:Material = Material::numeric;
            parsed_materials_first_node.insert(opr_first_node, str_last);
            leaf_builder(&parsed_materials_first_node);
 
        };
        
   
    if lexer_string.len() > 1 {
    enginneer(&helper_functions::take_all_but_last_two_take(&lexer_string));
    }
    

}



fn leaf_builder(material: &HashMap<Material, &str>)-> Leaf {
    
    //VERY PRIMITIVE APPROACH!
    //segregate only operations ("x","/","+","-") for the parsed stream
    println!("ENTER lEAFBUILDER"); // testing code execution
    
    // *****************************************************
    println!("Material type {:?}", material.keys().next().unwrap());

    if material.keys().next().unwrap() == &Material::numeric {
    println!("Numeric = {:?}", material.get(&Material::numeric).unwrap());
    return Leaf::Numeric(material.get(&Material::numeric).unwrap().to_string());
    } else  if material.keys().next().unwrap() == &Material::variable {
        println!("Variable key = {:?}", material.get(&Material::variable).unwrap());
        return Leaf::Variable(material.get(&Material::variable).unwrap().to_string());
        }
    else {
        println!("no leaf");
        return Leaf::NoLeaf
    }
}

// fn node_builder(material: HashMap<Material, String>)-> Node {
//     

//     
//     TODO : node builder code
//     
//     
    
// 
// }


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
let right_node = Box::new(Some(Leaf::Numeric("3".to_string())));
let node = Node {operation: "+".to_string(),left: left_node, right: right_node };
println!("node {:?}", node)

}

#[test]
//testing the node_builder
fn test_leaf_builder () {
    
    let mut test_string =  vec!["4","/","18","x","2"];
    println!("START TEST vec LENGTH:------ {} --------", test_string.len());
    //test_materials.insert(Material::operation, "plus".to_string());
    enginneer(&test_string);
   
}