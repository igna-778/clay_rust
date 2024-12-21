use crate::{bindings::*, mem::zeroed_init, TypedConfig};

use super::ElementConfigType;

pub enum RectangleCornerRadius {
    All(f32),
    Individual {
        top_left: f32,
        top_right: f32,
        bottom_left: f32,
        bottom_right: f32,
    },
}

impl Into<Clay_CornerRadius> for RectangleCornerRadius {
    fn into(self) -> Clay_CornerRadius {
        match self {
            RectangleCornerRadius::All(radius) => Clay_CornerRadius {
                topLeft: radius,
                topRight: radius,
                bottomLeft: radius,
                bottomRight: radius,
            },
            RectangleCornerRadius::Individual {
                top_left,
                top_right,
                bottom_left,
                bottom_right,
            } => Clay_CornerRadius {
                topLeft: top_left,
                topRight: top_right,
                bottomLeft: bottom_left,
                bottomRight: bottom_right,
            },
        }
    }
}

pub struct Rectangle {
    inner: Clay_RectangleElementConfig,
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            inner: zeroed_init(),
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

    pub fn corner_radius(&mut self, radius: RectangleCornerRadius) -> &mut Self {
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
