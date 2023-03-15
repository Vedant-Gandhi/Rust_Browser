extern crate Browser;
use Browser::{
    csselements, cssparser, dom, htmlparser, layouttree, screencommanddisplay, screenrender,
    styletree,
};

use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let html = get_html();

    let ref root_node = html[0];

    let css = get_css();

    let styletree_root = styletree::StyleNode::new(&root_node, &css);

    let mut viewport = layouttree::Dimensions::default();
    viewport.coordinates.width = 1024.0;
    viewport.coordinates.height = 768.0;

    let layouttree = layouttree::get_layout_tree(&styletree_root, viewport);

    let display_coommands = screencommanddisplay::get_display_command_list(&layouttree);
    println!("Display Commands:{:?}", display_coommands);
    screenrender::render_loop(&display_coommands);
}

fn get_html() -> Vec<dom::Node> {
    let mut currentpath = env::current_dir().unwrap();
    currentpath.push("websrc/index.html");
    let mut file_reader = match File::open(&currentpath) {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("file:{},error:{}", currentpath.display(), e),
    };

    let mut html_input = String::new();
    file_reader.read_to_string(&mut html_input).unwrap();

    let nodes = htmlparser::HtmlParser::new(&html_input).parse_nodes();

    nodes
}
fn get_css() -> csselements::Stylesheet {
    let mut currentpath = env::current_dir().unwrap();
    currentpath.push("websrc/index.css");
    let mut file_reader = match File::open(&currentpath) {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("file:{},error:{}", currentpath.display(), e),
    };
    let mut css_input = String::new();
    file_reader.read_to_string(&mut css_input).unwrap();

    let css_stylesheet = cssparser::cssparser::new(&css_input).parse_stylesheet();
    css_stylesheet
}
