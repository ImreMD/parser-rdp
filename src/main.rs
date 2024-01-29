#![allow(warnings)]
mod helper_functions;
use crate::helper_functions::*;
mod structures;
use crate::structures::all_structures::*;

use std::collections::HashMap;


fn engineer(lexer_string: &Vec<&str>) -> () {

    

    if lexer_string.len() > 2 {
        println!("ENGINEER STARTS");// testing code execution
        println!("LENGTH ---- : {}",lexer_string.len());
         // *****************************************************
         // send two last token to node_leaf_builder
         // helper function : create HashMap <material, &Str>
         let hm = create_hashmap_to_builder(&lexer_string[lexer_string.len() - 2 ..]);

         node_leaf_builder(&hm);
        
        engineer(&helper_functions::take_all_but_last(&lexer_string));
        } else {

        println!("ENGINEER EXITS");// testing code execution

        }
  
    
    

}



fn node_leaf_builder(material: &HashMap<Material, &str>)-> () { //leaf<'a>
    
    //VERY PRIMITIVE APPROACH!
    //segregate only operations ("x","/","+","-") for the parsed stream
    println!("*************************************************");
    
    println!("ENTER lEAFBUILDER"); // testing code execution
    println!("Material type {:?}", material.keys().next().unwrap());
    
    println!("*************************************************");
    // *****************************************************
    // match case on 
    for (type_material, value) in material.iter() {
    //let t_material = material.keys().next().unwrap();

    match type_material {
        &Material::numeric => println!(".........build Numeric right leaf"), //Leaf::Numeric(material.get(&Material::numeric).unwrap().to_string()),
        &Material::variable => println!("build left leaf .........."), //Leaf::Variable(material.get(&Material::variable).unwrap().to_string()),
        &Material::operation => println!("......build Operation Node.........")
    };
    }

    
    }


fn node_builder(material: HashMap<Material, String>)-> () {
        
        
        todo!();

//     
//     TODO : node builder code
//     
//     
    
// 
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

let left_node = Box::new(Some(Leaf::Variable("x")));
let right_node = Box::new(Some(Leaf::Numeric("3")));
let node = Node {operation: "+", left: left_node, right: right_node };
println!("node {:?}", node)

}

#[test]
//testing the node_builder
fn test_leaf_builder () {
    
    let mut test_string =  vec!["4","/","18","x","2"];
    println!("START TEST vec LENGTH:------ {} --------", test_string.len());
    //test_materials.insert(Material::operation, "plus".to_string());
    engineer(&test_string);
   
}