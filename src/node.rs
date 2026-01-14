use super::*;

#[readonly::make]
pub struct ItemNode {
    pub key: state::graph::NodeKey,
    pub element: Element,
    pub position: Point<f64>,
    start_drag: EventListener,
}
impl ItemNode {
    pub fn create(state:&mut state::State, position:Point<f64>) {
        let element = document().create_element("node").unwrap();
        _ = document().get_element_by_id("nodeElements").unwrap().append_child(&element);
        _ = element.as_html_element().class_list().add_1("itemNode");

        state::graph::add_node(|key| {
            let start_drag = EventListener::new(&element, "mousedown",  move |event| {
                let event = event.dyn_ref::<MouseEvent>().unwrap();
                event.stop_propagation();
                if event.button() == 0 {
                    state::dragged::drag_node(key);
                }
            });
            let mut node = ItemNode {
                key,
                element,
                position: Point::new(0.0, 0.0),
                start_drag
            };
            _ = node.element.as_html_element().set_attribute("node_id", &node.key.data().as_ffi().to_string());
            node.set_position(position);

            node
        }, state);
    }

    pub fn set_position(self:&mut Self, to:Point<f64>) {
        self.position = to;
        _ = self.element.as_html_element().style().set_property("--posX", &to.x.to_string());
        _ = self.element.as_html_element().style().set_property("--posY", &to.y.to_string());
    }

    pub fn move_position(self:&mut Self, by:Point<f64>) {
        self.set_position(self.position+by);
    }
}
