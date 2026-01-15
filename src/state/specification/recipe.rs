use super::*;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct RecipeId(pub String);

#[derive(PartialEq, Clone)]
pub struct Recipe {
    pub(super) name: String,
    pub(super) image: Option<String>,
    pub(super) machine: Specifier<Machine>,
    pub(super) duration: f64,
    pub(super) inputs: Vec<ItemStack>,
    pub(super) outputs: Vec<ItemStack>,
    pub(super) tags: Vec<RecipeTag>,
    pub(super) preprocessing: Option<Preprocessing>,
}
#[derive(PartialEq, Clone)]

pub struct Preprocessing {
    pub(super) possible_machines:Option<Vec<MachineId>>,
    pub(super) possible_process_modifiers:Option<Vec<ModifierId>>
}

#[derive(PartialEq, Eq, Clone)]
pub struct RecipeTag(pub String);

impl Specifiable for Recipe {
    type Id = RecipeId;
    type Tag = RecipeTag;

    fn get<'a>(recipe_id:&Self::Id, state:&'a S) -> Option<&'a Self> {
        state.specification.recipes.get(recipe_id)
    }
    
    fn has_tag(self:&Self, tag:&Self::Tag) -> bool {
        self.tags.iter().any(|check| check == tag)
    }
}
