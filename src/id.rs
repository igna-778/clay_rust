use crate::{bindings::*, mem::zeroed_init};

#[derive(Debug, Clone, Copy)]
pub struct Id {
    inner: Clay_ElementId,
}

impl Id {
    /// Creates a clay id using the `label`
    pub fn new(label: &str) -> Self {
        Self {
            inner: unsafe { Clay__HashString(label.into(), 0, 0) },
        }
    }

    /// Creates a clay id using the `label` and the `index`
    pub fn new_index(label: &str, index: u32) -> Self {
        Self {
            inner: unsafe { Clay__HashString(label.into(), index, 0) },
        }
    }
}

impl Default for Id {
    fn default() -> Self {
        Self {
            inner: zeroed_init(),
        }
    }
}

impl From<Id> for Clay_ElementId {
    fn from(value: Id) -> Self {
        value.inner
    }
}
