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
    
pub fn take_all_but_last_two_take<T: std::clone::Clone>(vec: &Vec<T>) -> Vec<T> {
        vec[..vec.len() - 1].to_vec()
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