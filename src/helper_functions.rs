
pub use crate::structures::all_structures;

use std::collections::HashMap;

const OPERATIONS: [&str;4] = ["x","/","+","-"];

pub fn remove_whitespace(s: String) -> String {
    let mut new_s = String::new();
    for c in s.chars() {
        if c != ' ' {
            new_s.push(c);
        }
    }
    new_s
}

fn iterate_string_and_accumulate(s: &str) -> () {

    //let mut results = Vec::new();
    let mut next_c ="\0";
    //reverse the string into a vector of chars
    let reverse_string: Vec<char> = s.chars().rev().collect();
    println!("string:{}, reversed:{:?}", s, reverse_string);
    for element in 0..reverse_string.len() {
        println!("Elemenf: {}",reverse_string[element]);
    }
    }

fn split_vector_by_slicing(vector: Vec<i32>, mut collect: &mut Vec<Vec<i32>>) -> () {
    
    
    if vector.len() == 1 {
        collect.push(vector);
        
    } else {
        let head = vector[..vector.len() - 2].to_vec();
        println!("vector head:{:?}", &head);
        let tails = vector[vector.len() - 2..].to_vec();
        // let clone_tails =  vector[vector.len() - 2..].to_vec();
        collect.push(tails);
        split_vector_by_slicing(head, collect);
        
    }
        
    }
    
pub fn take_all_but_last<T: std::clone::Clone>(vec: &Vec<T>) -> Vec<T> {
        vec[..vec.len() - 2].to_vec()
    }


// receive two strings and create a HashMap for the engineer

fn identify_components(component: &str) -> (all_structures::Material, &str) {
    let op = ["x","/","+","-"];

    if op.iter().any(|&i| i == component) {
        (all_structures::Material::operation, component)
    } else {
        (all_structures::Material::numeric, "atom")
    }


}


pub fn create_hashmap_to_builder<'a>(vec_tokens: &'a [&'a str]) -> HashMap<all_structures::Material, &'a str> {

    println!("ENTER HASHMAP to BUILDER vec_token: {:?}", vec_tokens);

    let mut parsed_materials_first_node: HashMap<all_structures::Material, &str> = HashMap::new();
    
   //map over the vec and create a hasmap
    for vec_token in vec_tokens {

        parsed_materials_first_node.insert(identify_components(vec_token).0, identify_components(vec_token).1);
    }
    
    //let opr_first_node:all_structures::Material = all_structures::Material::operation;

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

    //parsed_materials_first_node.insert(opr_first_node, "3");
    return parsed_materials_first_node; 

}


#[test]
fn test_clear_white_space() {
    println!("Testing remove_whitespace function...");

    let original_string = String::from("3 + x");
    let expected_string = String::from("3+x");
    println!{"input: {}, expected = {}, run test....", & original_string, & expected_string};
    let cleaned_string = remove_whitespace(original_string);

    assert_eq!(expected_string, cleaned_string);
   
    println!("remove_whitespace function passed all tests!");

}



#[test]
fn test_simple_lexer() {

    let string_to_lexify = String::from("3*y+x");
    println!("lexified {}, results: {:?}", & string_to_lexify, iterate_string_and_accumulate(& string_to_lexify));
}

#[test]
fn test_splitter() {

    let vector = vec![1,2,3,4,5];
    let mut collector: Vec<Vec<i32>> = vec![]; 
    split_vector_by_slicing(vector, &mut collector);
    
    println!("tail: {:?}", collector)
}
