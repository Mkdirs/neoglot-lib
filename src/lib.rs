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

            let size = line.len() - loc.column;
            let highlighted = highlight(line, loc.column, size);

            format!("{message} at {} {}:{}\n{highlighted}", loc.file, loc.line+1, loc.column+1)
        }else{
            format!("{message} at {} {}:{}", loc.file, loc.line+1, loc.column+1)
        }

    }else{
        format!("{message} at {} {}:{}", loc.file, loc.line+1, loc.column+1)
    }
}

/// Reports an error message with the line of the error
pub fn report(message:&str, loc:Location){
    eprintln!("{}", build_report(message, loc));
}

/// Highlights an area under a text
fn highlight(text:&str, start:usize, size:usize) -> String{
    format!("{text}\n{}{}", " ".repeat(start), "^".repeat(size))
}

#[test]
fn test_highlight(){
    let txt = "Hello W0rld !";
    let highlighted = highlight(txt, 6, 1);

    assert_eq!(&highlighted, "Hello W0rld !\n      ^")
}


#[cfg(test)]
mod tests;

