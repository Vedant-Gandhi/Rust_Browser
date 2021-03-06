use std::collections::HashMap;
use std::fmt;

use crate::dom::{ElementData,Node,NodeType};
use crate::csselements::{Selector,Stylesheet,Value};

type StyleMap<'a>=HashMap<& 'a str, & 'a Value>;
pub struct StyleNode<'a>{
     node: & 'a Node,
     styles:StyleMap<'a>,
     pub children:Vec<StyleNode<'a>>,
}
pub enum Display{
    Block,InlineBlock,Inline,None
}
impl<'a>  StyleNode<'a>{
     pub fn new(node:& 'a Node, stylesheet:& 'a Stylesheet)->StyleNode<'a>{
        let mut style_children=Vec::new();

        for child in &node.children{
            match child.node_type{
                NodeType::Element(_)=>style_children.push(StyleNode::new(&child, stylesheet)),
                _=>{}
            }

        }

        StyleNode{node,styles:match node.node_type{
            NodeType::Element(ref e)=>StyleNode::get_styles(e,stylesheet),
            _=>StyleMap::new()
        },
        children:style_children    
    }

    }
    fn get_styles(e:& 'a ElementData,stylesheet:&'a Stylesheet)->StyleMap<'a>{
        let mut styles=StyleMap::new();
         for single_rule in &stylesheet.css{
             for selector in &single_rule.selectors{
                 if selector_matches(e,&selector){
                     for declarations in &single_rule.declarations{
                         styles.insert(&declarations.key,&declarations.value);
                     }
                     break;
                 }
             }
         }
         styles
    }
 
    pub fn get_value(&self,name:&str)->Option<&&Value>{
        self.styles.get(name)
    }
    pub fn get_display_value(&self)->Display{
        match self.get_value("display"){
            Some(s)=>match **s{
                Value::Other(ref v)=>match v.as_ref(){
                    "block"=>Display::Block,
                    "none"=>Display::None,
                    "inline-block"=>Display::InlineBlock,
                    _=>Display::Inline
                },
                _=>Display::Inline
            }
            None=>Display::Inline
        } 
    }

    pub fn num_or(&self,name:&str,default:f32)->f32{
        match self.get_value(name){
            Some(v)=>match **v{
                Value::Length(n,_)=>n,
                _=>default,
            }
            None=>default
        }
    }
  
   
}
impl <'a> fmt::Debug for StyleNode<'a>{
fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
    write!(f,"{:?}{:?}",self.node,self.styles)
}
}
pub fn pretty_print(node: &StyleNode,indent_size:usize){
    let indent=(0..indent_size).map(|_| " ").collect::<String>();
    println!("{}{:?}",indent,node);

    for child in node.children.iter(){
        pretty_print(&child, indent_size+2);
    }

}

fn selector_matches(element:&ElementData,selector:&Selector)->bool{
    for simple in &selector.simple{
        let mut selector_match=true;

        match simple.tag_name{
            Some(ref t)=>
            if *t != element.tag_name{
                continue;
            },
            None=>{}
        };
        match element.getId(){
            Some(i)=>match simple.id{
                Some (ref id)=> if *i != *id {
                    continue;
                }
                _=>{}
            },
            None=>{},
        }
        let element_classes=element.getClasses();

        for class in &simple.classes{
            selector_match&=element_classes.contains::<str>(class);
        }
        if selector_match{
            return true;
        }
    }
    false
}