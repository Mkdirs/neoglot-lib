/// Special module for expression parsing
pub mod expression;

use std::{fmt::{Debug, Display}, error::Error};

use crate::{lexer::{TokenKind, Token, Location}, regex::Regex};


#[derive(Debug, PartialEq)]
/// An Abstract Syntax Tree is a semantical unit generated from [tokens](Token)
/// 
/// children: The children of this node
/// 
/// [kind](ASTKind): The type of this AST 
pub struct AST<T:TokenKind>{
    pub children:Vec<AST<T>>,
    pub kind: T
}

#[derive(Debug)]
/// Error type of the parsing process
pub enum ParsingError{
    /// Groups are not closed properly
    InvalidGroups(Location),

    /// Could not parse a token
    UnparsedToken(String, Location)
}
impl Display for ParsingError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}
impl Error for ParsingError{}

/// Result type of the parsing process
#[derive(Debug)]
pub enum ParsingResult<T: TokenKind>{
    Ok(Vec<AST<T>>),
    Err(Vec<ParsingError>)
}

/// A ParserNode match a set of [tokens](Token) into one type of [AST]
/// 
/// # Exemples
/// ```rust
/// use crate::neoglot_lib::{lexer::*, regex::*, parser::*};
/// use std::path::Path;
/// 
/// #[derive(PartialEq, PartialOrd, Eq, Hash, Debug, Copy, Clone)]
/// enum TokenType{A, B}
/// 
/// impl Symbol for TokenType{}
/// impl TokenKind for TokenType{}
/// 
/// 
/// let tokens = &[
///     Token{
///         location: Location{ file: Path::new("file").to_path_buf(), line: 0, column: 0 },
///         kind: TokenType::A,
///         literal: String::from("a")
///     },
/// 
///     Token{
///         location: Location{ file: Path::new("file").to_path_buf(), line: 0, column: 2 },
///         kind: TokenType::A,
///         literal: String::from("a")
///     },
/// 
///     Token{
///         location: Location{ file: Path::new("file").to_path_buf(), line: 1, column: 0 },
///         kind: TokenType::B,
///         literal: String::from("b")
///     },
/// 
///     Token{
///         location: Location{ file: Path::new("file").to_path_buf(), line: 2, column: 0 },
///         kind: TokenType::B,
///         literal: String::from("b")
///     }
/// ];
/// 
/// let nodes = vec![
///     Box::new(
///         ParserNode{
///             regex: Regex::new().then(RegexElement::Item(TokenType::A, Quantifier::Exactly(1))),
///             parser: Box::new(|tokens| Ok(AST{ children: vec![], kind: TokenType::A }))
///         }
///     ),
/// 
///     Box::new(
///         ParserNode{
///             regex: Regex::new().then(RegexElement::Item(TokenType::B, Quantifier::Exactly(1))),
///             parser: Box::new(|tokens| Ok(AST{ children: vec![], kind: TokenType::B }))
///         }
///     )
/// ];
/// 
/// let parser = Parser{ nodes };
/// 
/// let result = parser.parse(tokens);
/// 
/// 
/// match result{
///     ParsingResult::Ok(forest) => {
///         assert_eq!(forest, vec![
///             AST{ children: vec![], kind: TokenType::A },
///             AST{ children: vec![], kind: TokenType::A },
///             AST{ children: vec![], kind: TokenType::B },
///             AST{ children: vec![], kind: TokenType::B },
///         ]);
///     },
/// 
///     ParsingResult::Err(_) => assert!(false)
/// }
/// 
/// ```

pub struct ParserNode<T: TokenKind>{
    /// The matching sequence
    pub regex: Regex<T>,

    /// The closure that transforms the [tokens](Token) into an [AST] ([Fn])
    pub parser: Box<dyn Fn(&[Token<T>]) -> Result<AST<T>, ParsingError>>
}



impl<T: TokenKind> ParserNode<T>{

    pub fn parse(&self, tokens: &mut &[Token<T>]) -> Option<Result<AST<T>, ParsingError>>{
        let token_types = tokens.iter().map(|e| e.kind).collect::<Vec<T>>();
        let (matched, _) = self.regex.split_first(&token_types);


        let result = if matched.is_empty(){
            None
        }else{
            Some((self.parser)(&tokens[0..matched.len()]))
        };

        *tokens = &tokens[matched.len()..];

        result


    }
}

/// Parse a set of [tokens](Token) into a list of [AST]
pub struct Parser<T: TokenKind>{
    /// The parsing modules
    pub nodes: Vec<Box<ParserNode<T>>>
}

impl<T: TokenKind> Parser<T>{

    pub fn parse(&self, mut tokens:&[Token<T>]) -> ParsingResult<T>{
        let mut abstract_syntax_forest:Vec<AST<T>> = vec![];
        let mut errors:Vec<ParsingError> = vec![];

        while !tokens.is_empty(){
            for node in &self.nodes{
                if let Some(result) = node.parse(&mut tokens){
                    match result{
                        Ok(ast) => abstract_syntax_forest.push(ast),
                        Err(e) => {
                            errors.push(e);

                            // Theoretically could panic if tokens is empty
                            // The loop condition should prevent that from happening
                            tokens = &tokens[1..];
                        }
                    }
                }
            }
        }

        
        if !errors.is_empty(){
            ParsingResult::Err(errors)
        }else{
            ParsingResult::Ok(abstract_syntax_forest)
        }
    }
}
