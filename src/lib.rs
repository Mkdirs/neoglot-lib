//! Neoglot is a library helping creating your own programming language.

/// A module for building abstract regular expressions
/// 
/// Build regular expressions with any types you want
pub mod regex;

/// Lexical analysis module
/// 
/// Extract tokens from files
pub mod lexer;

/// Semantical analysis module
/// 
/// Extracts Abstract Syntax Trees from tokens
pub mod parser;


#[cfg(test)]
mod tests;

