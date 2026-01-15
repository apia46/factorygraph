use super::*;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ModifierId(pub String);
pub struct Modifier {
    pub(super) conditions: Vec<ModifierCondition>, // anded together
    pub(super) effects: Vec<ModifierEffect>, // applied in order
    pub(super) toggleable: bool,
}
pub enum ModifierCondition {
    Recipe(Specifier<Recipe>),
    Machine(Specifier<Machine>),
    InputsSome(Specifier<Item>),
    InputsAll(Specifier<Item>),
    OutputsSome(Specifier<Item>),
    OutputsAll(Specifier<Item>),
}
pub enum ModifierEffect {
    AffectDuration(NumberEffect),
    AddInput(ItemStack),
    RemoveInput(Specifier<Item>),
    AffectInput(Specifier<Item>, NumberEffect),
    AddOutput(ItemStack),
    RemoveOutput(Specifier<Item>),
    AffectOutput(Specifier<Item>, NumberEffect),
}

pub enum NumberEffect {
    Set(f64),
    Add(f64),
    Mul(f64),
    Function(Box<dyn Fn(EffectFunctionArgs) -> f64>)
}
pub enum EffectFunctionArgs<'a> {
    Base(RecipeContext<'a>, &'a S),
    Input(RecipeContext<'a>, ItemContext<'a>, &'a S),
    Output(RecipeContext<'a>, ItemContext<'a>, &'a S),
}

pub struct RecipeContext<'a> {
    pub(super) recipe: &'a Recipe,
    pub(super) machine: &'a Machine
}
pub struct ItemContext<'a> {
    pub(super) item_stack: &'a SpecificItemStack
}
