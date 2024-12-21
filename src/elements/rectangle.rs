use crate::{bindings::*, mem::zeroed_init, TypedConfig};

use super::ElementConfigType;

pub struct Rectangle {
    inner: Clay_RectangleElementConfig,
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            inner: zeroed_init()
        }
    }

    pub fn color(&mut self, color: (f32, f32, f32, f32)) -> &mut Self {
        self.inner.color = Clay_Color {
            r: color.0,
            g: color.1,
            b: color.2,
            a: color.3,
        };
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