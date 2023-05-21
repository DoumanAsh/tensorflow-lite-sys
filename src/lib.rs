//!Tensorflow-lite bindings

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern "C" {
    ///The TensorFlow Lite Runtime version
    pub fn TfLiteVersion() -> *const i8;
}
