/// Special module for expression parsing
pub mod expression;

use std::{fmt::{Debug, Display}, error::Error};

use crate::{lexer::{TokenKind, Token, Location}, regex::Regex};


#[derive(Debug, PartialEq, Clone)]
/// An Abstract Syntax Tree is a semantical unit
pub struct AST<T:PartialEq+Clone>{
    /// The type of this AST
    pub kind: T,
    pub children:Vec<AST<T>>
}

#[derive(Debug, Clone, PartialEq)]
/// Error type of the parsing process
pub enum ParsingError<T:TokenKind>{
    /// Groups are not closed properly
    InvalidGroups(Location),

    /// Could not parse a sequence of tokens
    UnparsedSequence(Location),

    /// A block wasn't closed properly
    UnclosedBlock(Location),

    /// Self explanatory
    UnexpectedToken{
        expected: Option<T>,
        got: Option<T>,
        location: Location
    },

    /// No tokens provided
    NoTokens
}
impl<T:TokenKind> Display for ParsingError<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}
impl<T:TokenKind> Error for ParsingError<T>{}

/// Result type of the parsing process
pub type ParsingResult<T, E> = Result<AST<T>, ParsingError<E>>;


/// A ParserNode match a set of [tokens](Token) into one type of [AST]
/// 
/// # Exemples
/// ```rust
/// use crate::neoglot_lib::{lexer::*, regex::*, parser::*};
/// use std::path::Path;
/// 
/// #[derive(PartialEq, PartialOrd, Eq, Hash, Debug, Copy, Clone)]
/// enum TokenType{A, B, BlockS, BlockE}
/// 
/// impl Symbol for TokenType{}
/// impl TokenKind for TokenType{}
/// 
/// 
/// let tokens = &[
///     Token{
///         location: Location{ file: String::from("file"), line: 0, column: 0 },
///         kind: TokenType::A,
///         literal: String::from("a")
///     },
/// 
///     Token{
///         location: Location{ file: String::from("file"), line: 0, column: 2 },
///         kind: TokenType::A,
///         literal: String::from("a")
///     },
/// 
///     Token{
///         location: Location{ file: String::from("file"), line: 1, column: 0 },
///         kind: TokenType::B,
///         literal: String::from("b")
///     },
/// 
///     Token{
///         location: Location{ file: String::from("file"), line: 2, column: 0 },
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
/// let mut parser = Parser::new(tokens);
/// parser.nodes = nodes;
/// 
/// let mut forest:Vec<AST<TokenType>> = vec![];
/// let mut errors:Vec<ParsingError<TokenType>> = vec![];
/// 
/// while !parser.finished(){
///     match parser.parse_with_node(){
///         Ok(ast) => forest.push(ast),
///         Err(e) => {
///             errors.push(e);
///             parser.skip(1);
///         }
///     }
/// 
/// }
/// 
/// assert!(errors.is_empty());
/// 
/// 
/// assert_eq!(forest, vec![
///     AST{ children: vec![], kind: TokenType::A },
///     AST{ children: vec![], kind: TokenType::A },
///     AST{ children: vec![], kind: TokenType::B },
///     AST{ children: vec![], kind: TokenType::B },
/// ]);
/// 
/// ```

pub struct ParserNode<T: TokenKind, ASTNode:PartialEq+Clone>{
    /// The matching sequence
    pub regex: Regex<T>,

    /// The closure that transforms the [tokens](Token) into an [AST] ([Fn])
    pub parser: Box<dyn Fn(&[Token<T>]) -> ParsingResult<ASTNode, T>>
}



impl<T: TokenKind, ASTNode:PartialEq+Clone> ParserNode<T, ASTNode>{

    pub fn parse(&self, tokens: &mut &[Token<T>]) -> Option<ParsingResult<ASTNode, T>>{
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

/// Gives a ParsingError if kind is None or if it is not equals to expected
/// 
/// kind: The TokenKind got
/// 
/// expected: The expected TokenKind
/// 
/// location: The location where this assertion happened
/// 
pub fn expect<T:TokenKind>(kind:Option<T>, expected:T, location:Location) -> Result<(), ParsingError<T>>{
    if kind.is_none(){
        return Err(ParsingError::UnexpectedToken {
            expected: Some(expected), got: None, location
        });
    }
    if kind.unwrap() != expected{
        return Err(ParsingError::UnexpectedToken {
            expected: Some(expected), got: kind, location
        });
    }

    Ok(())
}


/// Parse a set of [tokens](Token) into a list of [AST]
pub struct Parser<'a, T: TokenKind, ASTNode:PartialEq+Clone>{
    /// Tokens to parse
    tokens: &'a [Token<T>],

    /// The parsing modules
    pub nodes: Vec<Box<ParserNode<T, ASTNode>>>
}

impl<'a, T: TokenKind, ASTNode:PartialEq+Clone> Parser<'a, T, ASTNode>{

    pub fn new(tokens: &'a[Token<T>]) -> Self{ Parser { tokens, nodes: vec![] } }

    /// Parse with the first [ParserNode] that match the current sequence of tokens
    pub fn parse_with_node(&mut self) -> ParsingResult<ASTNode, T>{

        if self.finished(){
            return Err(ParsingError::NoTokens);
        }

        for node in &self.nodes{
                
            if let Some(result) = node.parse(&mut self.tokens){
                return result;
                /*match result{
                    ParsingResult::Ok(frst) => {
                        for ast in frst{
                            abstract_syntax_forest.push(ast);
                        }
                    },
                    Err(errs) => {
                        for e in errs{ errors.push(e); }

                        // Theoretically could panic if tokens is empty
                        // The loop condition should prevent that from happening
                        self.tokens = &self.tokens[1..];
                    }
                }*/
            }

        }

        return Err(ParsingError::UnparsedSequence(self.tokens[0].location.clone()));

        
    }

    /// Skips *num* numbers of tokens if possible
    pub fn skip(&mut self, num: usize){
        if let Some(t) = self.tokens.get(num..){
            self.tokens = t;
        }
    }

    /// Pops the current token out of the parser and return it or None
    pub fn pop(&mut self) -> Option<&Token<T>>{
        if self.finished() { return None; }

        let t = &self.tokens[0];
        self.tokens = &self.tokens[1..];
        Some(t)
        
    }

    /// Returns the current token or None
    pub fn peek(&self) -> Option<&Token<T>>{
        self.tokens.get(0)
    }

    /// returns the token at index *i* or None
    pub fn peek_at(&self, i:usize) -> Option<&Token<T>>{
        self.tokens.get(i)
    }

    /// Returns true if all tokens have been consumed
    pub fn finished(&self) -> bool{ self.tokens.is_empty() }

    /// Returns true if the current token is of type *kind*
    pub fn on_token(&self, kind:T) -> bool{
        if self.finished(){ return false; }

        self.peek().unwrap().kind == kind
    }


    /// Slices a block out of the tokens for further parsing
    /// 
    /// The opening and last closing tokens are omitted
    pub fn slice_block(&self, begin:T, end:T) -> Result<&'a[Token<T>], ParsingError<T>>{

        let mut open_blocks = 1;
        let mut i = 1;
        let mut last_block_end = 0;

        if self.finished(){ return Ok(&[]); }

        if let Err(e) = expect(Some(self.tokens[0].kind), begin, self.tokens[0].location.clone()){
            return Err(e);
        }

        while i < self.tokens.len() && open_blocks != 0{
            let token = self.peek_at(i).unwrap();

            if token.kind == begin { open_blocks += 1; }
            else if token.kind == end{
                open_blocks -=1;

                if open_blocks == 0 { last_block_end = i; }
            }

            i += 1;
        }

        if open_blocks == 0{
            Ok(&self.tokens[1..last_block_end])
        }else{
            Err(ParsingError::UnclosedBlock(self.tokens[0].location.clone()))
        }

    }

}
