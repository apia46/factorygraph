use super::*;

// a receptacle for an item on a node
#[readonly::make]
pub struct Item {
    pub item_id: ItemId,
    pub element: Element,
}

impl Item {
    pub fn create(item_id:ItemId, parent:&Element, state: &state::State) -> Item {
        let element = parent.create_child("item");

        let item = specification::Item::get_or_default(&item_id, state);
        if let Some(image) = item.get_image() {
            _ = element.as_html_element().style().set_property("--image", format!("url({image})").as_str());
        }
        Item {
            item_id,
            element,
        }
    }

    pub fn update_html(self:&Self, state:&state::State) {
        let item = specification::Item::get_or_default(&self.item_id, state);
        if let Some(image) = item.get_image() {
            _ = self.element.as_html_element().style().set_property("--image", format!("url({image})").as_str());
        } else {
            _ = self.element.as_html_element().style().remove_property("--image");
        }
    }
}
