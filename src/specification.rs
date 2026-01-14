use super::*;

#[derive(Default)]
pub struct Specification {
    items: HashMap<ItemId, Item>,
    recipes: HashMap<RecipeId, Recipe>,
    machines: HashMap<MachineId, Machine>,
    process_modifiers: Vec<ProcessModifier>,
}

#[derive(PartialEq, Eq, Hash)]
pub struct ItemId(pub String);
pub struct Item {
    name: String,
    image: Option<String>,
    tags: Vec<ItemTag>,
}
#[derive(PartialEq)]
pub struct ItemStack(pub ItemId, pub f64);

#[derive(PartialEq, Eq, Hash)]
pub struct RecipeId(pub String);
pub struct Recipe {
    name: String,
    image: Option<String>,
    machine: MachineTag,
    inputs: Vec<ItemStack>,
    outputs: Vec<ItemStack>,
    duration: f64,
    tags: Vec<RecipeTag>,
}

#[derive(PartialEq, Eq, Hash)]
pub struct MachineId(pub String);
pub struct Machine {
    name: String,
    image: Option<String>,
    tags: Vec<MachineTag>,
}

#[derive(PartialEq, Eq)]
pub struct ItemTag(pub String);
#[derive(PartialEq, Eq)]
pub struct RecipeTag(pub String);
#[derive(PartialEq, Eq)]
pub struct MachineTag(pub String);


pub trait Specifiable {
    type Id;
    type Tag;
}
impl Specifiable for Item {
    type Id = ItemId;
    type Tag = ItemTag;
}
impl Specifiable for Recipe {
    type Id = RecipeId;
    type Tag = RecipeTag;
}
impl Specifiable for Machine {
    type Id = MachineId;
    type Tag = MachineTag;
}
#[derive(PartialEq, Eq)]
pub enum Specifier<T: Specifiable> {
    Is(T::Id),
    Tag(T::Tag),
    TagsSome(Vec<T::Tag>),
    TagsAll(Vec<T::Tag>),
}

pub struct ProcessModifier {
    conditions: Vec<ProcessModifierCondition>, // anded together
    effects: Vec<ProcessModifierEffect>, // applied in order
}
pub enum ProcessModifierCondition {
    Recipe(Specifier<Recipe>),
    Machine(Specifier<Machine>),
    InputsSome(Specifier<Item>),
    InputsAll(Specifier<Item>),
    OutputsSome(Specifier<Item>),
    OutputsAll(Specifier<Item>),
}
pub enum ProcessModifierEffect {
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
}
