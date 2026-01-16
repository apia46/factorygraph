use super::*;
type S = super::State;

mod item;
mod recipe;
mod machine;
mod specify;
mod modifier;

pub use item::*;
pub use recipe::*;
pub use machine::*;
pub use specify::*;
pub use modifier::*;

#[derive(Default, Debug)]
pub struct Specification {
    items: HashMap<ItemId, Item>,
    recipes: HashMap<RecipeId, Recipe>,
    machines: HashMap<MachineId, Machine>,
    modifiers: HashMap<ModifierId, Modifier>,
}

pub fn load_specification(specification: Specification, state: &mut S) {
    state.specification.items.extend(specification.items);
    state.specification.recipes.extend(specification.recipes);
    state.specification.machines.extend(specification.machines);
    state.specification.modifiers.extend(specification.modifiers);
    for node in state.graph.nodes.values() {
        node.update_html(state);
    }
}

fn count_item(vec:&Vec<ItemStack>, item_id:&ItemId, state:&S) -> f64 {
    vec.iter().map(|stack| if stack.specifier.matches(item_id, state) {stack.count} else {0.0}).sum()
}

pub fn defaults_specification() -> Specification {
    Specification {
        items: hash_map!{
            ItemId("unknown".into()) => Item {name: "Unknown Item".into(), image: None, unit: None, tags: vec![]}
        },
        recipes: hash_map!{
            RecipeId("unknown".into()) => Recipe {
                name: "Unknown Recipe".into(), image: None, machine: Specifier::None, duration: 1.0,
                inputs: vec![], outputs: vec![], tags: vec![], preprocessing: None
            }
        },
        machines: hash_map!{
            MachineId("unknown".into()) => Machine { name: "Unknown Machine".into(), image: None, tags: vec![] }
        },
        modifiers: hash_map!{}
    }
}

pub fn test_specification() -> Specification {
    Specification {
        items: hash_map!{
            ItemId("testing:a".into()) => Item {name: "Item A".into(), image: Some("../assets/testing/images/a.png".into()), unit: None, tags:vec![]},
            ItemId("testing:b".into()) => Item {name: "Item B".into(), image: Some("../assets/testing/images/b.png".into()), unit: Some("L".into()), tags:vec![]},
        },
        recipes: hash_map!{
            RecipeId("testing:b".into()) => Recipe {
                name: "Item B".into(), image: Some("../assets/testing/images/b.png".into()), machine: Specifier::Any, duration: 1.0,
                inputs: vec![ItemStack{specifier: Specifier::Is(ItemId("testing::a".into())), count: 2.0}],
                outputs: vec![ItemStack{specifier: Specifier::Is(ItemId("testing::b".into())), count: 2.0}],
                tags: vec![], preprocessing:None
            },
        },
        machines: hash_map!{
            MachineId("testing:distillery".into()) => Machine { name: "Distillery".into(), image: Some("../assets/testing/images/distillery.png".into()), tags: vec![] },
            MachineId("testing:distillery_2".into()) => Machine { name: "Distillery 2".into(), image: Some("../assets/testing/images/distillery.png".into()), 
                tags: vec![MachineTag("fast".into())]
            },
        },
        modifiers: hash_map!{
            ModifierId("testing:fast".into()) => Modifier {
                conditions: vec![ModifierCondition::Machine(Specifier::Tag(MachineTag("fast".into())))],
                effects: vec![ModifierEffect::AffectDuration(NumberEffect::Mul(0.5))],
                toggleable: false
            },
            ModifierId("testing:productivity_module".into()) => Modifier {
                conditions: vec![],
                effects: vec![
                    ModifierEffect::AffectDuration(NumberEffect::Mul(1.0/0.95)),
                    ModifierEffect::AffectOutput(Specifier::Any, NumberEffect::Function(Box::new(|args:EffectFunctionArgs| {
                        let EffectFunctionArgs::Output(recipe_ctx, output_ctx, state) = args else {panic!()}; // i know this is bad but i couldnt figure out the lifetime stuff otherwise
                        let inputs = count_item(&recipe_ctx.recipe.inputs, &output_ctx.item_stack.id, state);
                        let outputs = output_ctx.item_stack.count;
                        inputs + (outputs - inputs) * 1.04
                    }))),
                ],
                toggleable: true
            }
        }
    }
}
