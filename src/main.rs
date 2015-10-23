extern crate webplatform;
extern crate mithril;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use webplatform::Event;
use mithril::{attribute, m, text_node, text_value, AttributeValue, Component, Node};

struct Main {
    state: Rc<RefCell<HashMap<String, String>>>
}

impl Component for Main {
    fn render(&self) -> Node {
        let key = String::from_str("message");
        let message = self.state.borrow().get(&key).unwrap().clone();
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
                        let state = self.state.clone();
                        move |e: &mut Event| {
                            let key = String::from_str("message");
                            let mut st = state.borrow_mut();
                            let node = e.target.as_mut().unwrap();
                            st.insert(key, node.prop_get_str("value"));
                            let key = String::from_str("message");
                            println!("{}", st.get(&key).unwrap());
                        }
                    }))),
                ], vec![])
            ]
        )
    }
}

fn main() {
    let mut state = HashMap::new();
    state.insert(String::from_str("message"), String::from_str("Hello"));
    let main_component = Rc::new(RefCell::new(Main {
        state: Rc::new(RefCell::new(state)),
    }));

    let document = Rc::new(RefCell::new(webplatform::init()));
    let body = Rc::new(RefCell::new(document.borrow().element_query("body").unwrap()));
    mithril::mount(document.clone(), body.clone(), main_component.clone());
    webplatform::spin();
}
