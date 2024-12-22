use crate::{bindings::*, elements::ElementConfigType, mem::zeroed_init, TypedConfig};

pub struct ScrollContainer {
    inner: Clay_ScrollElementConfig,
}

impl ScrollContainer {
    pub fn new() -> Self {
        Self {
            inner: zeroed_init(),
        }
    }

    pub fn horizontal(&mut self) -> &mut Self {
        self.inner.horizontal = true;
        self
    }

    pub fn vertical(&mut self) -> &mut Self {
        self.inner.vertical = true;
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreScrollElementConfig(self.inner) };

        TypedConfig {
            config_memory: memory as _,
            id: zeroed_init(),
            config_type: ElementConfigType::ScrollContainer as _,
        }
    }
}
