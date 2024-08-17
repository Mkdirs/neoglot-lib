use std::{error::Error, fmt::Display, fs, hash::Hash};

use pattern_matcher::{MatchAgainst, MatchingPipeline, PipelineError, StringDigester, Symbol, TerminatedPipeline};

use crate::{/*regex::{/*Regex,*/ self},*/ build_report};

#[derive(Debug, Clone, PartialEq)]
/// The location of a [token](Token) in a file
pub struct Location {
    pub file: String,
    pub line: usize,
    pub column: usize
}

impl Location{
    pub fn line(&mut self, l:usize){ self.line = l; }
    pub fn column(&mut self, col:usize){ self.column = col; }
}

/// A trait representing the type of a [token](Token) (integer, float, keword...)
pub trait TokenKind : PartialEq+Eq+Hash+Copy+Symbol{}

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

impl<T:TokenKind> Symbol for Token<T>{}

impl<T:TokenKind> Symbol for &Token<T>{}

/// A LexerNode match a set of characters into one type of [token](Token)
/// 
/// # Exemples
/// ```rust
/// use crate::neoglot_lib::{lexer::*};
/// use pattern_matcher::*;
/// use std::path::Path;
/// 
/// #[derive(PartialEq, PartialOrd, Hash, Eq, Copy, Clone, Debug)]
/// enum TokenType{
///     UInt
/// }
/// impl Symbol for TokenType{}
/// impl TokenKind for TokenType{}
/// 
/// let uint_node = LexerModule::new(TokenType::UInt, |pipeline| {
///     Ok(
///         pipeline
///         .with_quantifier(AtLeast(1), |p| p.expect_any_of(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))?
///         .terminate()
///     )
///     
/// 
/// });
/// 
/// let mut location1 = Location{ file: "virtual_file".to_string(), line:0, column:0};
/// let mut location2 = Location{ file: "virtual_file".to_string(), line:1, column:0};
/// 
/// let candidate1 = String::from("25");
/// let candidate2 = String::from("#test");
/// 
/// let result1 = Some( (Token{ location: location1.clone(), kind:TokenType::UInt, literal: "25".to_string() }, 2) );
/// 
/// let result2 = None;
/// 
/// 
/// assert_eq!(uint_node.tokenize(&candidate1, &location1), result1);
/// assert_eq!(uint_node.tokenize(&candidate2, &location2), result2);
/// 
/// ```
pub struct LexerModule<'a, Kind:TokenKind>
{
    /// The matcher of this module
    matcher: Box<dyn Fn(MatchingPipeline<char>) -> Result<TerminatedPipeline<char>, PipelineError<'a, char>>>,

    /// The type of tokens to work with
    kind: Kind

}

impl<'a, Kind:TokenKind> LexerModule<'a, Kind>
{
    pub fn new(kind:Kind, matcher: impl (Fn(MatchingPipeline<char>) -> Result<TerminatedPipeline<char>, PipelineError<'a, char>>) +'static) -> Self
    { LexerModule{ matcher: Box::new(matcher), kind} }


    /// This function tries to construct the first token that match the given string
    /// 
    /// It returns the [token](Token) that was found which can be [None] if no [token](Token) was found
    pub fn tokenize(&self, line: &str, location: &Location) -> Option<(Token<Kind>, usize)>{

        if let Some(pipeline) = line.match_against(&self.matcher){
            let offset = pipeline.offset();

            let literal = pipeline.digest::<StringDigester>();

            let token = Token { location: location.clone(), kind: self.kind, literal };


            return Some( (token, offset) );
        }

        None
    }
}

#[derive(Debug, PartialEq)]
/// Error type for the lexing process
pub struct LexingError{
    pub location: Location
}

impl Display for LexingError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = build_report("Failed to parse token", self.location.clone());
        f.write_str(&msg)
    }
}

impl Error for LexingError{}

/// Result type of the lexing process
pub type LexingResult<T> = Result<Vec<Token<T>>, LexingError>;


/// The Lexer performs a lexical analysis on characters and extract the [tokens](Token)
/// 
/// # Exemples
/// ```rust
/// use crate::neoglot_lib::{lexer::*};
/// use pattern_matcher::*;
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
/// let uint_node = LexerModule::new(TokenType::UInt, |pipeline| {
///     Ok(
///         pipeline
///         .with_quantifier(AtLeast(1), |p| p.expect_any_of(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))?
///         .terminate()
///     )
/// });
/// 
/// let plus_node = LexerModule::new(TokenType::Plus, |pipeline| Ok(pipeline.expect_symbol(&'+')?.terminate()));
/// 
/// let mut lexer = Lexer::<TokenType>::new();
/// lexer.register(uint_node);
/// lexer.register(plus_node);
/// 
/// let result = lexer.tokenize_content(String::from("10 +   25"), "");
/// let location = Location{ file: String::new(), line:0, column:0};
/// 
/// match result{
///     Ok(tokens) =>{
///         assert_eq!(tokens, vec![
///             Token{ location: location.clone(), kind:TokenType::UInt, literal:String::from("10") },
///             
///             Token{ location: Location{ file: String::new(), line:0, column:3 },
///                 kind: TokenType::Plus, literal:String::from("+")
///             },
///             
///             Token{ location: Location{ file: String::new(), line:0, column:7 },
///                 kind: TokenType::UInt, literal: String::from("25")
///             }
///         ]);
///     },
/// 
///     Err(_) => assert!(false)
/// }
/// 
/// ```
pub struct Lexer<'a, Kind:TokenKind>
{
    modules: Vec<LexerModule<'a, Kind>>
}

impl<'a, Kind: TokenKind> Lexer<'a, Kind>
{
    pub fn new() -> Self {Lexer { modules: vec![], }}

    /// Adds a [LexerNode] to this Lexer
    pub fn register(&mut self, module: LexerModule<'a, Kind>) {
        self.modules.push(module);
    }

    /// Extracts the [tokens](Token) from a [String]
    /// 
    /// content: The source [String] to extract the [tokens](Token) from
    /// 
    /// path: The path to the file where content was taken
    pub fn tokenize_content(&self, content:String, path: &str) -> LexingResult<Kind> {

        let lines = content.lines().collect::<Vec<&str>>();
        let mut tokens = vec![];
        let mut location = Location { file: path.to_string(), line: 0, column: 0 };

        
    
        for indx in 0..lines.len() {
            let mut line = lines[indx];
            location.line = indx;
            location.column = 0;
            

            loop {
                let mut parsed_a_token = false;
                

                for module in &self.modules {
                    if let Some( (token, offset) ) = module.tokenize(&line, &location) {
                        tokens.push(token);
                        line = line.get(offset..).unwrap_or_default();
                        location.column += offset;
                        parsed_a_token = true;
                        break;
                    }
                }
                //break;
                if line.is_empty() {
                    break;
                }

                if line.chars().collect::<Vec<char>>()[0].is_whitespace(){
                    location.column += 1;
                    line = line.get(1..).unwrap_or_default();
                    continue;
                }

                if !parsed_a_token {
                    return Err(LexingError{location});
                }
            }
        }

        Ok(tokens)

    }

    /// Extracts the [tokens](Token) from a file
    /// 
    /// path: The path to the file to extract the [tokens](Token) from
    pub fn tokenize_file(&self, path: &str) -> LexingResult<Kind>{
        let content = fs::read_to_string(path);
        let location = Location { file: path.to_string(), line: 0, column: 0 };

        // Could not read the file
        if content.is_err() { return Err(LexingError { location }) }

        self.tokenize_content(content.unwrap(), path)

        
    }
}
