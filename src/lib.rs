#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem::MaybeUninit;
use std::os::raw::{c_uchar, c_uint};

mod bindings;
use bindings::*;

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

pub enum ClaySize {
    MinMax(Clay_SizingMinMax),
    Percent(f32),
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TypedConfig {
    pub config_memory: *const u8,
    pub id: Clay_ElementId,
    pub config_type: ElementConfigType,
}

pub struct Layout {
    inner: Clay_LayoutConfig,
}

pub enum Sizing {
    Fit(f32, f32),
    Grow(f32, f32),
    Fixed(f32),
    Percent(f32),
}

impl Layout {
    pub fn new() -> Self {
        let inner = MaybeUninit::<Clay_LayoutConfig>::zeroed(); // Creates zero-initialized uninitialized memory
        let inner = unsafe { inner.assume_init() };
        Self { inner }
    }

    fn sizing(size: Sizing) -> Clay_SizingAxis {
        match size {
            Sizing::Fit(min, max) => Clay_SizingAxis {
                type_: SizingType::Fit as _,
                __bindgen_anon_1: Clay_SizingAxis__bindgen_ty_1 {
                    sizeMinMax: Clay_SizingMinMax { min, max },
                },
            },

            Sizing::Grow(min, max) => Clay_SizingAxis {
                type_: SizingType::Grow as _,
                __bindgen_anon_1: Clay_SizingAxis__bindgen_ty_1 {
                    sizeMinMax: Clay_SizingMinMax { min, max },
                },
            },

            Sizing::Fixed(size) => Clay_SizingAxis {
                type_: SizingType::Fixed as _,
                __bindgen_anon_1: Clay_SizingAxis__bindgen_ty_1 {
                    sizeMinMax: Clay_SizingMinMax {
                        min: size,
                        max: size,
                    },
                },
            },

            Sizing::Percent(percent) => Clay_SizingAxis {
                type_: SizingType::Percent as _,
                __bindgen_anon_1: Clay_SizingAxis__bindgen_ty_1 {
                    sizePercent: percent,
                },
            },
        }
    }

    fn null_id() -> Clay_ElementId {
        let inner = MaybeUninit::<Clay_ElementId>::zeroed(); // Creates zero-initialized uninitialized memory
        unsafe { inner.assume_init() }
    }

    pub fn sizing_width(&mut self, sizing: Sizing) -> &mut Self {
        self.inner.sizing.width = Self::sizing(sizing);
        self
    }

    pub fn sizing_height(&mut self, sizing: Sizing) -> &mut Self {
        self.inner.sizing.height = Self::sizing(sizing);
        self
    }

    pub fn padding(&mut self, padding: (u16, u16)) -> &mut Self {
        self.inner.padding.x = padding.0;
        self.inner.padding.y = padding.1;
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreLayoutConfig(self.inner) };

        TypedConfig {
            config_memory: memory as _,
            id: Self::null_id(),
            config_type: ElementConfigType::Layout as _,
        }
    }
}

pub struct Rectangle {
    inner: Clay_RectangleElementConfig,
}

impl Rectangle {
    pub fn new() -> Self {
        let inner = MaybeUninit::<Clay_RectangleElementConfig>::zeroed(); // Creates zero-initialized uninitialized memory
        let inner = unsafe { inner.assume_init() };
        Self { inner }
    }

    fn null_id() -> Clay_ElementId {
        let inner = MaybeUninit::<Clay_ElementId>::zeroed(); // Creates zero-initialized uninitialized memory
        unsafe { inner.assume_init() }
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
            id: Self::null_id(),
            config_type: ElementConfigType::Rectangle as _,
        }
    }
}

pub struct Clay {
    // Memory used internally by clay
    _memory: Vec<u8>,
}

impl Clay {
    pub fn new(width: f32, height: f32) -> Self {
        let memory_size = unsafe { Clay_MinMemorySize() };
        let memory = vec![0; memory_size as usize];
        unsafe {
            let arena = Clay_CreateArenaWithCapacityAndMemory(memory_size as _, memory.as_ptr() as _);
            Clay_Initialize(arena, Clay_Dimensions { width, height });
        }

        Self { _memory: memory }
    }

    pub fn begin(&self) {
        unsafe { Clay_BeginLayout() };
    }

    pub fn end(&self) -> Clay_RenderCommandArray {
        unsafe { Clay_EndLayout() }
    }

    pub fn with<F: FnOnce(&Clay), const N: usize>(&self, configs: [TypedConfig; N], f: F) {
        unsafe { Clay__OpenElement() };

        for config in configs {
            if config.config_type == ElementConfigType::Id as _ {
                unsafe { Clay__AttachId(config.id) };
            } else if config.config_type == ElementConfigType::Layout as _ {
                unsafe { Clay__AttachLayoutConfig(config.config_memory as _) };
            } else {
                unsafe {
                    Clay__AttachElementConfig(
                        // This isn't strictcly correct, but as this is a union of pointers
                        // we can cast to any of them.
                        Clay_ElementConfigUnion {
                            rectangleElementConfig: config.config_memory as _,
                        },
                        config.config_type as _,
                    )
                };
            }
        }

        unsafe { Clay__ElementPostConfiguration() };

        f(self);

        unsafe { Clay__CloseElement(); }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;

    /*
    #[test]
    fn test_create() {
        let _clay = Clay::new(800.0, 600.0);
    }
    */

    #[test]
    fn test_begin() {
        let clay = Clay::new(800.0, 600.0);

        clay.begin();

        clay.with([Layout::new()
                    .sizing_width(Sizing::Fixed(100.0))
                    .sizing_height(Sizing::Fixed(100.0))
                    .padding((10, 10))
                    .end(),
                Rectangle::new().color((255.0, 255.0, 255.0, 0.0)).end()], |clay| 
            {
                clay.with([Layout::new()
                            .sizing_width(Sizing::Fixed(100.0))
                            .sizing_height(Sizing::Fixed(100.0))
                            .padding((10, 10))
                            .end(),
                Rectangle::new().color((255.0, 255.0, 255.0, 0.0)).end()], |_clay| 
                {

                }); 
            },
        );

        // TODO: Cleanup
        let render_array = clay.end();
        let items = unsafe {
            std::slice::from_raw_parts(render_array.internalArray, render_array.length as _)
        };

        for item in items {
            if item.commandType == RenderCommandType::Rectangle as _ {
                let rectangle = unsafe { item.config.rectangleElementConfig };
                unsafe {
                    println!("{:?}", ((*rectangle).cornerRadius));
                }
            }
        }
    }

    /*
    #[test]
    fn clay_floating_attach_point_type() {
        assert_eq!(FloatingAttachPointType::LeftTop as c_uchar, 0 as c_uchar);
    }

    #[test]
    fn clay_element_config_type() {
        assert_eq!(ElementConfigType::BorderContainer as c_uchar, 2 as c_uchar);
    }

    */

    #[test]
    fn size_of_union() {
        assert_eq!(
            mem::size_of::<Clay_SizingAxis__bindgen_ty_1>(),
            mem::size_of::<Clay_SizingAxis__bindgen_ty_1>()
        )
    }
}
