//! Lingua compiler library.

// Lets test helpers shared with tests/ (tests/support) name this crate `lingua`
#[cfg(test)]
extern crate self as lingua;

pub mod ast;
pub mod build;
pub mod check;
pub mod eval;
pub mod lex;
pub mod parse;
pub mod provider;
pub mod report;
pub mod tir;
