use crate::bindings::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 255.0 }
    }
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Clay_Color> for Color {
    fn from(value: Clay_Color) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
impl From<Color> for Clay_Color {
    fn from(value: Color) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
