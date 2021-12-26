#![feature(associated_type_bounds)]

mod context;
pub use context::Context;

mod result;
pub use result::{Error, Result};

mod parser;
pub use parser::Parser;

mod just;
pub use just::{just, Just};

mod fail;
pub use fail::{fail, Fail};

mod any;
pub use any::Any;

pub mod consumers;
pub use consumers::*;

pub mod adapters;
