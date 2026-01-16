use super::*;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct MachineId(pub String);

#[derive(PartialEq, Clone, Debug)]
pub struct Machine {
    pub(super) name: String,
    pub(super) image: Option<String>,
    pub(super) tags: Vec<MachineTag>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct MachineTag(pub String);

impl Specifiable for Machine {
    type Id = MachineId;
    type Tag = MachineTag;

    fn get<'a>(machine_id:&Self::Id, state:&'a S) -> Option<&'a Self> {
        state.specification.machines.get(machine_id)
    }

    fn get_default(state:&S) -> &Self {
        state.specification.machines.get(&MachineId("unknown".to_owned())).expect("Did you forget to load the default specification?")
    }

    fn get_or_default<'a>(machine_id:&Self::Id, state:&'a S) -> &'a Self {
        state.specification.machines.get(machine_id).unwrap_or_else(|| Self::get_default(state))
    }
    
    fn has_tag(self:&Self, tag:&Self::Tag) -> bool {
        self.tags.iter().any(|check| check == tag)
    }
}
