use super::*;

/// A node that marks a differential of an item at a connection
#[readonly::make]
pub struct ItemNode {
    pub key: state::graph::NodeKey,
    pub element: Element,
    pub position: Point<f64>,
    pub item: item::Item,
    start_drag: EventListener,
    deleter:EventListener,
}
impl ItemNode {
    pub fn create(position:Point<f64>, item_id:ItemId, state:&mut state::State) {
        let element = make_node_element("itemNode");
        let item = item::Item::create(item_id, &element, state);

        let right_side = element.create_child("div").with_class("right-side");
        
        let number_container = right_side.create_child("div").with_class("number-container");

        let item_spec = specification::Item::get_or_default(&item.item_id, state);
        if let Some(unit) = item_spec.get_unit() {
            _ = number_container.set_attribute("unit", unit);
        }

        let number_input = number_container.create_child("input").with_class("quantity")
            .with_attr("type", "numeric")
            .with_attr("placeholder", "unconstrained");

        let constrain_label = right_side.create_child("label").with_class("constrain");

        let constrain = constrain_label.create_child("input").with_attr("type", "checkbox");

        let flipper = right_side.create_child("button").with_class("flipper").with_text_content("\u{f2f1}");

        state::graph::add_node(|key| {
            let deleter = make_deleter_element(key, &element);
            
            let start_drag = EventListener::new(&element, "mousedown",  move |event| {
                let event = event.dyn_ref::<MouseEvent>().unwrap();
                event.stop_propagation();
                if event.button() == 0 {
                    state::borrow_state_mut(|state| {state::dragged::drag_node(key, state);});
                }
            });
            let mut node = ItemNode {
                key,
                element,
                position: Point::default(),
                item,
                start_drag,
                deleter,
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

    pub fn update_html(self:&Self, state:&state::State) {
        let item = specification::Item::get_or_default(&self.item.item_id, state);
        self.item.update_html(state);
        let number_container = self.element.query_selector_expect(".number_container");
        if let Some(unit) = item.get_unit() {
            _ = number_container.set_attribute("unit", unit);
        } else {
            _ = number_container.remove_attribute("unit");
        }
    }
}
impl Drop for ItemNode {
    fn drop(&mut self) {
        self.element.remove();
    }
}

fn make_node_element(class:&str) -> Element {
    document().get_element_by_id("nodeElements").unwrap().create_child("node").with_class(class)
}

fn make_deleter_element(key:state::graph::NodeKey, element:&Element) -> EventListener {
    let deleter = element.create_child("div").with_class("deleter").with_text_content("\u{f00d}");
    EventListener::once(&deleter, "click", move |_| {
        state::borrow_state_mut(|state| {state::graph::delete_node(key, state)});
    })
}
