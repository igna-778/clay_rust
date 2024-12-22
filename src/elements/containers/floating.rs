use crate::{
    bindings::*,
    elements::{ElementConfigType, PointerCaptureMode},
    math::{Dimensions, Vector2},
    mem::zeroed_init,
    TypedConfig,
};

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

pub struct FloatingContainer {
    inner: Clay_FloatingElementConfig,
}

impl FloatingContainer {
    pub fn new() -> Self {
        Self {
            inner: zeroed_init(),
        }
    }

    pub fn offset(&mut self, offset: Vector2) -> &mut Self {
        self.inner.offset = offset.into();
        self
    }

    pub fn dimensions(&mut self, dimensions: Dimensions) -> &mut Self {
        self.inner.expand = dimensions.into();
        self
    }

    pub fn z_index(&mut self, index: u16) -> &mut Self {
        self.inner.zIndex = index;
        self
    }

    pub fn parent_id(&mut self, id: u32) -> &mut Self {
        self.inner.parentId = id;
        self
    }

    pub fn attachment(
        &mut self,
        element: FloatingAttachPointType,
        parent: FloatingAttachPointType,
    ) -> &mut Self {
        self.inner.attachment = Clay_FloatingAttachPoints {
            element: element as _,
            parent: parent as _,
        };
        self
    }

    pub fn pointer_capture_mode(&mut self, mode: PointerCaptureMode) -> &mut Self {
        self.inner.pointerCaptureMode = mode as _;
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreFloatingElementConfig(self.inner) };

        TypedConfig {
            config_memory: memory as _,
            id: zeroed_init(),
            config_type: ElementConfigType::FloatingContainer as _,
        }
    }
}
