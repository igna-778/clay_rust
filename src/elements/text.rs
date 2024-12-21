use crate::{bindings::*, mem::zeroed_init};

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum TextElementConfigWrapMode {
    Words = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_WORDS,
    Newline = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_NEWLINES,
    None = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_NONE,
}

pub struct TextElementConfig {
    inner: *mut Clay_TextElementConfig,
}

impl Into<*mut Clay_TextElementConfig> for TextElementConfig {
    fn into(self) -> *mut Clay_TextElementConfig {
        self.inner
    }
}

pub struct Text {
    inner: Clay_TextElementConfig,
}

impl Text {
    pub fn new() -> Self {
        Self {
            inner: zeroed_init(),
        }
    }

    pub fn color(&mut self, color: (f32, f32, f32, f32)) -> &mut Self {
        self.inner.textColor = Clay_Color {
            r: color.0,
            g: color.1,
            b: color.2,
            a: color.3,
        };
        self
    }

    pub fn font_size(&mut self, size: u16) -> &mut Self {
        self.inner.fontSize = size;
        self
    }

    pub fn font_id(&mut self, id: u16) -> &mut Self {
        self.inner.fontId = id;
        self
    }
    
    pub fn letter_spacing(&mut self, spacing: u16) -> &mut Self {
        self.inner.letterSpacing = spacing;
        self
    }
    
    pub fn line_height(&mut self, height: u16) -> &mut Self {
        self.inner.lineHeight = height;
        self
    }

    pub fn wrap_mode(&mut self, mode: TextElementConfigWrapMode) -> &mut Self {
        self.inner.wrapMode = mode as u32;
        self
    }

    pub fn end(&self) -> TextElementConfig {
        let memory = unsafe { Clay__StoreTextElementConfig(self.inner) };

        TextElementConfig { inner: memory }
    }
}
