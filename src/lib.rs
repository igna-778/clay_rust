#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
mod bindings;

use std::os::raw::{c_uchar, c_uint};

use bindings::*;

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//use this if using the version compiled by the build script. This library is using a copy of bindings.rs in this directory

#[repr(u8)]
pub enum ClayElementConfigType {
    Rectangle = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_RECTANGLE,
    BorderContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_BORDER_CONTAINER,
    FloatingContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_FLOATING_CONTAINER,
    ScrollContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_SCROLL_CONTAINER,
    Image = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_IMAGE,
    Text = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_TEXT,
    Custom = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_CUSTOM,
}

#[repr(u8)]
pub enum ClayLayoutDirection {
    LeftToRight = Clay_LayoutDirection_CLAY_LEFT_TO_RIGHT,
    TopToBottom = Clay_LayoutDirection_CLAY_TOP_TO_BOTTOM,
}

#[repr(u8)]
pub enum ClayLayoutAlignmentX {
    Left = Clay_LayoutAlignmentX_CLAY_ALIGN_X_LEFT,
    Center = Clay_LayoutAlignmentX_CLAY_ALIGN_X_CENTER,
    Right = Clay_LayoutAlignmentX_CLAY_ALIGN_X_RIGHT,
}

#[repr(u8)]
pub enum ClayLayoutAlignmentY {
    Top = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_TOP,
    Center = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_CENTER,
    Bottom = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_BOTTOM,
}

#[repr(u8)]
pub enum ClaySizingType {
    Fit = Clay__SizingType_CLAY__SIZING_TYPE_FIT,
    Grow = Clay__SizingType_CLAY__SIZING_TYPE_GROW,
    Percent = Clay__SizingType_CLAY__SIZING_TYPE_PERCENT,
    Fixed = Clay__SizingType_CLAY__SIZING_TYPE_FIXED,
}

#[repr(u32)]
pub enum ClayTextElementConfigWrapMode {
    Words = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_WORDS,
    Newline = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_NEWLINES,
    None = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_NONE,
}

#[repr(u32)]
pub enum ClayRenderCommandType {
    None = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_NONE,
    Rectangle = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_RECTANGLE,
    Border = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_BORDER,
    Text = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_TEXT,
    Image = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_IMAGE,
    ScissorStart = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_START,
    ScissorEnd = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_END,
    Custom = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_CUSTOM,
}

#[repr(u32)]
pub enum ClayPointerCaptureMode {
    Capture = Clay_PointerCaptureMode_CLAY_POINTER_CAPTURE_MODE_CAPTURE,
    Passthrough = Clay_PointerCaptureMode_CLAY_POINTER_CAPTURE_MODE_PASSTHROUGH,
}
