use web_sys::{Event, HtmlInputElement};

use super::*;

/// A node that marks a differential of an item at a connection
#[readonly::make]
pub struct ItemNode {
    pub key: state::graph::NodeKey,
    pub element: Element,
    pub position: Point<f64>,
    pub item: item::Item,
    pub value: ConstrainValue,
    start_drag_event: EventListener,
    delete_event: EventListener,
    flip_event: EventListener,
    toggle_constrain_event: EventListener,
    change_value_event: EventListener, // changed the value
    set_value_event: EventListener, // finished changing the value
}

#[derive(Debug)]
pub enum ConstrainValue {
    Constrained(f64),
    Unconstrained(Option<f64>),
}

impl ItemNode {
    pub fn create(position:Point<f64>, item_id:ItemId, state:&mut state::State) {
        let element = make_node_element("itemNode");
        let contents = element.create_child("div").with_class("contents");
        let item = item::Item::create(item_id, &contents, state);

        let right_side = contents.create_child("div").with_class("right-side");
        
        let value_container = right_side.create_child("div").with_class("value-container");

        let item_spec = specification::Item::get_or_default(&item.item_id, state);
        if let Some(unit) = item_spec.get_unit() {
            _ = value_container.set_attribute("unit", unit);
        }

        let value_input = value_container.create_child("input").with_class("value")
            .with_attr("type", "numeric")
            .with_attr("placeholder", "unconstrained");

        let constrain_label = right_side.create_child("label").with_class("constrain");

        let constrain_input = constrain_label.create_child("input").with_attr("type", "checkbox");

        let flipper = right_side.create_child("button").with_class("flipper").with_text_content("\u{f2f1}");

        state::graph::add_node(|key| {
            let delete_event = make_deleter_element(key, &element);
            
            let start_drag_event = EventListener::new(&element, "mousedown",  move |event| {
                let event = event.dyn_ref::<MouseEvent>().unwrap();
                event.stop_propagation();
                if event.button() == 0 {
                    state::borrow_state_mut(|state| state::dragged::drag_node(key, state));
                }
            });
            let flip_event = EventListener::new(&flipper, "click", move |_| {
                state::borrow_state(|state| state::graph::get_node(key, state).unwrap().flip());
            });
            let toggle_constrain_event = EventListener::new(&constrain_input, "click", move |_| {
                state::borrow_state_mut(|state| state::graph::get_node_mut(key, state).unwrap().constrain_toggled());
            });
            let change_value_event = EventListener::new(&value_input, "input", move |_| {
                state::borrow_state_mut(|state| state::graph::get_node_mut(key, state).unwrap().value_changed());
            });
            let set_value_event = EventListener::new(&value_input, "change", move |_| {
                state::borrow_state(|state| state::graph::get_node(key, state).unwrap().coerce_value_input());
            });
            let mut node = ItemNode {
                key,
                element,
                position: Point::default(),
                item,
                value: ConstrainValue::Unconstrained(None),
                start_drag_event, delete_event, flip_event, toggle_constrain_event, change_value_event, set_value_event
            };
            _ = node.element.cast::<HtmlElement>().set_attribute("node_id", &node.key.data().as_ffi().to_string());
            node.set_position(position);

            node
        }, state);
    }

    fn get_value_input(self:&Self) -> Element { self.element.query_selector_expect("input.value") }
    fn get_constrain_input(self:&Self) -> Element { self.element.query_selector_expect("label.constrain input") }

    pub fn set_position(self:&mut Self, to:Point<f64>) {
        self.position = to;
        _ = self.element.cast::<HtmlElement>().style().set_property("--posX", &to.x.to_string());
        _ = self.element.cast::<HtmlElement>().style().set_property("--posY", &to.y.to_string());
    }

    pub fn move_position(self:&mut Self, by:Point<f64>) {
        self.set_position(self.position+by);
    }

    fn flip(self:&Self) {
        _ = self.element.toggle_attribute("flipped");
    }

    pub fn set_constrain(self:&mut Self, to:bool) {
        let constrain_input = self.get_constrain_input();
        let constrain_input = constrain_input.cast::<HtmlInputElement>();
        if constrain_input.checked() != to {
            constrain_input.set_checked(to);
            self.constrain_toggled();
        }
    }

    fn constrain_toggled(self:&mut Self) {
        let value_input = self.get_value_input();
        let value_input = value_input.cast::<HtmlInputElement>();
        if self.get_constrain_input().cast::<HtmlInputElement>().checked() {
            value_input.set_placeholder("0");
            match self.value {
                ConstrainValue::Unconstrained(Some(value)) => self.value = ConstrainValue::Constrained(value),
                ConstrainValue::Unconstrained(None) => self.value = ConstrainValue::Constrained(0.0),
                _ => {}
            }
        } else {
            value_input.set_placeholder("unconstrained");
            match self.value {
                ConstrainValue::Constrained(0.0) => self.value = ConstrainValue::Unconstrained(None),
                ConstrainValue::Constrained(value) => self.value = ConstrainValue::Unconstrained(Some(value)),
                _ => {}
            }
        }
        self.coerce_value_input();
    }

    fn value_changed(self:&mut Self) {
        self.value = ConstrainValue::Constrained(self.get_value_input().cast::<HtmlInputElement>().value().parse::<f64>().unwrap_or_else(|_| 0.0));
        self.set_constrain(true);
    }

    fn coerce_value_input(self:&Self) {
        let value_input = self.get_value_input();
        let value_input = value_input.cast::<HtmlInputElement>();
        value_input.set_value(match self.value {
            ConstrainValue::Constrained(0.0) => "".to_owned(),
            ConstrainValue::Constrained(value) => value.to_string(),
            ConstrainValue::Unconstrained(Some(value)) => value.to_string(),
            ConstrainValue::Unconstrained(None) => "".to_owned(),
        }.as_str());
    }

    pub fn update_html(self:&Self, state:&state::State) {
        let item = specification::Item::get_or_default(&self.item.item_id, state);
        self.item.update_html(state);
        let value_container = self.element.query_selector_expect(".value_container");
        if let Some(unit) = item.get_unit() {
            _ = value_container.set_attribute("unit", unit);
        } else {
            _ = value_container.remove_attribute("unit");
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
    let deleter = element.create_child_prepend("div").with_class("deleter").with_text_content("\u{f00d}");
    EventListener::once(&deleter, "click", move |_| {
        state::borrow_state_mut(|state| {state::graph::delete_node(key, state)});
    })
}
