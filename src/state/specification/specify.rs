use super::*;

pub trait Specifiable: Sized {
    type Id: PartialEq + Eq + Hash + Clone + Debug;
    type Tag: PartialEq + Eq + Clone + Debug;

    fn get<'a>(id:&Self::Id, state:&'a S) -> Option<&'a Self>;
    fn get_or_default<'a>(id:&Self::Id, state:&'a S) -> &'a Self;
    fn get_default(state:&S) -> &Self;
    fn has_tag(self:&Self, tag:&Self::Tag) -> bool; 
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Specifier<T: Specifiable> {
    Any,
    None,
    Is(T::Id),
    Isnt(T::Id),
    Tag(T::Tag),
    NotTag(T::Tag),
    TagsNone(Vec<T::Tag>),
    TagsNotNone(Vec<T::Tag>),
    TagsAll(Vec<T::Tag>),
    TagsNotAll(Vec<T::Tag>),
}

impl<T: Specifiable> Specifier<T> {
    pub(super) fn matches(self:&Self, check_id:&T::Id, state:&S) -> bool {
        let Some(check) = T::get(check_id, state) else {return false;};
        match self {
            Specifier::Any => true,
            Specifier::None => false,
            Specifier::Is(id) => check_id == id,
            Specifier::Isnt(id) => check_id != id,
            Specifier::Tag(tag) => check.has_tag(tag),
            Specifier::NotTag(tag) => !check.has_tag(tag),
            Specifier::TagsNone(tags) => !tags.iter().any(|tag| check.has_tag(tag)),
            Specifier::TagsNotNone(tags) => tags.iter().any(|tag| check.has_tag(tag)),
            Specifier::TagsAll(tags) => tags.iter().all(|tag| check.has_tag(tag)),
            Specifier::TagsNotAll(tags) => !tags.iter().all(|tag| check.has_tag(tag)),
        }
    }
}
