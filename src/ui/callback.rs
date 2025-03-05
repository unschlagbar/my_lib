#![allow(dead_code)]
use core::ffi::c_void;
use core::mem::transmute;
use core::ptr::null_mut;
use std::ptr::null;

use super::{UiElement, UiState};

/// ErasedFnPointer can either points to a free function or associated one that
/// `&mut self`
pub struct ErasedFnPointer {
    struct_pointer: *mut c_void,
    fp: *const (),
    id: usize,
    ui: bool,
}

impl Copy for ErasedFnPointer {}
impl Clone for ErasedFnPointer {
    fn clone(&self) -> Self {
        *self
    }
}

impl ErasedFnPointer {
    pub fn from_associated<S>(struct_pointer: &mut S, fp: fn(&mut S, &mut UiElement)) -> ErasedFnPointer {
        ErasedFnPointer {
            struct_pointer: struct_pointer as *mut _ as *mut c_void,
            fp: fp as *const (),
            id: usize::MAX,
            ui: false,
        }
    }

    pub fn from_associated_ui<S>(struct_pointer: &mut S, fp: fn(&mut S, &mut UiState, &mut UiElement)) -> ErasedFnPointer {
        ErasedFnPointer {
            struct_pointer: struct_pointer as *mut _ as *mut c_void,
            fp: fp as *const (),
            id: usize::MAX,
            ui: true,
        }
    }

    pub fn from_associated_vars<S, T>(struct_pointer: &mut S, fp: fn(&mut S, &mut T)) -> ErasedFnPointer {
        ErasedFnPointer {
            struct_pointer: struct_pointer as *mut _ as *mut c_void,
            fp: fp as *const (),
            id: usize::MAX,
            ui: false
        }
    }

    pub fn from_free_vars<T>(fp: fn(&mut T)) -> ErasedFnPointer {
        ErasedFnPointer {
            struct_pointer: null_mut(),
            fp: fp as *const (),
            id: usize::MAX,
            ui: false
        }
    }
    
    pub const fn from_free(fp: fn(UiElement)) -> ErasedFnPointer {
        ErasedFnPointer {
            struct_pointer: null_mut(),
            fp: fp as *const (),
            id: usize::MAX,
            ui: false
        }
    }

    pub const fn from_id<S>(struct_pointer: &mut S, id: usize) -> ErasedFnPointer {
        ErasedFnPointer {
            struct_pointer: struct_pointer as *mut _ as *mut c_void,
            fp: null(),
            id,
            ui: false
        }
    }

    pub const fn null() -> ErasedFnPointer {
        ErasedFnPointer {
            struct_pointer: null_mut(),
            fp: null(),
            id: usize::MAX,
            ui: false
        }
    }

    pub fn is_null(&self) -> bool {
        self.fp.is_null()
    }
    
    pub fn call(&self, ui: &mut UiState, element: &mut UiElement) {
        if !self.fp.is_null() && self.id == usize::MAX {
            if self.struct_pointer.is_null() {
                let fp: fn(&mut UiElement) = unsafe { transmute::<_, fn(&mut UiElement)>(self.fp) };
                fp(element)
            } else if self.ui {
                let fp = unsafe { transmute::<_, fn(_, &mut UiState, &mut UiElement)>(self.fp) };
                fp(self.struct_pointer, ui, element)
            } else {
                let fp = unsafe { transmute::<_, fn(_, &mut UiElement)>(self.fp) };
                fp(self.struct_pointer, element)
            }
        }
    }

    pub fn call_vars<T>(&self, vars: &mut T) {
        if !self.fp.is_null() && self.id == usize::MAX {
            if self.struct_pointer.is_null() {
                let fp: fn(&mut T) = unsafe { transmute::<_, fn(&mut T)>(self.fp) };
                fp(vars)
            } else {
                let fp = unsafe { transmute::<_, fn(_, &mut T)>(self.fp) };
                fp(self.struct_pointer, vars)
            }
        } else {
            let fp: fn(id: usize, &mut T) = unsafe { transmute::<_, fn(id: usize, &mut T)>(self.fp) };
            fp(self.id, vars)
        }
    }
}