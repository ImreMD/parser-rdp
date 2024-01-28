pub mod all_structures {

#[derive(Debug)]
pub enum Leaf<'a> {
    Variable(&'a str),
    Numeric(&'a str),
    NoLeaf

}

#[derive(Debug)]
pub struct Node<'a> {
    pub operation : &'a str,
    pub left : Box<Option<Leaf<'a >>>,
    pub right : Box<Option<Leaf<'a>>>,
}

pub type Precedence = i32;

pub enum Build_Instruction {

    build_left,
    build_right

}


#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
pub enum Material {

    operation,  //build a node
    numeric,    //build a right leaf (by convention right leaf will numeric by default)
    variable    //build a left leaf (!opposite of numeric) 

}
}