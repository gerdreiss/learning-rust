use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Property {
    Simple(&'static str, String),
    Style(&'static str, String),
    Transform(String),
}

#[derive(Debug, PartialEq)]
pub struct SvgTag {
    pub kind: &'static str,
    pub properties: Vec<Property>,
    pub children: Vec<SvgTag>,
}

impl SvgTag {
    pub fn new(kind: &'static str) -> Self {
        SvgTag {
            kind,
            properties: Vec::new(),
            children: Vec::new(),
        }
    }
    pub fn child(mut self, c: SvgTag) -> Self {
        self.children.push(c);
        self
    }
    pub fn property<V: Display>(mut self, k: &'static str, v: V) -> Self {
        self.properties.push(Property::Simple(k, v.to_string()));
        self
    }
    pub fn style<V: Display>(mut self, k: &'static str, v: V) -> Self {
        self.properties.push(Property::Style(k, v.to_string()));
        self
    }
    pub fn x<V: Display>(self, v: V) -> Self {
        self.property("x", v)
    }
    pub fn y<V: Display>(self, v: V) -> Self {
        self.property("y", v)
    }
    pub fn w<V: Display>(self, v: V) -> Self {
        self.property("width", v)
    }
    pub fn h<V: Display>(self, v: V) -> Self {
        self.property("height", v)
    }
}

fn main() {
    let a = SvgTag::new("svg")
        .w("60px")
        .h("80px")
        .child(SvgTag::new("rect").x(5).y(5).w(50).h(20));
    println!("{:?}", a);
}
