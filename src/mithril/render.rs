extern crate webplatform;

use self::webplatform::{Document,HtmlNode};
use super::core::{AttributeValue,Component,Node};

pub fn mount<'a, Controller, T: Component<Controller>>(document: &Document<'a>, target: &HtmlNode<'a>, component: T) {
    let mut controller = component.controller();
    let node = component.view(&mut controller);
    render(&document, &target, node);
}

pub fn render<'a>(document: &Document<'a>, target: &HtmlNode<'a>, node: Node) {
    match node {
        Node::Text(content) => {
            target.html_set(content);
        },
        Node::Element(element) => {
            let this = document.element_create(element.tag).unwrap();
            for (name, value) in element.attributes {
                match value {
                    AttributeValue::Text(text) => {
                        this.prop_set_str(name, text)
                    },
                    AttributeValue::EventHandler => {
                        this.on("input", |e| {
                            let node = e.target.unwrap();
                            println!("{}", node.prop_get_str("value"));
                        });
                    },
                }
            }
            for child in element.children {
                render(&document, &this, child);
            }
            target.append(&this);
        }
    }
}
