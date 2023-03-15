use crate::csselements::{
    match_color_by_name, match_unit, Color, Declaration, Rule, Selector, SimpleSelector,
    Stylesheet, Unit, Value,
};

use std::iter::Peekable;
use std::str::Chars;

pub struct cssparser<'a> {
    chars: Peekable<Chars<'a>>,
}
impl<'a> cssparser<'a> {
    pub fn new(full_css: &str) -> cssparser {
        cssparser {
            chars: full_css.chars().peekable(),
        }
    }

    pub fn parse_stylesheet(&mut self) -> Stylesheet {
        let mut stylesheet = Stylesheet::default();

        while self.chars.peek().is_some() {
            let selectors = self.parse_selectors();
            let styles = self.parse_declarations();
            let rule = Rule::new(selectors, styles);

            stylesheet.css.push(rule);
        }
        stylesheet
    }
    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();

        while self.chars.peek().map_or(false, |c| *c != '{') {
            let selector = self.parse_single_selector();

            if selector != Selector::default() {
                selectors.push(selector);
            }

            self.consume_while(char::is_whitespace);
            if self.chars.peek().map_or(false, |c| *c == ',') {
                self.chars.next();
            }
        }

        self.chars.next();
        selectors
    }

    fn parse_single_selector(&mut self) -> Selector {
        let mut sselector = SimpleSelector::default();
        let mut selector = Selector::default();

        self.consume_while(char::is_whitespace);

        sselector.tag_name = match self.chars.peek() {
            Some(&c) if is_valid_start_indent(c) => Some(self.parse_identifier()),
            _ => None,
        };

        let mut multiple_ids = false;
        while self
            .chars
            .peek()
            .map_or(false, |c| *c != ',' && *c != '{' && !(*c).is_whitespace())
        {
            match self.chars.peek() {
                Some(&c) if c == '#' => {
                    self.chars.next();
                    if sselector.id.is_some() || multiple_ids {
                        sselector.id = None;
                        multiple_ids = true;
                        self.parse_id();
                    } else {
                        sselector.id = self.parse_id();
                    }
                }
                Some(&c) if c == '.' => {
                    self.chars.next();
                    let class_name = self.parse_identifier();

                    if class_name != String::from("") {
                        sselector.classes.push(class_name);
                    }
                }
                _ => {
                    self.consume_while(|c| c != ',' && c != '{');
                }
            }
        }

        if sselector != SimpleSelector::default() {
            selector.simple.push(sselector);
        }

        selector
    }

    fn parse_identifier(&mut self) -> String {
        let mut indent = String::new();

        match self.chars.peek() {
            Some(&c) => {
                if is_valid_start_indent(c) {
                    indent.push_str(&self.consume_while(is_valid_indent))
                }
            }
            None => {}
        }

        indent.to_lowercase()
    }
    fn parse_id(&mut self) -> Option<String> {
        match &self.parse_identifier()[..] {
            "" => None,
            s @ _ => Some(s.to_string()),
        }
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::<Declaration>::new();

        while self.chars.peek().map_or(false, |c| *c != '}') {
            self.consume_while(char::is_whitespace);

            let property = self.consume_while(|x| x != ':').to_lowercase();

            self.chars.next();
            self.consume_while(char::is_whitespace);

            let value = self
                .consume_while(|x| x != ';' && x != '\n' && x != '}')
                .to_lowercase();

            let value_enum = match property.as_ref() {
                "background-color" | "border-color" | "color" => {
                    Value::Color(translate_color(&value))
                }
                "margin-right"
                | "margin-bottom"
                | "margin-left"
                | "margin-top"
                | "padding-right"
                | "padding-bottom"
                | "padding-left"
                | "padding-top"
                | "border-right-width"
                | "border-bottom-width"
                | "border-left-width"
                | "border-top-width"
                | "height"
                | "width" => translate_length(&value),
                _ => Value::Other(value),
            };

            let declaration = Declaration::new(property, value_enum);

            if self.chars.peek().map_or(false, |c| *c == ';') {
                declarations.push(declaration);
                self.chars.next();
            } else {
                self.consume_while(char::is_whitespace);
                if self.chars.peek().map_or(false, |c| *c == '}') {
                    declarations.push(declaration);
                }
            }
            self.consume_while(char::is_whitespace);
        }

        self.chars.next();
        declarations
    }

    fn consume_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while self.chars.peek().map_or(false, |c| condition(*c)) {
            result.push(self.chars.next().unwrap());
        }

        result
    }
}

fn translate_length(value: &str) -> Value {
    let mut num_str = String::new();
    let mut unit = String::new();
    let mut parsing_num = true;

    for c in value.chars() {
        if c.is_numeric() && parsing_num {
            num_str.push(c);
        } else {
            unit.push(c);
            parsing_num = false;
        }
    }
    let parsed_number = num_str.parse().unwrap_or(0.0);
    Value::Length(parsed_number, match_unit(unit.as_ref()))
}

fn translate_color(color: &str) -> Color {
    if color.starts_with("#") {
        if color.len() == 7 {
            let red = match u8::from_str_radix(&color[1..3], 16) {
                Ok(n) => n as f32 / 255.0,
                Err(_) => 0.0,
            };
            let green = match u8::from_str_radix(&color[3..5], 16) {
                Ok(n) => n as f32 / 255.0,
                Err(_) => 0.0,
            };
            let blue = match u8::from_str_radix(&color[5..7], 16) {
                Ok(n) => n as f32 / 255.0,
                Err(_) => 0.0,
            };
            return Color::new(red, green, blue, 1.0);
        } else if color.len() == 4 {
            let red = match u8::from_str_radix(&color[1..2], 16) {
                Ok(n) => n as f32 / 15.0,
                Err(_) => 0.0,
            };
            let green = match u8::from_str_radix(&color[2..3], 16) {
                Ok(n) => n as f32 / 15.0,
                Err(_) => 0.0,
            };
            let blue = match u8::from_str_radix(&color[3..4], 16) {
                Ok(n) => n as f32 / 15.0,
                Err(_) => 0.0,
            };
            return Color::new(red, green, blue, 1.0);
        } else {
            return Color::default();
        }
    } else if color.starts_with("rgb") {
        return Color::default();
    } else if color.starts_with("hsl") {
        return Color::default();
    } else {
        match_color_by_name(color)
    }
}

fn is_valid_indent(c: char) -> bool {
    is_valid_start_indent(c) || c.is_digit(10) || c == '-'
}

fn is_valid_start_indent(c: char) -> bool {
    is_letter(c) || is_non_ascii(c) || c == '_'
}

fn is_letter(c: char) -> bool {
    is_upper_letter(c) || is_lower_letter(c)
}

fn is_upper_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

fn is_lower_letter(c: char) -> bool {
    c >= 'a' && c <= 'z'
}

fn is_non_ascii(c: char) -> bool {
    c >= '\u{0080}'
}
