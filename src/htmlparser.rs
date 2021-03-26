use crate::dom::{AttrMap,ElementData,Node,NodeType};


use std::iter::Peekable;
use std::str::Chars;

pub struct HtmlParser<'a>{
    chars:Peekable<Chars<'a>>,
    node_q:Vec<String>,
}
impl<'a> HtmlParser<'a>{
    pub fn new(full_html:&str)-> HtmlParser{
        HtmlParser{
            chars:full_html.chars().peekable(),
            node_q:Vec::new(),
        }
    }
    //Function to parse Nodes from the String HTML File
    pub fn parse_nodes(&mut self)->Vec<Node>{
        let mut nodes=Vec::new();
        //Check if a character is there then loop
        while self.chars.peek().is_some(){
            self.parse_until_whitespace();
            if self.check_current_char_condition(false, |c| *c== '<'){
                self.chars.next();
                if self.check_current_char_condition(false,|c| *c == '/'){
                    self.chars.next();
                    self.parse_until_whitespace();

                    let close_tag_name=self.consume_while(is_valid_tag_name);
                    
                    self.consume_while(|x| x!= '>');
                    self.chars.next();

                    self.node_q.push(close_tag_name);
                    break;
                }
                //Current Node is a comment 
                else if self.chars.peek().map_or(false,|c| *c != '!'){
                    self.chars.next();
                    nodes.push(self.get_comment_node());
                }
                else{
                    let mut node=self.parse_node();
                    let insertIndex=nodes.len();
                    match node.node_type{
                        NodeType::Element(ref e)=> if self.node_q.len() > 0{
                            let assumed_tag=self.node_q.remove(0);
                            if e.tag_name != assumed_tag{
                                nodes.append(&mut node.children);
                                self.node_q.insert(0,assumed_tag);
                            }
                        },
                        _=>{}
                    }
                    nodes.insert(insertIndex,node)

                }

            } else{
                nodes.push(self.get_text_node())
            }
        }
        nodes
    }

    pub fn parse_node(&mut self)->Node{
        let tagname=self.consume_while(is_valid_tag_name);
        let attributes=self.parse_attributes();

        let element=ElementData::new(tagname, attributes);
        let children=self.parse_nodes();
        Node::new(NodeType::Element(element), children)
    }

    fn get_text_node(&mut self)->Node{
        let mut text_content=String::new();

        while self.check_current_char_condition(false,|c| *c != '<'){
            let word=self.parse_until_whitespace();
            if word.len() > 0{
                text_content.push(' ');
            }
            let text_part=self.consume_while(|x| !x.is_whitespace() && x!= '<');
            text_content.push_str(&text_part);
        }
        Node::new(NodeType::Text(text_content), Vec::new())
    }

    fn get_comment_node(&mut self)->Node{
        let mut comment=String::new();
        if self.check_current_char_condition(false,|c| *c == '-'){
            self.chars.next();
            if self.check_current_char_condition(false,|c| *c == '-') {
                self.chars.next();
            }
            else{
                self.consume_while(|c| c != '>');
                return Node::new(NodeType::Comment(comment), Vec::new())
            }
        }
        else{
            self.consume_while(|c| c != '>');
            return Node::new(NodeType::Comment(comment), Vec::new())
        }
        if self.check_current_char_condition(false,|c| *c == '>'){
            self.chars.next();
            return Node::new(NodeType::Comment(comment), Vec::new());
        }
        if self.check_current_char_condition(false, |c| *c == '-') {
            self.chars.next();
            if self.check_current_char_condition(false, |c| *c == '>') {
                self.chars.next();
                return Node::new(NodeType::Comment(comment), Vec::new());
            } else {
                comment.push('-');
            }
        }
        while self.chars.peek().is_some(){
            comment.push_str(&self.consume_while(|c| c!= '<' && c!='-'));
            if self.check_current_char_condition(false,|c| *c == '<'){
                self.chars.next();
                if self.check_current_char_condition(false,|c| *c == '!'){
                    self.chars.next();
                    if self.check_current_char_condition(false,|c| *c == '-'){
                        self.chars.next();
                        if self.check_current_char_condition(false,|c| *c == '-'){
                            self.consume_while(|c| c != '>');
                            return Node::new(NodeType::Comment(String::from("")), Vec::new());
            }
            else{
                comment.push_str("<!-");
            }
        }else if self.check_current_char_condition(false,|c| *c == ' '){
            self.chars.next();
            if self.check_current_char_condition(false,|c| *c == '-'){
                self.chars.next();
                if self.check_current_char_condition(false,|c| *c == '-'){
                    self.chars.next();
                    if self.check_current_char_condition(false,|c| *c == '-'){
                        self.chars.next();
                        if self.check_current_char_condition(false,|c| *c == '>'){
                            self.chars.next();
                            return Node::new(NodeType::Comment(String::from("")), Vec::new());
                        }
                        else{
                            comment.push_str("<! --");
                        }
                    }
                    else{
                        comment.push_str("<! -");
                        }
                }
                else{
                    comment.push_str("<! ");
                }
            }
        }
        else{
                comment.push_str("<!");
            }
                }
                else{
                    comment.push('<');
                }
            }
            else if self.check_current_char_condition(false,|c| *c == '-'){
                self.chars.next();
                if self.check_current_char_condition(false,|c| *c == '-'){
                    self.chars.next();
                    if self.check_current_char_condition(false,|c| *c == '>'){
                        self.chars.next();
                        break;
            }else{
                comment.push_str("--");
            }
        }else{
            comment.push('-');
        }
    }
}
    Node::new(NodeType::Comment(comment), Vec::new())
}

    fn consume_while<F>(&mut self,condition:F)->String where F: Fn(char)->bool,{
        let mut result=String::new();
        while self.chars.peek().map_or(false,|c| condition(*c)){
            result.push(self.chars.next().unwrap());
        }
        result

    }

    //Parse the attributes of Element Type in the Attribute Type
    fn parse_attributes(&mut self)->AttrMap{
        let mut attributes=AttrMap::new();
        while  self.check_current_char_condition(false,|c| *c == '='){
            self.parse_until_whitespace();
            let name=self.consume_while(|c| is_valid_attr_name(c)).to_lowercase();
            self.parse_until_whitespace();
            let value=if self.check_current_char_condition(false,|c| *c == '='){
                self.chars.next();
                self.parse_until_whitespace();
                let s=self.parse_attr_value();
                self.consume_while(|c| !c.is_whitespace() && c!= '>');
                self.parse_until_whitespace();
                s
            }
            else
            {
                "".to_string()
    
            };
            attributes.insert(name, value);
        }
        self.chars.next();
        attributes
        }
        //arse the value of an attribute
    fn parse_attr_value(&mut self)->String{
            self.parse_until_whitespace();
            let result=match self.chars.peek(){
                Some(&c) if c=='"' || c== '\'' =>{
                    self.chars.next();
                    let ret=self.consume_while(|x| x!=c);
                    self.chars.next();
                    ret
                }
                _=>self.consume_while(is_valid_attr_value)
            };
            result
        }

        fn parse_until_whitespace(&mut self)->String{
            self.consume_while(char::is_whitespace)
        }
        fn check_current_char_condition<V,F>(&mut self,default_value:V,function:F)->V where F: Fn(&char)->V{
            self.chars.peek().map_or(default_value,function) 
        
        } 

}
fn is_control(ch:char)->bool{
    match ch{
        '\u{007F}'=>true,
        ch if ch>= '\u{0000}' && ch<= '\u{001F}' =>true,
        ch if ch>= '\u{0080}' && ch<= '\u{009F}' =>true,
        _=>false,
    }
}
fn is_valid_tag_name(ch:char)->bool{
    ch.is_digit(36)
}
fn is_valid_attr_name(ch:char)->bool{
    !is_excluded_name(ch) && !is_control(ch)
}
fn is_excluded_name(ch:char)->bool{
    match ch{
        ' '| '"' | '\'' | '>' | '/' | '=' =>true,
        _=>false, 
    }
}
fn is_valid_attr_value(ch:char)->bool{
    match ch{
        ' '| '"' | '\'' | '=' | '<' | '>' | '`' =>false,
        _=>true, 
    }
}
