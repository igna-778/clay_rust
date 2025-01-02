use crate::bindings::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Id<'a> {
    pub id: u32,
    pub offset: u32,
    pub base_id: u32,
    pub string_id: &'a str,
}

impl<'a> Id<'a> {
    /// Creates a clay id using the `label`
    pub fn new(label: &'a str) -> Self {
        unsafe { Clay__HashString(label.into(), 0, 0) }.into()
    }

    /// Creates a clay id using the `label` and the `index`
    pub fn new_index(label: &'a str, index: u32) -> Self {
        unsafe { Clay__HashString(label.into(), index, 0) }.into()
    }
}

impl From<Id<'_>> for Clay_ElementId {
    fn from(value: Id) -> Self {
        Self {
            id: value.id,
            offset: value.offset,
            baseId: value.base_id,
            stringId: value.string_id.into(),
        }
    }
}

impl From<Clay_ElementId> for Id<'_> {
    fn from(value: Clay_ElementId) -> Self {
        Self {
            id: value.id,
            offset: value.offset,
            base_id: value.baseId,
            string_id: value.stringId.into(),
        }
    }
}

impl<'a> From<&'a str> for Id<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value)
    }
}
