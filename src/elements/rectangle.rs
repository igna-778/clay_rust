use crate::{bindings::*, color::Color, mem::zeroed_init, TypedConfig};

use super::{CornerRadius, ElementConfigType};

pub struct Rectangle {
    inner: Clay_RectangleElementConfig,
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            inner: zeroed_init(),
        }
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.inner.color = color.into();
        self
    }

    pub fn corner_radius(&mut self, radius: CornerRadius) -> &mut Self {
        self.inner.cornerRadius = radius.into();
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreRectangleElementConfig(self.inner) };

        TypedConfig {
            config_memory: memory as _,
            id: zeroed_init(),
            config_type: ElementConfigType::Rectangle as _,
        }
    }
}
