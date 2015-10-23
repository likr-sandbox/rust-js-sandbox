extern crate webplatform;

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use self::webplatform::{Document,HtmlNode};
use super::core::{AttributeValue,Component,Node};

struct Context<'a, Controller, T: Component<Controller>> {
    controller: Rc<RefCell<Controller>>,
    component: T,
    doc: Rc<RefCell<Document<'a>>>,
    root: Rc<RefCell<HtmlNode<'a>>>,
}

pub fn mount<'a, Controller, T: Component<Controller>>(document: Rc<RefCell<Document<'a>>>, target: Rc<RefCell<HtmlNode<'a>>>, component: T) {
    let context = Rc::new(RefCell::new(Context {
        controller: Rc::new(RefCell::new(component.controller())),
        component: component,
        doc: document,
        root: target,
    }));
    redraw(context);
}

fn redraw<'a, Controller, T: Component<Controller>>(context: Rc<RefCell<Context<'a, Controller, T>>>) {
    let node = context.borrow().component.view(context.borrow().controller.clone());
    let ctx = context.borrow();
    let target = ctx.root.borrow();
    render(target.deref(), node, context.clone());
}

fn render<'a, Controller, T: Component<Controller>>(target: &HtmlNode<'a>, node: Node, context: Rc<RefCell<Context<'a, Controller, T>>>) {
    match node {
        Node::Text(content) => {
            target.html_set(content.as_slice());
        },
        Node::Element(element) => {
            let ctx = context.borrow();
            let doc = ctx.doc.borrow();
            let this = doc.element_create(element.tag.as_slice()).unwrap();
            for (name, value) in element.attributes {
                match value {
                    AttributeValue::Text(text) => {
                        this.prop_set_str(name.as_slice(), text.as_slice())
                    },
                    AttributeValue::EventHandler(mut f) => {
                        this.on("input", {
                            let ctx = context.clone();
                            move |mut e| {
                                f(&mut e);
                                // redraw(ctx);
                            }
                        });
                    },
                }
            }
            for child in element.children {
                render(&this, child, context.clone());
            }
            target.append(&this);
        }
    }
}
