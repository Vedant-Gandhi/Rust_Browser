use std::fmt;

use crate::csselements::{Unit, Value};
use crate::styletree::{Display, StyleNode};

#[derive(Clone)]
pub struct LayoutContainer<'a> {
    pub dims: Dimensions,
    boxtype: BoxType,
    pub style_node: &'a StyleNode<'a>,
    pub children_nodes: Vec<LayoutContainer<'a>>,
}
#[derive(Clone, Copy, Default)]
pub struct Dimensions {
    pub coordinates: Rectangle,
    padding: EdgeValues,
    pub border: EdgeValues,
    margin: EdgeValues,
    current: Rectangle,
}
#[derive(Clone, Copy, Default)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
#[derive(Clone, Copy, Default)]
pub struct EdgeValues {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}
#[derive(Clone)]
pub enum BoxType {
    Block,
    InlineBlock,
    Inline,
    Anonymous,
}
impl<'a> LayoutContainer<'a> {
    pub fn new(boxtype: BoxType, style_node: &'a StyleNode) -> LayoutContainer<'a> {
        LayoutContainer {
            boxtype,
            dims: Default::default(),
            style_node,
            children_nodes: Vec::new(),
        }
    }
    fn layout(&mut self, dims: Dimensions) {
        match self.boxtype {
            BoxType::Block => self.layout_block(dims),
            BoxType::Inline => self.layout_block(dims),
            BoxType::InlineBlock => self.layout_inline_block(dims),
            BoxType::Anonymous => {}
        }
    }
    fn layout_inline_block(&mut self, dims: Dimensions) {
        self.calculate_inline_horizontal(dims);
        self.calculate_inline_vertical(dims);
        self.layout_children();
        self.calculate_height();
    }
    fn layout_block(&mut self, dims: Dimensions) {
        self.calculate_horizontal(dims);
        self.calculate_vertical(dims);
        self.layout_children();
        self.calculate_height();
    }
    fn calculate_inline_horizontal(&mut self, dims: Dimensions) {
        let s = self.style_node;
        let d = &mut self.dims;

        d.coordinates.width = get_absolute_num(s, dims, "width").unwrap_or(0.0);
        d.margin.left = s.num_or("margin-left", 0.0);
        d.margin.right = s.num_or("margin-right", 0.0);
        d.padding.left = s.num_or("padding-left", 0.0);
        d.padding.right = s.num_or("padding-right", 0.0);
        d.border.left = s.num_or("border-left-width", 0.0);
        d.border.right = s.num_or("border-right-width", 0.0);
    }
    fn calculate_inline_vertical(&mut self, dims: Dimensions) {
        let style = self.style_node;
        let d = &mut self.dims;

        d.margin.top = style.num_or("margin-top", 0.0);
        d.margin.bottom = style.num_or("margin-bottom", 0.0);
        d.border.top = style.num_or("border-top-width", 0.0);
        d.border.bottom = style.num_or("border-bottom-width", 0.0);
        d.padding.top = style.num_or("padding-top", 0.0);
        d.padding.bottom = style.num_or("padding-bottom", 0.0);

        d.coordinates.x =
            dims.coordinates.x + dims.current.x + d.margin.left + d.border.left + d.padding.left;
        d.coordinates.y = dims.coordinates.height
            + dims.coordinates.y
            + d.margin.top
            + d.border.top
            + d.padding.top;
    }

    fn calculate_horizontal(&mut self, dims: Dimensions) {
        let s = self.style_node;
        let d = &mut self.dims;
        let width = get_absolute_num(s, dims, "width").unwrap_or(0.0);
        let is_left_margin = s.get_value("margin-left");
        let is_right_margin = s.get_value("margin-right");
        let left_margin = match is_left_margin {
            Some(m) => match **m {
                Value::Other(ref s) => s.parse().unwrap_or(0.0),
                _ => 0.0,
            },
            None => 0.0,
        };
        let right_margin = match is_right_margin {
            Some(m) => match **m {
                Value::Other(ref s) => s.parse().unwrap_or(0.0),
                _ => 0.0,
            },
            None => 0.0,
        };
        d.padding.left = s.num_or("padding-left", 0.0);
        d.padding.right = s.num_or("padding-right", 0.0);
        d.border.left = s.num_or("border-left-width", 0.0);
        d.border.right = s.num_or("border-right-width", 0.0);

        let total = width
            + left_margin
            + right_margin
            + d.border.left
            + d.border.right
            + d.padding.right
            + d.padding.left;
        let underflow = dims.coordinates.width - total;
        match (width, is_left_margin, is_right_margin) {
            (0.0, _, _) => {
                if underflow >= 0.0 {
                    d.coordinates.width = underflow;
                    d.margin.right = right_margin;
                } else {
                    d.margin.right = right_margin + underflow;
                    d.coordinates.width = width;
                }
                d.margin.left = left_margin;
            }
            (w, None, Some(_)) if w != 0.0 => {
                d.margin.left = underflow;
                d.margin.right = right_margin;
                d.coordinates.width = w;
            }
            (w, Some(_), None) if w != 0.0 => {
                d.margin.right = underflow;
                d.margin.left = left_margin;
                d.coordinates.width = w;
            }
            (w, None, None) if w != 0.0 => {
                d.margin.left = underflow / 2.0;
                d.margin.right = underflow / 2.0;
                d.coordinates.width = w;
            }
            (_, _, _) => {
                d.margin.right = right_margin + underflow;
                d.margin.left = left_margin;
                d.coordinates.width = width
            }
        }
    }
    fn calculate_vertical(&mut self, dims: Dimensions) {
        let style = self.style_node;
        let d = &mut self.dims;

        d.margin.top = style.num_or("margin-top", 0.0);
        d.margin.bottom = style.num_or("margin-bottom", 0.0);
        d.padding.top = style.num_or("padding-top", 0.0);
        d.padding.bottom = style.num_or("padding-bottom", 0.0);
        d.border.top = style.num_or("border-top-width", 0.0);
        d.border.bottom = style.num_or("border-bottom-width", 0.0);

        d.coordinates.x = dims.coordinates.x + d.margin.left + d.padding.left + d.border.left;
        d.coordinates.y =
            dims.coordinates.y + d.current.y + d.border.top + d.margin.top + d.padding.top;
    }
    fn layout_children(&mut self) {
        let dims = &mut self.dims;
        let mut max_child_height = 0.0;

        let mut previous_box_type = BoxType::Block;

        for single_child in &mut self.children_nodes {
            match previous_box_type {
                BoxType::InlineBlock => match single_child.boxtype {
                    BoxType::Block => {
                        dims.coordinates.height = max_child_height;
                        dims.current.x = 0.0;
                    }
                    _ => {}
                },
                _ => {}
            }
            single_child.layout(*dims);
            let new_height = single_child.dims.get_margin_dimensions().width;

            if new_height > max_child_height {
                max_child_height = new_height;
            }

            match single_child.boxtype {
                BoxType::Block => {
                    dims.coordinates.height += single_child.dims.get_margin_dimensions().height
                }
                BoxType::InlineBlock => {
                    dims.coordinates.x += single_child.dims.get_margin_dimensions().width;

                    if dims.coordinates.x > dims.coordinates.width {
                        dims.coordinates.height += max_child_height;
                        dims.current.x = 0.0;
                        single_child.layout(*dims);
                        dims.current.x += single_child.dims.get_margin_dimensions().width;
                    }
                }
                _ => {}
            }
            previous_box_type = single_child.boxtype.clone();
        }
    }
    fn calculate_height(&mut self) {
        self.style_node
            .get_value("height")
            .map_or((), |h| match **h {
                Value::Length(n, _) => self.dims.coordinates.height = n,
                _ => {}
            })
    }
}
impl<'a> fmt::Debug for LayoutContainer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "type:\n BoxType: {:?}\n Dimensions: {:?}\n",
            self.boxtype, self.dims
        )
    }
}

impl Dimensions {
    pub fn get_padding_dimensions(&self) -> Rectangle {
        self.coordinates.expand(self.padding)
    }

    pub fn get_border_dimensions(&self) -> Rectangle {
        self.get_padding_dimensions().expand(self.border)
    }
    pub fn get_margin_dimensions(&self) -> Rectangle {
        self.get_border_dimensions().expand(self.margin)
    }
}
impl fmt::Debug for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Coordinates: {:?}\n Padding: {:?} Margin: {:?} Border: {:?}\n",
            self.coordinates, self.padding, self.margin, self.border
        )
    }
}
impl Rectangle {
    fn expand(&self, e: EdgeValues) -> Rectangle {
        Rectangle {
            x: self.x - e.left,
            y: self.y - e.top,
            width: self.width + e.left + e.right,
            height: self.height + e.top + e.bottom,
        }
    }
}
impl fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "X:{:?}\n Y:{:?}\n Width:{:?}\n Height:{:?}\n",
            self.x, self.y, self.width, self.height
        )
    }
}

impl fmt::Debug for EdgeValues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Left:{:?} \n Top:{:?}\n Right:{:?}\n Bottom:{:?}",
            self.left, self.top, self.right, self.bottom
        )
    }
}

impl fmt::Debug for BoxType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let box_type_string = match *self {
            BoxType::Block => "block",
            BoxType::Inline => "inline",
            BoxType::InlineBlock => "inline-block",
            _ => "undefined",
        };
        write!(f, "Box Type:{:?}\n", box_type_string)
    }
}
//Get value of a property number
fn get_absolute_num(style_node: &StyleNode, parent_dims: Dimensions, prop: &str) -> Option<f32> {
    match style_node.get_value(prop) {
        Some(ref v) => match ***v {
            Value::Length(l, ref u) => match *u {
                Unit::Px => Some(l),
                Unit::Percent => Some(l * parent_dims.coordinates.width / 100.0),
                _ => Some(0.0),
            },
            _ => None,
        },
        None => None,
    }
}

pub fn get_layout_tree<'a>(
    root: &'a StyleNode<'a>,
    mut viewport_dims: Dimensions,
) -> LayoutContainer<'a> {
    viewport_dims.coordinates.height = 0.0;
    let mut root_container = generate_layout_tree(root);
    root_container.layout(viewport_dims);
    root_container
}

fn generate_layout_tree<'a>(node: &'a StyleNode) -> LayoutContainer<'a> {
    let mut layout_node = LayoutContainer::new(
        match node.get_display_value() {
            Display::Block => BoxType::Block,
            Display::Inline => BoxType::Inline,
            Display::InlineBlock => BoxType::InlineBlock,
            Display::None => BoxType::Anonymous,
        },
        node,
    );
    for child in &node.children {
        match child.get_display_value() {
            Display::None => {}
            _ => layout_node.children_nodes.push(generate_layout_tree(child)),
        }
    }
    layout_node
}
