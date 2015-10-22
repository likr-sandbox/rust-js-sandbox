extern crate webplatform;
extern crate mithril;

use std::rc::Rc;
use std::cell::RefCell;
use webplatform::Event;
use mithril::{attribute, m, text_node, text_value, AttributeValue, Component, Node};

struct MainController {
    message: String,
}

struct Main;

impl Component<MainController> for Main {
    fn controller(&self) -> MainController {
        MainController {message: String::from_str("Hello")}
    }

    fn view(&self, ctrl: Rc<RefCell<MainController>>) -> Node {
        let message = ctrl.borrow().message.clone();
        m("div",
            vec![attribute("class", text_value("unko"))],
            vec![
                m("p", vec![], vec![
                    Node::Text(message.clone()),
                ]),
                m("p", vec![], vec![
                  text_node("world"),
                ]),
                m("input", vec![
                    attribute("value", AttributeValue::Text(message.clone())),
                    attribute("change", AttributeValue::EventHandler(Box::new({
                        let ctrl = ctrl.clone();
                        move |e: &mut Event| {
                            let node = e.target.as_mut().unwrap();
                            ctrl.borrow_mut().message = node.prop_get_str("value");
                            println!("{}", ctrl.borrow_mut().message);
                        }
                    }))),
                ], vec![])
            ]
        )
    }
}

fn main() {
    let main_component = Main;

    let document = webplatform::init();
    let body = document.element_query("body").unwrap();
    mithril::mount(&document, &body, main_component);
    webplatform::spin();
}
