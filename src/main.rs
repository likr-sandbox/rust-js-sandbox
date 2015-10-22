extern crate webplatform;
extern crate mithril;

use mithril::{m, AttributeValue, Component, Node};

struct MainController {
    message: &'static str,
}

struct Main;

impl Component<MainController> for Main {
    fn controller(&self) -> MainController {
        MainController {message: "Hello"}
    }

    fn view(&self, ctrl: &mut MainController) -> Node {
        m("div",
            vec![("class", AttributeValue::Text("unko"))],
            vec![
                m("p", vec![], vec![
                    Node::Text(ctrl.message)
                ]),
                m("p", vec![], vec![
                    Node::Text("world")
                ]),
                m("input", vec![
                    ("value", AttributeValue::Text(ctrl.message)),
                    ("onchange", AttributeValue::EventHandler),
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
