#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
mod bindings;

use std::os::raw::{c_uchar, c_uint};

use bindings::*;

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//use this if using the version compiled by the build script. This library is using a copy of bindings.rs in this directory

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ElementConfigType {
    Rectangle = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_RECTANGLE,
    BorderContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_BORDER_CONTAINER,
    FloatingContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_FLOATING_CONTAINER,
    ScrollContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_SCROLL_CONTAINER,
    Image = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_IMAGE,
    Text = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_TEXT,
    Custom = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_CUSTOM,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum LayoutDirection {
    LeftToRight = Clay_LayoutDirection_CLAY_LEFT_TO_RIGHT,
    TopToBottom = Clay_LayoutDirection_CLAY_TOP_TO_BOTTOM,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum LayoutAlignmentX {
    Left = Clay_LayoutAlignmentX_CLAY_ALIGN_X_LEFT,
    Center = Clay_LayoutAlignmentX_CLAY_ALIGN_X_CENTER,
    Right = Clay_LayoutAlignmentX_CLAY_ALIGN_X_RIGHT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum LayoutAlignmentY {
    Top = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_TOP,
    Center = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_CENTER,
    Bottom = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_BOTTOM,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum SizingType {
    Fit = Clay__SizingType_CLAY__SIZING_TYPE_FIT,
    Grow = Clay__SizingType_CLAY__SIZING_TYPE_GROW,
    Percent = Clay__SizingType_CLAY__SIZING_TYPE_PERCENT,
    Fixed = Clay__SizingType_CLAY__SIZING_TYPE_FIXED,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum TextElementConfigWrapMode {
    Words = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_WORDS,
    Newline = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_NEWLINES,
    None = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_NONE,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum RenderCommandType {
    None = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_NONE,
    Rectangle = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_RECTANGLE,
    Border = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_BORDER,
    Text = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_TEXT,
    Image = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_IMAGE,
    ScissorStart = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_START,
    ScissorEnd = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_END,
    Custom = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_CUSTOM,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum PointerCaptureMode {
    Capture = Clay_PointerCaptureMode_CLAY_POINTER_CAPTURE_MODE_CAPTURE,
    Passthrough = Clay_PointerCaptureMode_CLAY_POINTER_CAPTURE_MODE_PASSTHROUGH,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum FloatingAttachPointType {
    LeftTop = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_TOP,
    LeftCenter = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_CENTER,
    LeftBottom = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_BOTTOM,
    CenterTop = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_TOP,
    CenterCenter = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_CENTER,
    CenterBottom = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_BOTTOM,
    RightTop = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_TOP,
    RightCenter = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_CENTER,
    RightBottom = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_BOTTOM,
}
pub enum ClaySize{
    MinMax(Clay_SizingMinMax),
    Percent(f32)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clay_floating_attach_point_type() {
        assert_eq!(FloatingAttachPointType::LeftTop as c_uchar, 0 as c_uchar);
    }
    #[test]
    fn clay_element_config_type() {
        assert_eq!(ElementConfigType::BorderContainer as c_uchar, 2 as c_uchar);
    }
}
