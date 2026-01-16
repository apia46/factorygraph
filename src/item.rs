use super::*;

// a receptacle for an item on a node
#[readonly::make]
pub struct Item {
    pub item_id: ItemId,
    pub element: Element,
}

impl Item {
    pub fn create(item_id:ItemId, parent:&Element, state: &state::State) -> Item {
        let element = document().create_element("item").unwrap();
        _ = parent.append_child(&element);
        let item = specification::Item::get(&item_id, state).unwrap_or_else(|| specification::Item::get_default(state));
        if let Some(image) = item.get_image() {
            _ = element.as_html_element().style().set_property("--image", format!("url({image})").as_str());
        }
        Item {
            item_id,
            element,
        }
    }
}
