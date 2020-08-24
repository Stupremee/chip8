#![deny(rust_2018_idioms)]
#![warn(clippy::pedantic)]

pub mod instruction;

/// A index into the register table.
pub type Register = u8;
