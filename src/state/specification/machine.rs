use super::*;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct MachineId(pub String);

#[derive(PartialEq, Clone)]
pub struct Machine {
    pub(super) name: String,
    pub(super) image: Option<String>,
    pub(super) tags: Vec<MachineTag>,
}

#[derive(PartialEq, Eq, Clone)]
pub struct MachineTag(pub String);

impl Specifiable for Machine {
    type Id = MachineId;
    type Tag = MachineTag;

    fn get<'a>(machine_id:&Self::Id, state:&'a S) -> Option<&'a Self> {
        state.specification.machines.get(machine_id)
    }
    
    fn has_tag(self:&Self, tag:&Self::Tag) -> bool {
        self.tags.iter().any(|check| check == tag)
    }
}
