use std::fmt;
use std::default::Default;

pub struct Stylesheet{
    pub css:Vec<Rule>,
}
pub struct Rule{
    pub selectors:Vec<Selector>,
    pub declarations:Vec<Declaration>,
}
#[derive(PartialEq,Eq)]
pub struct Selector{
    pub simple:Vec<SimpleSelector>,
    pub combinators:Vec<char>,
}
#[derive(PartialEq,Eq)]
pub struct SimpleSelector{
    pub tag_name:Option<String>,
    pub id :Option<String>,
    pub classes:Vec<String>, 
}
pub struct Declaration{
    pub key:String,
    pub value:Value,
}
pub enum Value{
    Color(Color),
    Length(f32,Unit),
    Other(String),

}
#[derive(PartialEq, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}



impl Stylesheet{
    pub fn new(css:Vec<Rule>)->Stylesheet{
        Stylesheet{
            css
        }
    }
}
impl Default for Stylesheet{
   fn default()->Self{
       Stylesheet{css:Vec::new()}
   }
}
impl fmt::Debug for Stylesheet{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        let mut result=String::new();
        for rule in &self.css {
            if result.len() >0{
                result.push_str("\n\n");
            }
            result.push_str(&format!("{:?}",rule));
        }
        write!(f,"{}",result)
    }
}

impl Rule{
    pub fn new(selectors:Vec<Selector>,declarations:Vec<Declaration>)->Rule{
        Rule{selectors,declarations}
    }
}
impl Default for Rule{
    fn default()->Self{
       Rule{selectors:Vec::new(),declarations:Vec::new()}
   } 
}

impl fmt::Debug for Rule{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        let mut selector_result=String::new();
        let mut declaration_result=String::new();

        for selector in &self.selectors{
            if selector_result.len() >0{
                selector_result.push(',');
            }
            selector_result.push_str(&format!("{:?}",selector));

        }
        for declaration in &self.declarations{
            if declaration_result.len() >0{
                declaration_result.push(',');
            }
            declaration_result.push_str(&format!("{:?}",declaration));
        }
        write!(f,"{} {{\n{}}}",selector_result,declaration_result)

    }
}

impl Selector{
    pub fn new(simple:Vec<SimpleSelector>,combinators:Vec<char>)->Selector{
        Selector{simple,combinators}
    }
}
impl Default for Selector{
    fn default()->Self{
        Selector{simple:Vec::new(),combinators:Vec::new()}
   } 
}

impl fmt::Debug for Selector{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        let mut result=String::new();
        for sel in &self.simple{
            if result.len() >0{
                result.push(',');
            }
            result.push_str(&format!("{:?}",sel));
        }   
    
    write!(f,"{}",result)

    }
}

impl SimpleSelector{
    pub fn new(tag_name:Option<String>,id:Option<String>,classes:Vec<String>)->SimpleSelector{
        SimpleSelector{tag_name,id,classes}
    }
}
impl Default for SimpleSelector{
    fn default()->Self{
        SimpleSelector{tag_name:Option::None,id:Option::None,classes:Vec::new()}
    }
}

impl fmt::Debug for SimpleSelector{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        let mut result=String::new();
        match self.tag_name{
            Some(ref t)=>result.push_str(t),
            None=>{}
        }
        match self.id{
            Some(ref t)=>result.push_str(t),
            None=>{}
        }
        for class in &self.classes{
            result.push('.');
            result.push_str(class);

        }
        write!(f,"{}",result)
    }
}


impl Declaration{
    pub fn new(key:String,value:Value)->Declaration{
        Declaration{key,value}
    }
}

impl Default for Declaration{
    fn default()->Self{
        Declaration{key:String::new(),value:Value::Other(String::from("")),}
    }
}

impl fmt::Debug for Declaration{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        write!(f,"{}:{:?}",self.key,self.value)
    }
}

impl fmt::Debug for Value{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        match *self{
            Value::Color(ref c)=>write!(f,"{:?}",c),
            Value::Length(l,_)=>write!(f,"{:?}",l),
            Value::Other(ref s)=>write!(f,"{:?}",s),
        }
    }
}
impl Color{
    pub fn new(r:f32,g:f32,b:f32,a:f32)->Color{
        Color{r,g,b,a}
    }
}
impl Default for Color{
     fn default()->Color{
        Color::new(1.0, 1.0, 1.0, 1.0)
    }
}
impl fmt::Debug for Color{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        write!(f,"{} , {} ,{} ,{}",self.r,self.g,self.b,self.a)
    }
}


#[derive(PartialEq)]
pub enum Unit {
    Em,
    Ex,
    Ch,
    Rem,
    Vh,
    Vw,
    Vmin,
    Vmax,
    Px,
    Mm,
    Q,
    Cm,
    In,
    Pt,
    Pc,
    Percent,
}
pub fn match_unit(s:&str)->Unit{
    match s{
        "em" => Unit::Em,
        "ex" =>  Unit::Ex,
        "ch" =>  Unit::Ch,
        "rem" =>  Unit::Rem,
        "vh" => Unit::Vh,
        "vw" => Unit::Vw,
        "vmin" => Unit::Vmin,
        "vmax" => Unit::Vmax,
        "mm" => Unit::Mm,
        "q" => Unit::Q,
        "cm" => Unit::Cm,
        "in" => Unit::In,
        "pt" => Unit::Pt,
        "pc" => Unit::Pc,
        "%" => Unit::Percent,
        _ => Unit::Px,

    }
}

pub fn match_color_by_name(color:&str)->Color{
    return match color {
        "black" => Color::new(0.0, 0.0, 0.0, 1.0),
        "silver" => Color::new(
            0.7529411764705882,
            0.7529411764705882,
            0.7529411764705882,
            1.0,
        ),
        "gray" | "grey" => Color::new(
            0.5019607843137255,
            0.5019607843137255,
            0.5019607843137255,
            1.0,
        ),
        "white" => Color::new(1.0, 1.0, 1.0, 1.0),
        "maroon" => Color::new(0.5019607843137255, 0.0, 0.0, 1.0),
        "red" => Color::new(1.0, 0.0, 0.0, 1.0),
        "purple" => Color::new(0.5019607843137255, 0.0, 0.5019607843137255, 1.0),
        "fuchsia" => Color::new(1.0, 0.0, 1.0, 1.0),
        "green" => Color::new(0.0, 0.5019607843137255, 0.0, 1.0),
        "lime" => Color::new(0.0, 1.0, 0.0, 1.0),
        "olive" => Color::new(0.5019607843137255, 0.5019607843137255, 0.0, 1.0),
        "yellow" => Color::new(1.0, 1.0, 0.0, 1.0),
        "navy" => Color::new(0.0, 0.0, 0.5019607843137255, 1.0),
        "blue" => Color::new(0.0, 0.0, 1.0, 1.0),
        "teal" => Color::new(0.0, 0.5019607843137255, 0.5019607843137255, 1.0),
        "aqua" => Color::new(0.0, 1.0, 1.0, 1.0),
        "orange" => Color::new(1.0, 0.6470588235294118, 0.0, 1.0),
        "aliceblue" => Color::new(0.9411764705882353, 0.9725490196078431, 1.0, 1.0),
        "antiquewhite" => Color::new(
            0.9803921568627451,
            0.9215686274509803,
            0.8431372549019608,
            1.0,
        ),
        "aquamarine" => Color::new(0.4980392156862745, 1.0, 0.8313725490196079, 1.0),
        "azure" => Color::new(0.9411764705882353, 1.0, 1.0, 1.0),
        "beige" => Color::new(
            0.9607843137254902,
            0.9607843137254902,
            0.8627450980392157,
            1.0,
        ),
        "bisque" => Color::new(1.0, 0.8941176470588236, 0.7686274509803922, 1.0),
        "blanchedalmond" => Color::new(1.0, 0.9215686274509803, 0.803921568627451, 1.0),
        "blueviolet" => Color::new(
            0.5411764705882353,
            0.16862745098039217,
            0.8862745098039215,
            1.0,
        ),
        "brown" => Color::new(
            0.6470588235294118,
            0.16470588235294117,
            0.16470588235294117,
            1.0,
        ),
        "burlywood" => Color::new(
            0.8705882352941177,
            0.7215686274509804,
            0.5294117647058824,
            1.0,
        ),
        "cadetblue" => Color::new(
            0.37254901960784315,
            0.6196078431372549,
            0.6274509803921569,
            1.0,
        ),
        "chartreuse" => Color::new(0.4980392156862745, 1.0, 0.0, 1.0),
        "chocolate" => Color::new(
            0.8235294117647058,
            0.4117647058823529,
            0.11764705882352941,
            1.0,
        ),
        "coral" => Color::new(1.0, 0.4980392156862745, 0.3137254901960784, 1.0),
        "cornflowerblue" => Color::new(
            0.39215686274509803,
            0.5843137254901961,
            0.9294117647058824,
            1.0,
        ),
        "cornsilk" => Color::new(1.0, 0.9725490196078431, 0.8627450980392157, 1.0),
        "crimson" => Color::new(
            0.8627450980392157,
            0.0784313725490196,
            0.23529411764705882,
            1.0,
        ),
        "darkblue" => Color::new(0.0, 0.0, 0.5450980392156862, 1.0),
        "darkcyan" => Color::new(0.0, 0.5450980392156862, 0.5450980392156862, 1.0),
        "darkgoldenrod" => Color::new(
            0.7215686274509804,
            0.5254901960784314,
            0.043137254901960784,
            1.0,
        ),
        "darkgray" | "darkgrey" => Color::new(
            0.6627450980392157,
            0.6627450980392157,
            0.6627450980392157,
            1.0,
        ),
        "darkgreen" => Color::new(0.0, 0.39215686274509803, 0.0, 1.0),
        "darkkhaki" => Color::new(
            0.7411764705882353,
            0.7176470588235294,
            0.4196078431372549,
            1.0,
        ),
        "darkmagenta" => Color::new(0.5450980392156862, 0.0, 0.5450980392156862, 1.0),
        "darkolivegreen" => Color::new(
            0.3333333333333333,
            0.4196078431372549,
            0.1843137254901961,
            1.0,
        ),
        "darkorange" => Color::new(1.0, 0.5490196078431373, 0.0, 1.0),
        "darkorchid" => Color::new(0.6, 0.19607843137254902, 0.8, 1.0),
        "darkred" => Color::new(0.5450980392156862, 0.0, 0.0, 1.0),
        "darksalmon" => Color::new(
            0.9137254901960784,
            0.5882352941176471,
            0.47843137254901963,
            1.0,
        ),
        "darkseagreen" => Color::new(
            0.5607843137254902,
            0.7372549019607844,
            0.5607843137254902,
            1.0,
        ),
        "darkslateblue" => Color::new(
            0.2823529411764706,
            0.23921568627450981,
            0.5450980392156862,
            1.0,
        ),
        "darkslategray" | "darkslategrey" => Color::new(
            0.1843137254901961,
            0.30980392156862746,
            0.30980392156862746,
            1.0,
        ),
        "darkturquoise" => Color::new(0.0, 0.807843137254902, 0.8196078431372549, 1.0),
        "darkviolet" => Color::new(0.5803921568627451, 0.0, 0.8274509803921568, 1.0),
        "deeppink" => Color::new(1.0, 0.0784313725490196, 0.5764705882352941, 1.0),
        "deepskyblue" => Color::new(0.0, 0.7490196078431373, 1.0, 1.0),
        "dimgray" | "dimgrey" => Color::new(
            0.4117647058823529,
            0.4117647058823529,
            0.4117647058823529,
            1.0,
        ),
        "dodgerblue" => Color::new(0.11764705882352941, 0.5647058823529412, 1.0, 1.0),
        "firebrick" => Color::new(
            0.6980392156862745,
            0.13333333333333333,
            0.13333333333333333,
            1.0,
        ),
        "floralwhite" => Color::new(1.0, 0.9803921568627451, 0.9411764705882353, 1.0),
        "forestgreen" => Color::new(
            0.13333333333333333,
            0.5450980392156862,
            0.13333333333333333,
            1.0,
        ),
        "gainsboro" => Color::new(
            0.8627450980392157,
            0.8627450980392157,
            0.8627450980392157,
            1.0,
        ),
        "ghostwhite" => Color::new(0.9725490196078431, 0.9725490196078431, 1.0, 1.0),
        "gold" => Color::new(1.0, 0.8431372549019608, 0.0, 1.0),
        "goldenrod" => Color::new(
            0.8549019607843137,
            0.6470588235294118,
            0.12549019607843137,
            1.0,
        ),
        "greenyellow" => Color::new(0.6784313725490196, 1.0, 0.1843137254901961, 1.0),
        "honeydew" => Color::new(0.9411764705882353, 1.0, 0.9411764705882353, 1.0),
        "hotpink" => Color::new(1.0, 0.4117647058823529, 0.7058823529411765, 1.0),
        "indianred" => Color::new(
            0.803921568627451,
            0.3607843137254902,
            0.3607843137254902,
            1.0,
        ),
        "indigo" => Color::new(0.29411764705882354, 0.0, 0.5098039215686274, 1.0),
        "ivory" => Color::new(1.0, 1.0, 0.9411764705882353, 1.0),
        "khaki" => Color::new(
            0.9411764705882353,
            0.9019607843137255,
            0.5490196078431373,
            1.0,
        ),
        "lavender" => Color::new(
            0.9019607843137255,
            0.9019607843137255,
            0.9803921568627451,
            1.0,
        ),
        "lavenderblush" => Color::new(1.0, 0.9411764705882353, 0.9607843137254902, 1.0),
        "lawngreen" => Color::new(0.48627450980392156, 0.9882352941176471, 0.0, 1.0),
        "lemonchiffon" => Color::new(1.0, 0.9803921568627451, 0.803921568627451, 1.0),
        "lightblue" => Color::new(
            0.6784313725490196,
            0.8470588235294118,
            0.9019607843137255,
            1.0,
        ),
        "lightcoral" => Color::new(
            0.9411764705882353,
            0.5019607843137255,
            0.5019607843137255,
            1.0,
        ),
        "lightcyan" => Color::new(0.8784313725490196, 1.0, 1.0, 1.0),
        "lightgoldenrodyellow" => Color::new(
            0.9803921568627451,
            0.9803921568627451,
            0.8235294117647058,
            1.0,
        ),
        "lightgray" | "lightgrey" => Color::new(
            0.8274509803921568,
            0.8274509803921568,
            0.8274509803921568,
            1.0,
        ),
        "lightgreen" => Color::new(
            0.5647058823529412,
            0.9333333333333333,
            0.5647058823529412,
            1.0,
        ),
        "lightpink" => Color::new(1.0, 0.7137254901960784, 0.7568627450980392, 1.0),
        "lightsalmon" => Color::new(1.0, 0.6274509803921569, 0.47843137254901963, 1.0),
        "lightseagreen" => Color::new(
            0.12549019607843137,
            0.6980392156862745,
            0.6666666666666666,
            1.0,
        ),
        "lightskyblue" => Color::new(
            0.5294117647058824,
            0.807843137254902,
            0.9803921568627451,
            1.0,
        ),
        "lightslategray" | "lightslategrey" => {
            Color::new(0.4666666666666667, 0.5333333333333333, 0.6, 1.0)
        }
        "lightsteelblue" => Color::new(
            0.6901960784313725,
            0.7686274509803922,
            0.8705882352941177,
            1.0,
        ),
        "lightyellow" => Color::new(1.0, 1.0, 0.8784313725490196, 1.0),
        "limegreen" => Color::new(
            0.19607843137254902,
            0.803921568627451,
            0.19607843137254902,
            1.0,
        ),
        "linen" => Color::new(
            0.9803921568627451,
            0.9411764705882353,
            0.9019607843137255,
            1.0,
        ),
        "mediumaquamarine" => Color::new(0.4, 0.803921568627451, 0.6666666666666666, 1.0),
        "mediumblue" => Color::new(0.0, 0.0, 0.803921568627451, 1.0),
        "mediumorchid" => Color::new(
            0.7294117647058823,
            0.3333333333333333,
            0.8274509803921568,
            1.0,
        ),
        "mediumpurple" => Color::new(
            0.5764705882352941,
            0.4392156862745098,
            0.8588235294117647,
            1.0,
        ),
        "mediumseagreen" => Color::new(
            0.23529411764705882,
            0.7019607843137254,
            0.44313725490196076,
            1.0,
        ),
        "mediumslateblue" => Color::new(
            0.4823529411764706,
            0.40784313725490196,
            0.9333333333333333,
            1.0,
        ),
        "mediumspringgreen" => Color::new(0.0, 0.9803921568627451, 0.6039215686274509, 1.0),
        "mediumturquoise" => Color::new(0.2823529411764706, 0.8196078431372549, 0.8, 1.0),
        "mediumvioletred" => Color::new(
            0.7803921568627451,
            0.08235294117647059,
            0.5215686274509804,
            1.0,
        ),
        "midnightblue" => Color::new(
            0.09803921568627451,
            0.09803921568627451,
            0.4392156862745098,
            1.0,
        ),
        "mintcream" => Color::new(0.9607843137254902, 1.0, 0.9803921568627451, 1.0),
        "mistyrose" => Color::new(1.0, 0.8941176470588236, 0.8823529411764706, 1.0),
        "moccasin" => Color::new(1.0, 0.8941176470588236, 0.7098039215686275, 1.0),
        "navajowhite" => Color::new(1.0, 0.8705882352941177, 0.6784313725490196, 1.0),
        "oldlace" => Color::new(
            0.9921568627450981,
            0.9607843137254902,
            0.9019607843137255,
            1.0,
        ),
        "olivedrab" => Color::new(
            0.4196078431372549,
            0.5568627450980392,
            0.13725490196078433,
            1.0,
        ),
        "orangered" => Color::new(1.0, 0.27058823529411763, 0.0, 1.0),
        "orchid" => Color::new(
            0.8549019607843137,
            0.4392156862745098,
            0.8392156862745098,
            1.0,
        ),
        "palegoldenrod" => Color::new(
            0.9333333333333333,
            0.9098039215686274,
            0.6666666666666666,
            1.0,
        ),
        "palegreen" => Color::new(0.596078431372549, 0.984313725490196, 0.596078431372549, 1.0),
        "paleturquoise" => Color::new(
            0.6862745098039216,
            0.9333333333333333,
            0.9333333333333333,
            1.0,
        ),
        "palevioletred" => Color::new(
            0.8588235294117647,
            0.4392156862745098,
            0.5764705882352941,
            1.0,
        ),
        "papayawhip" => Color::new(1.0, 0.9372549019607843, 0.8352941176470589, 1.0),
        "peachpuff" => Color::new(1.0, 0.8549019607843137, 0.7254901960784313, 1.0),
        "peru" => Color::new(
            0.803921568627451,
            0.5215686274509804,
            0.24705882352941178,
            1.0,
        ),
        "pink" => Color::new(1.0, 0.7529411764705882, 0.796078431372549, 1.0),
        "plum" => Color::new(
            0.8666666666666667,
            0.6274509803921569,
            0.8666666666666667,
            1.0,
        ),
        "powderblue" => Color::new(
            0.6901960784313725,
            0.8784313725490196,
            0.9019607843137255,
            1.0,
        ),
        "rosybrown" => Color::new(
            0.7372549019607844,
            0.5607843137254902,
            0.5607843137254902,
            1.0,
        ),
        "royalblue" => Color::new(
            0.2549019607843137,
            0.4117647058823529,
            0.8823529411764706,
            1.0,
        ),
        "saddlebrown" => Color::new(
            0.5450980392156862,
            0.27058823529411763,
            0.07450980392156863,
            1.0,
        ),
        "salmon" => Color::new(
            0.9803921568627451,
            0.5019607843137255,
            0.4470588235294118,
            1.0,
        ),
        "sandybrown" => Color::new(
            0.9568627450980393,
            0.6431372549019608,
            0.3764705882352941,
            1.0,
        ),
        "seagreen" => Color::new(
            0.1803921568627451,
            0.5450980392156862,
            0.3411764705882353,
            1.0,
        ),
        "seashell" => Color::new(1.0, 0.9607843137254902, 0.9333333333333333, 1.0),
        "sienna" => Color::new(
            0.6274509803921569,
            0.3215686274509804,
            0.17647058823529413,
            1.0,
        ),
        "skyblue" => Color::new(
            0.5294117647058824,
            0.807843137254902,
            0.9215686274509803,
            1.0,
        ),
        "slateblue" => Color::new(
            0.41568627450980394,
            0.35294117647058826,
            0.803921568627451,
            1.0,
        ),
        "slategray" | "slategrey" => Color::new(
            0.4392156862745098,
            0.5019607843137255,
            0.5647058823529412,
            1.0,
        ),
        "snow" => Color::new(1.0, 0.9803921568627451, 0.9803921568627451, 1.0),
        "springgreen" => Color::new(0.0, 1.0, 0.4980392156862745, 1.0),
        "steelblue" => Color::new(
            0.27450980392156865,
            0.5098039215686274,
            0.7058823529411765,
            1.0,
        ),
        "tan" => Color::new(
            0.8235294117647058,
            0.7058823529411765,
            0.5490196078431373,
            1.0,
        ),
        "thistle" => Color::new(
            0.8470588235294118,
            0.7490196078431373,
            0.8470588235294118,
            1.0,
        ),
        "tomato" => Color::new(1.0, 0.38823529411764707, 0.2784313725490196, 1.0),
        "turquoise" => Color::new(
            0.25098039215686274,
            0.8784313725490196,
            0.8156862745098039,
            1.0,
        ),
        "violet" => Color::new(
            0.9333333333333333,
            0.5098039215686274,
            0.9333333333333333,
            1.0,
        ),
        "wheat" => Color::new(
            0.9607843137254902,
            0.8705882352941177,
            0.7019607843137254,
            1.0,
        ),
        "whitesmoke" => Color::new(
            0.9607843137254902,
            0.9607843137254902,
            0.9607843137254902,
            1.0,
        ),
        "yellowgreen" => Color::new(
            0.6039215686274509,
            0.803921568627451,
            0.19607843137254902,
            1.0,
        ),
        "rebeccapurple" => Color::new(0.4, 0.2, 0.6, 1.0),
        _ => Color::new(0.0, 0.0, 0.0, 1.0),
    }
}