use crate::{
    bindings::*,
    elements::{
        containers::border::BorderContainer, custom::Custom, image::Image, rectangle::Rectangle,
        text::Text,
    },
    math::BoundingBox,
};

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

#[derive(Debug, Clone)]
pub enum RenderCommandConfig {
    None(),
    Rectangle(Rectangle),
    Border(BorderContainer),
    Text(String, Text),
    Image(Image),
    ScissorStart(),
    ScissorEnd(),
    Custom(Custom),
}

impl From<&Clay_RenderCommand> for RenderCommandConfig {
    fn from(value: &Clay_RenderCommand) -> Self {
        match unsafe { std::mem::transmute(value.commandType) } {
            RenderCommandType::None => Self::None(),
            RenderCommandType::Rectangle => Self::Rectangle(Rectangle::from(unsafe {
                (&mut *(value.config.rectangleElementConfig)).to_owned()
            })),
            RenderCommandType::Border => Self::Border(BorderContainer::from(unsafe {
                (&mut *(value.config.borderElementConfig)).to_owned()
            })),
            RenderCommandType::Text => Self::Text(
                <Clay_String as Into<&str>>::into(value.text).to_string(),
                Text::from(unsafe { (&mut *(value.config.textElementConfig)).to_owned() }),
            ),
            RenderCommandType::Image => Self::Image(Image::from(unsafe {
                (&mut *(value.config.imageElementConfig)).to_owned()
            })),
            RenderCommandType::ScissorStart => Self::ScissorStart(),
            RenderCommandType::ScissorEnd => Self::ScissorEnd(),
            RenderCommandType::Custom => Self::Custom(Custom::from(unsafe {
                (&mut *(value.config.customElementConfig)).to_owned()
            })),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderCommand {
    pub id: u32,
    pub bounding_box: BoundingBox,
    pub config: RenderCommandConfig,
}

impl From<Clay_RenderCommand> for RenderCommand {
    fn from(value: Clay_RenderCommand) -> Self {
        Self {
            id: value.id,
            bounding_box: value.boundingBox.into(),
            config: (&value).into(),
        }
    }
}
