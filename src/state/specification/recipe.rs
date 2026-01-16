use super::*;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct RecipeId(pub String);

#[derive(PartialEq, Clone, Debug)]
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
#[derive(PartialEq, Clone, Debug)]

pub struct Preprocessing {
    pub(super) possible_machines:Option<Vec<MachineId>>,
    pub(super) possible_process_modifiers:Option<Vec<ModifierId>>
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RecipeTag(pub String);

impl Specifiable for Recipe {
    type Id = RecipeId;
    type Tag = RecipeTag;

    fn get<'a>(recipe_id:&Self::Id, state:&'a S) -> Option<&'a Self> {
        state.specification.recipes.get(recipe_id)
    }

    fn get_default(state:&S) -> &Self {
        state.specification.recipes.get(&RecipeId("unknown".to_owned())).expect("Did you forget to load the default specification?")
    }
    
    fn get_or_default<'a>(recipe_id:&Self::Id, state:&'a S) -> &'a Self {
        state.specification.recipes.get(recipe_id).unwrap_or_else(|| Self::get_default(state))
    }

    fn has_tag(self:&Self, tag:&Self::Tag) -> bool {
        self.tags.iter().any(|check| check == tag)
    }
}
