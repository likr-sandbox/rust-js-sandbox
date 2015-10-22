extern crate webplatform;

use std::rc::Rc;
use std::cell::RefCell;
use self::webplatform::Event;

pub enum AttributeValue {
    Text(String),
    EventHandler(Box<FnMut(&mut Event)>),
}

pub type Attribute = (String, AttributeValue);

pub struct Element {
    pub tag: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}

pub enum Node {
    Text(String),
    Element(Element),
}

pub trait Component<T> {
    fn controller(&self) -> T;
    fn view(&self, Rc<RefCell<T>>) -> Node;
}

pub fn m(tag: &str, attributes: Vec<Attribute>, children: Vec<Node>) -> Node {
    Node::Element(Element {
        tag: String::from_str(tag),
        attributes: attributes,
        children: children
    })
}

pub fn text_value(val: &str) -> AttributeValue {
    AttributeValue::Text(String::from_str(val))
}

pub fn text_node(val: &str) -> Node {
    Node::Text(String::from_str(val))
}

pub fn attribute(name: &str, value: AttributeValue) -> Attribute {
    (String::from_str(name), value)
}
