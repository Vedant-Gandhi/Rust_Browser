//Node Types-Element,Text,Comment
use std::collections::{HashMap,HashSet};
use std::fmt;

pub type AttrMap=HashMap<String,String>;
//Defines a basic Node
#[derive(PartialEq,Eq)]
pub struct Node{
   pub children:Vec<Node>,
   pub node_type:NodeType,
}

#[warn(dead_code)] #[derive(PartialEq,Eq,Clone)] 
pub enum NodeType{
    Text(String),
    Element(ElementData),
    Comment(String),

}
#[derive(PartialEq,Eq,Clone)]
pub struct ElementData{
   pub tag_name:String,
    pub attributes:AttrMap,
}
impl Node{
    pub fn new(node_type:NodeType,children:Vec<Node>)->Node{
        Node{
            node_type,children
        }
    }
} 
 impl ElementData{
    pub fn new(tag_name:String,attributes:AttrMap)-> ElementData{
        ElementData{tag_name,attributes}
    }
    pub fn getId(&self)->Option<&String>{
        self.attributes.get("id")
    }
    pub fn getClasses(&self)->HashSet<&str>{
        match self.attributes.get("classes"){
            Some(s)=>s.split(' ').collect(),
            None=>HashSet::new(),
        }

    }
}



 impl fmt::Debug for NodeType{
     fn fmt(&self,f: &mut fmt::Formatter)->fmt::Result{
        match *self{
            NodeType::Text(ref t) | NodeType::Comment(ref t)=>write!(f,"{}",t),
            NodeType::Element(ref e)=>write!(f,"{:?}",e),
        }
    }
}
 impl fmt::Debug for Node{
     fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
    write!(f,"{:?} {:?}",self.children,self.node_type)
}
}
 impl fmt::Debug for ElementData{
     fn fmt(&self,f: &mut fmt::Formatter)->fmt::Result{
        let mut attr_String=String::new();
        for (attr,value) in self.attributes.iter() {
            attr_String.push_str(&format!("{}=\"{}\"",attr,value))
        }
        write!(f,"<{},{}>",self.tag_name,attr_String)
        
    }
}

pub fn pretty_print(n:&Node,indent_size:usize){
    let indent=(0..indent_size).map(|_| " ").collect::<String>();
    match n.node_type{
        NodeType::Element(ref e)=>println!("{}{:?}",indent,e),
        NodeType::Text(ref t)=>println!("{}{}",indent,t),
        NodeType::Comment(ref c)=>println!("{}<--!{}-->",indent,c),
    }
    for child in n.children.iter(){
        pretty_print(&child, indent_size+2)
    }
    match n.node_type{
        NodeType::Element(ref e)=> println!("{}</{}>",indent,e.tag_name),
        _=>{}
    }
}