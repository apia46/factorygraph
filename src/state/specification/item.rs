use super::*;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ItemId(pub String);

#[derive(PartialEq, Clone)]
pub struct Item {
    pub(super) name: String,
    pub(super) image: Option<String>,
    pub(super) tags: Vec<ItemTag>,
}

#[derive(PartialEq, Clone)]
pub struct ItemStack{
    pub(super) specifier: Specifier<Item>,
    pub(super) count: f64
}

#[derive(PartialEq, Clone)]
pub struct SpecificItemStack{
    pub(super) id: ItemId,
    pub(super) count: f64
}

#[derive(PartialEq, Eq, Clone)]
pub struct ItemTag(pub String);

impl Specifiable for Item {
    type Id = ItemId;
    type Tag = ItemTag;
    
    fn get<'a>(item_id:&Self::Id, state:&'a S) -> Option<&'a Self> {
        state.specification.items.get(item_id)
    }
    
    fn has_tag(self:&Self, tag:&Self::Tag) -> bool {
        self.tags.iter().any(|check| check == tag)
    }
}
