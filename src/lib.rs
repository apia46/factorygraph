use std::{cell::RefCell, collections::HashMap};
use wasm_bindgen::prelude::*;
use web_sys::{Element, MouseEvent, WheelEvent, HtmlElement};
use gloo::{events::EventListener,utils::document};
use slotmap::{SlotMap, Key, new_key_type};

mod util;
mod state;
mod node;
mod item;
mod specification;

use util::*;
use specification::{Specification, ItemId, RecipeId};

#[wasm_bindgen(start)]
fn init() -> Result<(), JsValue> {
    state::init();

    web_sys::console::log_1(&"ok!! hello".into());

    // dragging the graph
    Box::leak(EventListener::new(&get_wrapper(), "mousedown", move |event| {
        let event = event.dyn_ref::<MouseEvent>().unwrap();
        if event.button() == 0 {
            state::STATE.with_borrow_mut(|state| {state::dragged::drag_graph(state);});
        }
    }).into());

    ["mouseup", "mouseleave"].iter().for_each(|event_type| {
        Box::leak(EventListener::new(&get_wrapper(), *event_type, move |_event| {
            state::STATE.with_borrow_mut(|state| {state::dragged::stop_drag(state);});
        }).into());
    });

    Box::leak(EventListener::new(&get_wrapper(), "wheel", |event| {
        let event = event.dyn_ref::<WheelEvent>().unwrap();
        state::STATE.with_borrow_mut(|state| {state::graph::change_scale_target(-event.delta_y(), state);});
    }).into());

    Ok(())
}

#[wasm_bindgen]
pub fn step() {
    state::STATE.with_borrow_mut(|state| {state::graph::approach_scale_target(0.5, state);});
}

#[wasm_bindgen]
pub fn add_node() {
    state::STATE.with_borrow_mut(|state| {node::ItemNode::create(state, Point::default(), ItemId("ITEM".to_owned()));});
}
