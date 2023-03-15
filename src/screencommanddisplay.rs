use crate::csselements::{Color, Value};
use crate::layouttree::{LayoutContainer, Rectangle};
use std::fmt;

pub type DisplayCommandList = Vec<DisplayCommand>;

pub enum DisplayCommand {
    SolidRect(Color, Rectangle),
}

pub fn get_display_command_list(root: &LayoutContainer) -> DisplayCommandList {
    let mut commands = Vec::new();
    render_layout_container(&mut commands, root);
    commands
}

fn render_layout_container(commands: &mut DisplayCommandList, root: &LayoutContainer) {
    render_background(commands, root);
    render_borders(commands, root);

    for child in &root.children_nodes {
        render_layout_container(commands, child)
    }
}

fn render_background(commands: &mut DisplayCommandList, root: &LayoutContainer) {
    get_background_color(root, "background-color").map(|color| {
        commands.push(DisplayCommand::SolidRect(
            color,
            root.dims.get_border_dimensions(),
        ))
    });
}

fn get_background_color(layout_container: &LayoutContainer, name: &str) -> Option<Color> {
    match layout_container.style_node.get_value(name) {
        Some(v) => match **v {
            Value::Color(ref c) => return Some(c.clone()),
            _ => return None,
        },
        _ => return None,
    }
}

fn render_borders(commands: &mut DisplayCommandList, root: &LayoutContainer) {
    let color = match get_background_color(root, "border-color") {
        Some(color) => color,
        _ => return,
    };
    let root_dims = &root.dims;
    let root_border_dims = root_dims.get_border_dimensions();

    commands.push(DisplayCommand::SolidRect(
        color.clone(),
        Rectangle {
            x: root_border_dims.x,
            y: root_border_dims.y,
            width: root_dims.border.left,
            height: root_border_dims.height,
        },
    ));
    commands.push(DisplayCommand::SolidRect(
        color.clone(),
        Rectangle {
            x: root_border_dims.x + root_border_dims.width - root_dims.border.right,
            y: root_border_dims.y,
            width: root_dims.border.left,
            height: root_border_dims.height,
        },
    ));
    commands.push(DisplayCommand::SolidRect(
        color.clone(),
        Rectangle {
            x: root_border_dims.x,
            y: root_border_dims.y,
            width: root_border_dims.width,
            height: root_dims.border.top,
        },
    ));
    commands.push(DisplayCommand::SolidRect(
        color.clone(),
        Rectangle {
            x: root_border_dims.x,
            y: root_border_dims.y + root_border_dims.height - root_dims.border.bottom,
            width: root_border_dims.width,
            height: root_dims.border.bottom,
        },
    ));
}
impl fmt::Debug for DisplayCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisplayCommand::SolidRect(ref c, ref rect) => {
                write!(f, "Color:{:?}\n Rectangle :{:?}", c, rect)
            }
        }
    }
}
