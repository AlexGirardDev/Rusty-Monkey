#![allow(dead_code)]

mod builtin;
pub mod environment;
pub mod eval;
mod eval_error;
mod node;
pub mod object;
#[cfg(test)]
mod tests;
