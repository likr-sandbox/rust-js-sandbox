pub enum AttributeValue {
    Text(&'static str),
    EventHandler,
}

pub type Attribute = (&'static str, AttributeValue);

pub struct Element {
    pub tag: &'static str,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}

pub enum Node {
    Text(&'static str),
    Element(Element),
}

pub trait Component<T> {
    fn controller(&self) -> T;
    fn view(&self, &mut T) -> Node;
}

pub fn m(tag: &'static str, attributes: Vec<Attribute>, children: Vec<Node>) -> Node {
    Node::Element(Element {
        tag: tag,
        attributes: attributes,
        children: children
    })
}
