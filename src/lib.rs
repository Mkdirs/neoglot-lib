//! Neoglot is a library helping creating your own programming language.

use lexer::Location;

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

/// Build an error message
pub fn build_report(message:&str, loc:Location) -> String{
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    if let Ok(file) = File::open(loc.file.clone()){
        let mut reader = BufReader::new(file);
        let mut contents = String::new();

        if reader.read_to_string(&mut contents).is_ok(){
            let line = contents.lines().nth(loc.line).unwrap();

            let end = line.len() - loc.column;
            let highlighted = highlight(line, loc.column, end);

            format!("{message} at {} {}:{}\n{highlighted}", loc.file.display(), loc.line, loc.column)
        }else{
            format!("{message} at {} {}:{}", loc.file.display(), loc.line, loc.column)
        }

    }else{
        format!("{message} at {} {}:{}", loc.file.display(), loc.line, loc.column)
    }
}

/// Reports an error message with the line of the error
pub fn report(message:&str, loc:Location){
    eprintln!("{}", build_report(message, loc));
}

/// Highlights an area under a text
fn highlight(text:&str, start:usize, end:usize) -> String{
    let size = end - start;
    format!("{text}\n{}{}", " ".repeat(start), "^".repeat(size))
}


#[cfg(test)]
mod tests;

