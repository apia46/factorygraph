use super::*;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ModifierId(pub String);
#[derive(Debug)]
pub struct Modifier {
    pub(super) conditions: Vec<ModifierCondition>, // anded together
    pub(super) effects: Vec<ModifierEffect>, // applied in order
    pub(super) toggleable: bool,
}

#[derive(Debug)]
pub enum ModifierCondition {
    Recipe(Specifier<Recipe>),
    Machine(Specifier<Machine>),
    InputsSome(Specifier<Item>),
    InputsAll(Specifier<Item>),
    OutputsSome(Specifier<Item>),
    OutputsAll(Specifier<Item>),
}

#[derive(Debug)]
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
impl Debug for NumberEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Set(arg0) => f.debug_tuple("Set").field(arg0).finish(),
            Self::Add(arg0) => f.debug_tuple("Add").field(arg0).finish(),
            Self::Mul(arg0) => f.debug_tuple("Mul").field(arg0).finish(),
            Self::Function(_) => write!(f, "Function(?)"),
        }
    }
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
