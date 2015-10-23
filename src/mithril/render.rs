extern crate webplatform;

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use self::webplatform::{Document,HtmlNode};
use super::core::{AttributeValue,Component,Node};

pub fn mount<'a, T: 'a + Component>(
        document: Rc<RefCell<Document<'a>>>,
        root: Rc<RefCell<HtmlNode<'a>>>,
        component: Rc<RefCell<T>>) {
    let target = root.borrow();
    target.html_set("");
    let node = component.borrow().render();
    render(document, target.deref(), node, root.clone(), component);
}

fn render<'a, T: 'a + Component>(
        document: Rc<RefCell<Document<'a>>>,
        target: &HtmlNode<'a>,
        node: Node,
        root: Rc<RefCell<HtmlNode<'a>>>,
        component: Rc<RefCell<T>>) {
    match node {
        Node::Text(content) => {
            target.html_set(content.as_slice());
        },
        Node::Element(element) => {
            let this = document.borrow().element_create(element.tag.as_slice()).unwrap();
            for (name, value) in element.attributes {
                match value {
                    AttributeValue::Text(text) => {
                        this.prop_set_str(name.as_slice(), text.as_slice())
                    },
                    AttributeValue::EventHandler(mut f) => {
                        this.on("input", {
                            let document = document.clone();
                            let root = root.clone();
                            let component = component.clone();
                            move |mut e| {
                                f(&mut e);
                                mount(document.clone(), root.clone(), component.clone());
                            }
                        });
                    },
                }
            }
            for child in element.children {
                render(document.clone(), &this, child, root.clone(), component.clone());
            }
            target.append(&this);
        }
    }
}
