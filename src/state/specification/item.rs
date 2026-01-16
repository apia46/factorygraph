use super::*;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ItemId(pub String);

#[derive(PartialEq, Clone, Debug)]
pub struct Item {
    pub(super) name: String,
    pub(super) image: Option<String>,
    pub(super) tags: Vec<ItemTag>,
}
impl Item {
    pub fn get_name(self:&Self) -> &String {&self.name}
    pub fn get_image(self:&Self) -> Option<&String> {
        match &self.image {
            None => None,
            Some(string) => Some(string)
        }
    }
    pub fn get_tags(self:&Self) -> &Vec<ItemTag> {&self.tags}
}

#[derive(PartialEq, Clone, Debug)]
pub struct ItemStack{
    pub(super) specifier: Specifier<Item>,
    pub(super) count: f64
}

#[derive(PartialEq, Clone, Debug)]
pub struct SpecificItemStack{
    pub(super) id: ItemId,
    pub(super) count: f64
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ItemTag(pub String);

impl Specifiable for Item {
    type Id = ItemId;
    type Tag = ItemTag;
    
    fn get<'a>(item_id:&Self::Id, state:&'a S) -> Option<&'a Self> {
        state.specification.items.get(item_id)
    }

    fn get_default(state:&S) -> &Self {
        state.specification.items.get(&ItemId("unknown".to_owned())).expect("Did you forget to load the default specification?")
    }
    
    fn has_tag(self:&Self, tag:&Self::Tag) -> bool {
        self.tags.iter().any(|check| check == tag)
    }
}
