use std::{fmt::Display, error::Error, path::{Path, PathBuf}, fs};

use crate::regex::{Regex, self};

#[derive(Debug, Clone, PartialEq)]
/// The location of a [token](Token) in a file
pub struct Location {
    pub file: std::path::PathBuf,
    pub line: usize,
    pub column: usize
}

impl Location{
    pub fn line(&mut self, l:usize){ self.line = l; }
    pub fn column(&mut self, col:usize){ self.column = col; }
}

/// A trait representing the type of a [token](Token) (integer, float, keword...)
pub trait TokenKind : Copy+regex::Symbol{}

#[derive(Debug, PartialEq, Clone)]
/// A token is a lexical unit produced by a [Lexer]
pub struct Token<TokenKind> {
    /// Where the token is in a file
    pub location: Location,

    /// The [type](TokenKind) of this token
    pub kind: TokenKind,

    /// The value held by the token
    pub literal: String
}

/// A LexerNode match a set of characters into one type of [token](Token)
/// 
/// # Exemples
/// ```rust
/// use crate::neoglot_lib::{lexer::*, regex::*};
/// use std::path::Path;
/// 
/// #[derive(PartialEq, PartialOrd, Hash, Eq, Copy, Clone, Debug)]
/// enum TokenType{
///     UInt
/// }
/// 
/// impl Symbol for TokenType{}
/// impl TokenKind for TokenType{}
/// 
/// let uint_node = LexerNode::new(
///     Regex::new().then(RegexElement::Set('0', '9', Quantifier::OneOrMany)),
///     TokenType::UInt
/// );
/// 
/// let location = Location{ file: Path::new("virtual_file").to_path_buf(), line:0, column:0};
/// 
/// let candidate1 = "25+ world".chars().collect::<Vec<char>>();
/// let candidate2 = "#test".chars().collect::<Vec<char>>();
/// 
/// let result1:(&[char], Option<Token<TokenType>>) = (&['+', ' ', 'w', 'o', 'r', 'l', 'd'], Some(Token{ location: location.clone(), kind:TokenType::UInt, literal:String::from("25") }));
/// 
/// let result2:(&[char], Option<Token<TokenType>>) = (&['#', 't', 'e', 's', 't'], None);
/// 
/// 
/// assert_eq!(uint_node.tokenize(&candidate1, &location), result1);
/// assert_eq!(uint_node.tokenize(&candidate2, &location), result2);
/// 
/// ```
pub struct LexerNode<Kind:TokenKind> {
    /// The matching sequence
    regex: Regex<char>,

    /// The type of tokens to work with
    kind: Kind

}

impl<Kind:TokenKind> LexerNode<Kind>{
    pub fn new(regex: Regex<char>, kind:Kind) -> Self{ LexerNode{ regex, kind} }

    /// This function tries to construct the first token that match the matching sequence
    /// 
    /// It returns the rest of the unread characters and the [token](Token) that was found which can be [None] if no [token](Token) was found
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
/// Error type for the lexing process
pub struct LexingError{
    pub location: Location
}

impl Display for LexingError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Failed to parse token at {} {}:{}", self.location.file.display(), self.location.line, self.location.column))
    }
}

impl Error for LexingError{}

/// Result type of the lexing process
pub enum LexingResult<T:TokenKind>{
    Err(Vec<LexingError>),
    Ok(Vec<Token<T>>)
}

/// The Lexer performs a lexical analysis on characters and extract the [tokens](Token)
/// 
/// # Exemples
/// ```rust
/// use crate::neoglot_lib::{lexer::*, regex::*};
/// use std::path::{Path, PathBuf};
/// 
/// #[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Debug, Hash)]
/// enum TokenType{
///     UInt, Plus
/// }
/// 
/// impl Symbol for TokenType{}
/// impl TokenKind for TokenType{}
/// 
/// let uint_node = LexerNode::new(
///     Regex::new().then(RegexElement::Set('0', '9', Quantifier::OneOrMany)),
///     TokenType::UInt
/// );
/// 
/// let plus_node = LexerNode::new(
///     Regex::new().then(RegexElement::Item('+', Quantifier::Exactly(1))),
///     TokenType::Plus
/// );
/// 
/// let mut lexer = Lexer::<TokenType>::new();
/// lexer.register(uint_node);
/// lexer.register(plus_node);
/// 
/// let result = lexer.tokenize_content(String::from("10 +   25"), None);
/// let location = Location{ file: Path::new("virtual_file").to_path_buf(), line:0, column:0};
/// 
/// match result{
///     LexingResult::Ok(tokens) =>{
///         assert_eq!(tokens, vec![
///             Token{ location: location.clone(), kind:TokenType::UInt, literal:String::from("10") },
///             
///             Token{ location: Location{ file: Path::new("virtual_file").to_path_buf(), line:0, column:3 },
///                 kind: TokenType::Plus, literal:String::from("+")
///             },
///             
///             Token{ location: Location{ file: Path::new("virtual_file").to_path_buf(), line:0, column:7 },
///                 kind: TokenType::UInt, literal: String::from("25")
///             }
///         ]);
///     },
/// 
///     LexingResult::Err(_) => assert!(false)
/// }
/// 
/// ```
pub struct Lexer<Kind:TokenKind>{
    nodes: Vec<LexerNode<Kind>>
}

impl<Kind: TokenKind> Lexer<Kind>{
    pub fn new() -> Self {Lexer { nodes: vec![] }}

    /// Adds a [LexerNode] to this Lexer
    pub fn register(&mut self, node: LexerNode<Kind>) {
        self.nodes.push(node);
    }

    /// Extracts the [tokens](Token) from a [String]
    /// 
    /// content: The source [String] to extract the [tokens](Token) from
    /// 
    /// [path](Option<PathBuf>): The [path](PathBuf) to the file where content was taken, or [None] if it is irrelevent
    pub fn tokenize_content(&self, content:String, path:Option<PathBuf>) -> LexingResult<Kind>{
        let mut tokens:Vec<Token<Kind>> = vec![];
        let mut location = Location { file: path.unwrap_or(Path::new("virtual_file").to_path_buf()) , line: 0, column: 0 };

        let mut errors:Vec<LexingError> = vec![];

        for line_content in content.lines() {
            let mut stream = line_content.chars().collect::<Vec<char>>();

            while !stream.is_empty(){
                let mut matched = false;
                for node in &self.nodes{
                    let (others, result) = node.tokenize(&stream, &location);
                    
                    // If a token was found, add it to the list
                    // and updates location to the start of the next token

                    if let Some(token) = result{
                        location.column(location.column + token.literal.len());
                        tokens.push(token);
                        stream = others.to_vec();
                        matched = true;
                    }
                }

                if !matched{
                    if !stream[0].is_whitespace(){ errors.push(LexingError { location: location.clone() }) }

                    stream.remove(0);
                    location.column(location.column +1);
                }

                /*if !matched && stream[0].is_whitespace(){
                    // If no token was found and the current character is a whitespace
                    // Go to the next character
                    stream.remove(0);
                    location.column(location.column + 1);
                }
                else if !matched {
                    // Could not recognize the token
                    // return an error
                    errors.push(LexingError { location });
                    return Err(LexingError { location });
                }
                */
            }
            

            
            // Updates location to the start of the next line
            location.line(location.line + 1);
            location.column(0);

        }

        if !errors.is_empty(){ LexingResult::Err(errors) }
        else { LexingResult::Ok(tokens) }

    }

    /// Extracts the [tokens](Token) from a file
    /// 
    /// [path](Path): The path to the file to extract the [tokens](Token) from
    pub fn tokenize_file(&self, path: &Path) -> LexingResult<Kind>{
        let content = fs::read_to_string(path);
        let location = Location { file: path.to_path_buf(), line: 0, column: 0 };

        // Could not read the file
        if content.is_err() { return LexingResult::Err(vec![LexingError { location }]) }

        self.tokenize_content(content.unwrap(), Some(path.to_path_buf()))

        
    }
}