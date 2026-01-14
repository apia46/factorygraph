use super::*;

// a receptacle for an item on a node
#[readonly::make]
pub struct Item {
    pub item_id: ItemId,
    pub element: Element,
}

impl Item {
    pub fn create(item_id:ItemId, parent:&Element) -> Item {
        let element = document().create_element("item").unwrap();
        _ = parent.append_child(&element);

        Item {
            item_id,
            element,
        }
    }
}
