use std::{fmt::Display, error::Error, path::{Path, PathBuf}, fs};

use crate::regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub file: std::path::PathBuf,
    pub line: usize,
    pub column: usize
}

impl Location{
    pub fn line(&mut self, l:usize){ self.line = l; }
    pub fn column(&mut self, col:usize){ self.column = col; }
}



#[derive(Debug, PartialEq)]
pub struct Token<Kind:PartialEq+Copy> {
    pub location: Location,
    pub kind: Kind, 
    pub literal: String
}

pub struct Lexernode<Kind:PartialEq+Copy> {
    regex: Regex<char>,
    kind: Kind

}

impl<Kind:PartialEq+Copy> Lexernode<Kind>{
    pub fn new(regex: Regex<char>, kind:Kind) -> Self{ Lexernode{ regex, kind} }

    pub fn tokenize<'a>(&self, c:&'a [char], location: &Location) -> (&'a [char], Option<Token<Kind>>){
        let (matched, others) = self.regex.split_first(c);
        let token = if matched.is_empty() { None } else {
            let literal = matched.iter().collect::<String>();
            Some(Token{ location: location.clone(), kind: self.kind, literal})
        };

        (others, token)
    }
}

#[derive(Debug, PartialEq)]
pub struct LexingError{
    pub location: Location
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

    pub fn tokenize_content(&self, content:String, path:Option<PathBuf>) -> Result<Vec<Token<Kind>>, LexingError>{
        let mut tokens:Vec<Token<Kind>> = vec![];
        let mut location = Location { file: path.unwrap_or(Path::new("virtual_file").to_path_buf()) , line: 0, column: 0 };



        for line_content in content.lines() {
            let mut stream = line_content.chars().collect::<Vec<char>>();

            while !stream.is_empty(){
                let mut matched = false;
                for node in &self.nodes{
                    let (others, result) = node.tokenize(&stream, &location);
    
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

    pub fn tokenize_file(&self, path: &Path) -> Result<Vec<Token<Kind>>, LexingError>{
        let content = fs::read_to_string(path);
        let location = Location { file: path.to_path_buf(), line: 0, column: 0 };

        if content.is_err() { return Err(LexingError { location }) }

        self.tokenize_content(content.unwrap(), Some(path.to_path_buf()))

        
    }
}