use super::*;

/// A node that marks a differential of an item at a connection
#[readonly::make]
pub struct ItemNode {
    pub key: state::graph::NodeKey,
    pub element: Element,
    pub position: Point<f64>,
    pub item: item::Item,
    start_drag: EventListener,
}
impl ItemNode {
    pub fn create(state:&mut state::State, position:Point<f64>, item_id:ItemId) {
        let element = document().create_element("node").unwrap();
        _ = document().get_element_by_id("nodeElements").unwrap().append_child(&element);
        _ = element.as_html_element().class_list().add_1("itemNode");

        let item = item::Item::create(item_id, &element);

        state::graph::add_node(|key| {
            let start_drag = EventListener::new(&element, "mousedown",  move |event| {
                let event = event.dyn_ref::<MouseEvent>().unwrap();
                event.stop_propagation();
                if event.button() == 0 {
                    state::STATE.with_borrow_mut(|state| {state::dragged::drag_node(key, state);});
                }
            });
            let mut node = ItemNode {
                key,
                element,
                position: Point::default(),
                item,
                start_drag,
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
