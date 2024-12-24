use crate::bindings::*;

pub mod containers;
pub mod custom;
pub mod image;
pub mod rectangle;
pub mod text;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ElementConfigType {
    Rectangle = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_RECTANGLE,
    BorderContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_BORDER_CONTAINER,
    FloatingContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_FLOATING_CONTAINER,
    ScrollContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_SCROLL_CONTAINER,
    Image = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_IMAGE,
    Text = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_TEXT,
    Custom = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_CUSTOM,
    // Rust specific enum types (uses the same approach as Odin bindings)
    Id = 65,
    Layout = 66,
}

pub enum CornerRadius {
    All(f32),
    Individual {
        top_left: f32,
        top_right: f32,
        bottom_left: f32,
        bottom_right: f32,
    },
}

impl Into<Clay_CornerRadius> for CornerRadius {
    fn into(self) -> Clay_CornerRadius {
        match self {
            CornerRadius::All(radius) => Clay_CornerRadius {
                topLeft: radius,
                topRight: radius,
                bottomLeft: radius,
                bottomRight: radius,
            },
            CornerRadius::Individual {
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
