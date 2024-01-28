#![warn(unused_variables)]
#![warn(non_snake_case)]
mod helper_functions;
use crate::helper_functions::*;
mod structures;
use crate::structures::all_structures::*;

use std::collections::HashMap;


fn enginneer(lexer_string: &Vec<&str>) -> () {

    let mut vec_operations:Vec<&str> = vec!["x","/","+","-"];
    vec_operations.sort();
    let vec_operations = vec_operations;


    if lexer_string.len() > 2 {
        println!("ENGINEER STARTS");// testing code execution
        println!("LENGTH ---- : {}",lexer_string.len());
         // *****************************************************
         // send two last token to node_leaf_builder
         // helper function : create HashMap <material, &Str>
         let hm = create_hashmap_to_builder(&lexer_string[lexer_string.len() - 2 ..]);
         node_leaf_builder(&hm);
        
        enginneer(&helper_functions::take_all_but_last(&lexer_string));
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
    // match case on material
    let type_material = material.keys().next().unwrap();

    match type_material {
        &Material::numeric => println!(".........build right leaf"), //Leaf::Numeric(material.get(&Material::numeric).unwrap().to_string()),
        &Material::variable => println!("build left leaf .........."), //Leaf::Variable(material.get(&Material::variable).unwrap().to_string()),
        &Material::operation => println!("......build Node.........")
    };


    // if material.keys().next().unwrap() == &Material::numeric {
    // println!("Numeric = {:?}", material.get(&Material::numeric).unwrap());
    // return Leaf::Numeric(material.get(&Material::numeric).unwrap().to_string());
    // } else  if material.keys().next().unwrap() == &Material::variable {
    //     println!("Variable key = {:?}", material.get(&Material::variable).unwrap());
    //     return Leaf::Variable(material.get(&Material::variable).unwrap().to_string());
    //     }
    // else {
    //     println!("no leaf");
    //     return Leaf::NoLeaf
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
    enginneer(&test_string);
   
}