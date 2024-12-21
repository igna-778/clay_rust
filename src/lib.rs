#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
mod bindings;

use bindings::*;
use libc::{c_uchar, c_uint};

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub enum ClayElementConfigType {
    Rectangle,
    BorderContainer,
    FloatingContainer,
    ScrollContainer,
    Image,
    Text,
    Custom,
}

impl ClayElementConfigType {
    pub fn as_c_uchar(&self) -> c_uchar {
        match self {
            ClayElementConfigType::Rectangle => {
                Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_RECTANGLE
            }
            ClayElementConfigType::BorderContainer => {
                Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_BORDER_CONTAINER
            }
            ClayElementConfigType::FloatingContainer => {
                Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_FLOATING_CONTAINER
            }
            ClayElementConfigType::ScrollContainer => {
                Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_SCROLL_CONTAINER
            }
            ClayElementConfigType::Image => Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_IMAGE,
            ClayElementConfigType::Text => Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_TEXT,
            ClayElementConfigType::Custom => {
                Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_CUSTOM
            }
        }
    }
}

pub enum ClayLayoutDirection {
    LeftToRight,
    TopToBottom,
}

impl ClayLayoutDirection {
    pub fn as_c_uchar(&self) -> c_uchar {
        match self {
            ClayLayoutDirection::LeftToRight => Clay_LayoutDirection_CLAY_LEFT_TO_RIGHT,
            ClayLayoutDirection::TopToBottom => Clay_LayoutDirection_CLAY_TOP_TO_BOTTOM,
        }
    }
}

pub enum ClayLayoutAlignmentX {
    Left,
    Center,
    Right,
}

impl ClayLayoutAlignmentX {
    pub fn as_c_uchar(&self) -> c_uchar {
        match self {
            ClayLayoutAlignmentX::Left => Clay_LayoutAlignmentX_CLAY_ALIGN_X_LEFT,
            ClayLayoutAlignmentX::Center => Clay_LayoutAlignmentX_CLAY_ALIGN_X_CENTER,
            ClayLayoutAlignmentX::Right => Clay_LayoutAlignmentX_CLAY_ALIGN_X_RIGHT,
        }
    }
}
pub enum ClayLayoutAlignmentY {
    Top,
    Center,
    Bottom,
}

impl ClayLayoutAlignmentY {
    pub fn as_c_uchar(&self) -> c_uchar {
        match self {
            ClayLayoutAlignmentY::Top => Clay_LayoutAlignmentY_CLAY_ALIGN_Y_TOP,
            ClayLayoutAlignmentY::Center => Clay_LayoutAlignmentY_CLAY_ALIGN_Y_CENTER,
            ClayLayoutAlignmentY::Bottom => Clay_LayoutAlignmentY_CLAY_ALIGN_Y_BOTTOM,
        }
    }
}

pub enum ClayTextElementConfigWrapMode {
    Words,
    Newline,
    None,
}

impl ClayTextElementConfigWrapMode {
    pub fn as_c_uint(&self) -> c_uint {
        match self {
            ClayTextElementConfigWrapMode::Words => {
                Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_WORDS
            }
            ClayTextElementConfigWrapMode::Newline => {
                Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_NEWLINES
            }
            ClayTextElementConfigWrapMode::None => {
                Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_NONE
            }
        }
    }
}

pub enum ClayRenderCommandType {
    None,
    Rectangle,
    Border,
    Text,
    Image,
    ScissorStart,
    ScissorEnd,
    Custom,
}

impl ClayRenderCommandType {
    pub fn as_c_uint(&self) -> c_uint {
        match self {
            ClayRenderCommandType::None => Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_NONE,
            ClayRenderCommandType::Rectangle => {
                Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_RECTANGLE
            }
            ClayRenderCommandType::Border => Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_BORDER,
            ClayRenderCommandType::Text => Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_TEXT,
            ClayRenderCommandType::Image => Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_IMAGE,
            ClayRenderCommandType::ScissorStart => {
                Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_START
            }
            ClayRenderCommandType::ScissorEnd => {
                Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_END
            }

            ClayRenderCommandType::Custom => Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_CUSTOM,
        }
    }
}
