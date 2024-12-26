use crate::bindings::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ErrorType {
    TextMeasurementFunctionNotProvided =
        Clay_ErrorType_CLAY_ERROR_TYPE_TEXT_MEASUREMENT_FUNCTION_NOT_PROVIDED,
    ArenaCapacityExceeded = Clay_ErrorType_CLAY_ERROR_TYPE_ARENA_CAPACITY_EXCEEDED,
    ElementsCapacityExceeded = Clay_ErrorType_CLAY_ERROR_TYPE_ELEMENTS_CAPACITY_EXCEEDED,
    TextMeasurementCapacityExceeded =
        Clay_ErrorType_CLAY_ERROR_TYPE_TEXT_MEASUREMENT_CAPACITY_EXCEEDED,
    DuplicateId = Clay_ErrorType_CLAY_ERROR_TYPE_DUPLICATE_ID,
    FloatingContainerParentNotFound =
        Clay_ErrorType_CLAY_ERROR_TYPE_FLOATING_CONTAINER_PARENT_NOT_FOUND,
    InternalError = Clay_ErrorType_CLAY_ERROR_TYPE_INTERNAL_ERROR,
}

#[derive(Debug, Clone, Copy)]
pub struct Error<'a> {
    pub type_: ErrorType,
    pub text: &'a str,
}

impl From<Clay_ErrorData> for Error<'_> {
    fn from(value: Clay_ErrorData) -> Self {
        Self {
            type_: unsafe { core::mem::transmute::<u32, ErrorType>(value.errorType) },
            text: value.errorText.into(),
        }
    }
}
