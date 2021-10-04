// std
pub(crate) use std::fmt::{self, Display, Formatter};

// dependencies
pub(crate) use js_sys::Math::random;
pub(crate) use wasm_bindgen::prelude::*;

// structs and enums
pub use crate::life::{Cell, Universe};
