use std::{ffi::c_void, marker::PhantomData};

use crate::{bindings::*, mem::zeroed_init, TypedConfig};

use super::ElementConfigType;

pub struct Custom<Data> {
    inner: Clay_CustomElementConfig,
    phantom: PhantomData<Data>,
}

impl<Data> Custom<Data> {
    pub fn new() -> Self {
        Self {
            inner: zeroed_init(),
            phantom: PhantomData,
        }
    }

    pub fn data(&mut self, data: *const Data) -> &mut Self {
        self.inner.customData = data as *mut c_void;
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreCustomElementConfig(self.inner) };

        TypedConfig {
            config_memory: memory as _,
            id: zeroed_init(),
            config_type: ElementConfigType::Image as _,
        }
    }
}
