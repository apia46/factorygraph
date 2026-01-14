use super::*;

pub mod graph {
    use super::*;
    type S = super::State;

    new_key_type! { pub struct NodeKey; }
    #[readonly::make]
    pub struct State {
        pub position: Point<f64>,
        pub scale: f64,
        log_scale: f64,
        log_scale_target: f64,
        last_zoom_point: Point<f64>,
        nodes: SlotMap<NodeKey, node::ItemNode>,
    }

    impl Default for State {
        fn default() -> State {
            State {
                position: Point::default(),
                scale: 1.0,
                log_scale: 0.0,
                log_scale_target: 0.0,
                last_zoom_point: Point::default(),
                nodes: SlotMap::with_key(),
            }
        }
    }

    pub fn set_position(to:Point<f64>, state:&mut S) {
        state.graph.position = to;
        let wrapper = get_wrapper();
        _ = wrapper.as_html_element().style().set_property("--posX", &to.x.to_string());
        _ = wrapper.as_html_element().style().set_property("--posY", &to.y.to_string());
    }
    pub fn move_position(by:Point<f64>, state:&mut S) {
        set_position(state.graph.position+by, state);
    }

    pub fn set_scale_target(to:f64, state:&mut S) {
        state.graph.log_scale_target = to;
        state.graph.last_zoom_point = screen_to_world(state.mouse_screen_position, &state);
    }
    pub fn change_scale_target(by:f64, state:&mut S) {
        set_scale_target(state.graph.log_scale_target+by, state);
    }
    fn set_log_scale(to:f64, state:&mut S) {
        state.graph.log_scale = to;
        state.graph.scale = 2.0f64.powf(to*0.003);
        _ = get_wrapper().as_html_element().style().set_property("--scale", &state.graph.scale.to_string());
    }
    pub fn approach_scale_target(speed:f64, state:&mut S) {
        let scale_before = state.graph.scale;
        set_log_scale(state.graph.log_scale.move_toward_exp(state.graph.log_scale_target, speed), state);
        let scale_after = state.graph.scale;
        move_position(state.graph.last_zoom_point * (scale_before - scale_after), state);
    }

    pub fn add_node<F>(node:F, state:&mut S) -> NodeKey where F: FnOnce(NodeKey) -> node::ItemNode {
        state.graph.nodes.insert_with_key(node)
    }
    /*pub fn get_node(key:NodeKey, state:&S) -> Option<&node::ItemNode> {
        state.graph.nodes.get(key)
    }*/
    pub fn get_node_mut(key:NodeKey, state:&mut S) -> Option<&mut node::ItemNode> {
        state.graph.nodes.get_mut(key)
    }
}

pub mod dragged {
    use super::*;
    type S = super::State;

    #[derive(Clone, Copy, Default)]
    pub enum State {
        #[default]
        Nothing,
        Graph,
        Node(graph::NodeKey),
    }

    pub fn stop_drag(state: &mut S) {
        state.dragged = State::Nothing;
    }

    pub fn drag_graph(state: &mut S) {
        state.dragged = State::Graph;
    }

    pub fn drag_node(key:graph::NodeKey, state: &mut S) {
        state.dragged = State::Node(key);
    }

    pub(super) fn process_drag(dist:Point<f64>) {
        match STATE.with_borrow(|state| state.dragged) {
            State::Nothing => {},
            State::Graph => {
                STATE.with_borrow_mut(|state| {graph::move_position(dist, state);});
            },
            State::Node(key) => {
                STATE.with_borrow_mut(|state| {
                    let scale = state.graph.scale;
                    let Some(node) = graph::get_node_mut(key, state) else { stop_drag(state); return };
                    node.move_position(dist/scale);
                });
            }
        }
    }
}

#[readonly::make]
#[derive(Default)]
pub struct State {
    pub graph: graph::State,
    pub dragged: dragged::State,
    pub mouse_screen_position: Point<i32>,
    pub specification: Specification,
}

thread_local! { pub static STATE:RefCell<State> = RefCell::new(State::default()); }

pub fn init() {
    Box::leak(EventListener::new(&get_wrapper(), "mousemove", |event| {
        let event = event.dyn_ref::<MouseEvent>().unwrap();
        dragged::process_drag(Point::new(event.movement_x().into(), event.movement_y().into()));

        STATE.with_borrow_mut(|state| {state.mouse_screen_position = Point::new(event.client_x(), event.client_y());});
    }).into());
}
