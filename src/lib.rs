#[cfg(test)]
#[macro_use]
extern crate hamcrest;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::engine::Context;
use crate::engine::engine::Engine;

mod engine;
mod errors;
mod js;

pub fn init_engine(context: Context) -> Engine {
    Engine::new(context)
}

#[no_mangle]
pub extern "C" fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };
    CString::new("Hello ".to_owned() + recipient)
        .unwrap()
        .into_raw()
}

pub fn init_context(data_path: &str) -> Context {
    Context::new(data_path)
}
