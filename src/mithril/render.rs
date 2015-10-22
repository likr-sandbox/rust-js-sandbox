extern crate webplatform;

use std::rc::Rc;
use std::cell::RefCell;
use self::webplatform::{Document,HtmlNode};
use super::core::{AttributeValue,Component,Node};

pub fn mount<'a, Controller, T: Component<Controller>>(document: &Document<'a>, target: &HtmlNode<'a>, component: T) {
    let mut controller = component.controller();
    let node = component.view(Rc::new(RefCell::new(controller)));
    render(&document, &target, node);
}

pub fn render<'a>(document: &Document<'a>, target: &HtmlNode<'a>, node: Node) {
    match node {
        Node::Text(content) => {
            target.html_set(content.as_slice());
        },
        Node::Element(element) => {
            let this = document.element_create(element.tag.as_slice()).unwrap();
            for (name, value) in element.attributes {
                match value {
                    AttributeValue::Text(text) => {
                        this.prop_set_str(name.as_slice(), text.as_slice())
                    },
                    AttributeValue::EventHandler(mut f) => {
                        this.on("input", move |mut e| {
                            f(&mut e);
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
