#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
mod bindings;

use std::os::raw::{c_uchar, c_uint};

use bindings::*;

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//use this if using the version compiled by the build script. This library is using a copy of bindings.rs in this directory

pub enum ClayElementConfigType {
    Rectangle = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_RECTANGLE as isize,
    BorderContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_BORDER_CONTAINER as isize,
    FloatingContainer =
        Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_FLOATING_CONTAINER as isize,
    ScrollContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_SCROLL_CONTAINER as isize,
    Image = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_IMAGE as isize,
    Text = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_TEXT as isize,
    Custom = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_CUSTOM as isize,
}

pub enum ClayLayoutDirection {
    LeftToRight = Clay_LayoutDirection_CLAY_LEFT_TO_RIGHT as isize,
    TopToBottom = Clay_LayoutDirection_CLAY_TOP_TO_BOTTOM as isize,
}

pub enum ClayLayoutAlignmentX {
    Left = Clay_LayoutAlignmentX_CLAY_ALIGN_X_LEFT as isize,
    Center = Clay_LayoutAlignmentX_CLAY_ALIGN_X_CENTER as isize,
    Right = Clay_LayoutAlignmentX_CLAY_ALIGN_X_RIGHT as isize,
}

pub enum ClayLayoutAlignmentY {
    Top = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_TOP as isize,
    Center = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_CENTER as isize,
    Bottom = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_BOTTOM as isize,
}

pub enum ClaySizingType {
    Fit = Clay__SizingType_CLAY__SIZING_TYPE_FIT as isize,
    Grow = Clay__SizingType_CLAY__SIZING_TYPE_GROW as isize,
    Percent = Clay__SizingType_CLAY__SIZING_TYPE_PERCENT as isize,
    Fixed = Clay__SizingType_CLAY__SIZING_TYPE_FIXED as isize,
}

pub enum ClayTextElementConfigWrapMode {
    Words = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_WORDS as isize,
    Newline = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_NEWLINES as isize,
    None = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_NONE as isize,
}

pub enum ClayRenderCommandType {
    None = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_NONE as isize,
    Rectangle = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_RECTANGLE as isize,
    Border = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_BORDER as isize,
    Text = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_TEXT as isize,
    Image = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_IMAGE as isize,
    ScissorStart = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_START as isize,
    ScissorEnd = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_END as isize,
    Custom = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_CUSTOM as isize,
}

pub enum ClayPointerCaptureMode {
    Capture = Clay_PointerCaptureMode_CLAY_POINTER_CAPTURE_MODE_CAPTURE as isize,
    Passthrough = Clay_PointerCaptureMode_CLAY_POINTER_CAPTURE_MODE_PASSTHROUGH as isize,
}
