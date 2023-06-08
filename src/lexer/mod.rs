use std::{fmt::Display, error::Error, path::Path, fs};

use crate::regex::Regex;

#[derive(Debug, Clone)]
pub struct Location {
    file: std::path::PathBuf,
    line: usize,
    column: usize
}

impl Location{
    pub fn line(&mut self, l:usize){ self.line = l; }
    pub fn column(&mut self, col:usize){ self.column = col; }
}



#[derive(Debug)]
pub struct Token<Kind:PartialEq+Copy> {
    location: Location,
    kind: Kind, 
    literal: String
}

pub struct Lexernode<Kind:PartialEq+Copy> {
    regex: Regex<char>,
    kind: Kind

}

impl<Kind:PartialEq+Copy> Lexernode<Kind>{
    pub fn new(regex: Regex<char>, kind:Kind) -> Self{ Lexernode{ regex, kind} }

    pub fn tokenize<'a>(&self, c:&'a [char], location: Location) -> (&'a [char], Option<Token<Kind>>){
        let (matched, others) = self.regex.split_first(c);
        let token = if matched.is_empty() { None } else {
            let literal = matched.iter().collect::<String>();
            Some(Token{ location, kind: self.kind, literal})
        };

        (others, token)
    }
}

#[derive(Debug)]
pub struct LexingError{
    location: Location
}

impl Display for LexingError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Failed to parse token at {} {}:{}", self.location.file.display(), self.location.line, self.location.column))
    }
}


impl Error for LexingError{
    
}

pub struct Lexer<Kind:PartialEq+Copy>{
    nodes: Vec<Lexernode<Kind>>
}

impl<Kind: PartialEq+Copy> Lexer<Kind>{
    pub fn new() -> Self {Lexer { nodes: vec![] }}

    pub fn register(&mut self, node: Lexernode<Kind>) {
        self.nodes.push(node);
    }

    pub fn tokenize(&self, path: &Path) -> Result<Vec<Token<Kind>>, LexingError>{
        let mut tokens:Vec<Token<Kind>> = vec![];
        let content = fs::read_to_string(path);
        let mut location = Location { file: path.to_path_buf(), line: 0, column: 0 };

        if content.is_err() { return Err(LexingError { location }) }



        for line_content in content.unwrap().lines() {
            let mut stream = line_content.chars().collect::<Vec<char>>();

            while !stream.is_empty(){
                let mut matched = false;
                for node in &self.nodes{
                    let (others, result) = node.tokenize(&stream, location.clone());
    
                    if let Some(token) = result{
                        location.column(location.column + token.literal.len());
                        tokens.push(token);
                        stream = others.to_vec();
                        matched = true;
                    }
                }

                if !matched && stream[0].is_whitespace(){
                    stream.remove(0);
                    location.column(location.column + 1);
                }
                else if !matched {
                    return Err(LexingError { location });
                }
            }
            

            

            location.line(location.line + 1);
            location.column(0);

        }

        Ok(tokens)
    }
}