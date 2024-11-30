#![allow(dead_code)]
use core::ffi::c_void;
use core::mem::transmute;
use core::ptr::null_mut;
use std::ptr::null;

use super::UiState;

/// ErasedFnPointer can either points to a free function or associated one that
/// `&mut self`
pub struct ErasedFnPointer {
    struct_pointer: *mut c_void,
    fp: *const (),
}

impl Copy for ErasedFnPointer {}
impl Clone for ErasedFnPointer {
    fn clone(&self) -> Self {
        *self
    }
}

impl ErasedFnPointer {
    pub fn from_associated<S>(struct_pointer: &mut S, fp: fn(&mut S, &mut UiState)) -> ErasedFnPointer {
        ErasedFnPointer {
            struct_pointer: struct_pointer as *mut _ as *mut c_void,
            fp: fp as *const (),
        }
    }
    
    pub const fn from_free(fp: fn(UiState)) -> ErasedFnPointer {
        ErasedFnPointer {
            struct_pointer: null_mut(),
            fp: fp as *const (),
        }
    }

    pub const fn null() -> ErasedFnPointer {
        ErasedFnPointer {
            struct_pointer: null_mut(),
            fp: null(),
        }
    }
    
    pub fn call(&self, ui: &mut UiState) {
        if !self.fp.is_null() {
            if self.struct_pointer.is_null() {
                let fp: fn(&mut UiState) = unsafe { transmute::<_, fn(&mut UiState)>(self.fp) };
                fp(ui)
            } else {
                let fp = unsafe { transmute::<_, fn(_, &mut UiState)>(self.fp) };
                fp(self.struct_pointer, ui)
            }
        }
    }
}

#[test]
fn main() {
    let mut ui = UiState::create(vec![], vec![], true);
    let erased_ptr = ErasedFnPointer::from_free(|_ui| {
        println!("Hello, {}", 10);
    });
    erased_ptr.call(&mut ui);
    
    println!("size_of_val(erased_ptr) = {}", core::mem::size_of_val(&erased_ptr));

    let mut gg = Test { x: 1 };

    let callback = ErasedFnPointer::from_associated(
        &mut gg,
        Test::f
    );

    callback.call(&mut ui);
    
}

struct Test {
    x: i32
}
impl Test {
    fn f(&mut self, _ui: &mut UiState) -> () {
        let z = self.x;
        println!("Hello from Test, {}", z);
    }
}