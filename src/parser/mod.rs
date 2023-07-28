/// Special module for expression parsing
pub mod expression;

use std::{fmt::{Debug, Display}, error::Error};

use crate::{lexer::{TokenKind, Token, Location}, regex::Regex};


#[derive(Debug, PartialEq, Clone)]
/// An Abstract Syntax Tree is a semantical unit generated from [tokens](Token)
/// 
/// [kind](ASTKind): The type of this AST
/// 
/// children: The children of this node
pub struct AST<T:TokenKind>{
    pub kind: T,
    pub children:Vec<AST<T>>
}

#[derive(Debug, Clone)]
/// Error type of the parsing process
pub enum ParsingError<T:TokenKind>{
    /// Groups are not closed properly
    InvalidGroups(Location),

    /// Could not parse a token
    UnparsedToken(Token<T>),

    /// A block wasn't closed properly
    UnclosedBlock(Location),

    // Self explanatory
    UnexpectedToken{
        expected: T,
        got: T,
        location: Location
    }
}
impl<T:TokenKind> Display for ParsingError<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}
impl<T:TokenKind> Error for ParsingError<T>{}

/// Result type of the parsing process
#[derive(Debug)]
pub enum ParsingResult<T: TokenKind>{
    Ok(Vec<AST<T>>),
    Err(Vec<ParsingError<T>>)
}

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
///             parser: Box::new(|tokens, _| ParsingResult::Ok(vec![AST{ children: vec![], kind: TokenType::A }]))
///         }
///     ),
/// 
///     Box::new(
///         ParserNode{
///             regex: Regex::new().then(RegexElement::Item(TokenType::B, Quantifier::Exactly(1))),
///             parser: Box::new(|tokens, _| ParsingResult::Ok(vec![AST{ children: vec![], kind: TokenType::B }]))
///         }
///     )
/// ];
/// 
/// let parser = Parser{ nodes, block_start: TokenType::BlockS, block_end: TokenType::BlockE };
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
    pub parser: Box<dyn Fn(&[Token<T>], &Parser<T>) -> ParsingResult<T>>
}



impl<T: TokenKind> ParserNode<T>{

    pub fn parse(&self, tokens: &mut &[Token<T>], p:&Parser<T>) -> Option<ParsingResult<T>>{
        let token_types = tokens.iter().map(|e| e.kind).collect::<Vec<T>>();
        let (matched, _) = self.regex.split_first(&token_types);


        let result = if matched.is_empty(){
            None
        }else{
            Some((self.parser)(&tokens[0..matched.len()], p))
        };

        *tokens = &tokens[matched.len()..];

        result


    }
}

/// Parse a set of [tokens](Token) into a list of [AST]
pub struct Parser<T: TokenKind>{
    /// The parsing modules
    pub nodes: Vec<Box<ParserNode<T>>>,

    // The start of a block
    pub block_start: T,

    // The end of a block
    pub block_end: T
}

impl<T: TokenKind> Parser<T>{

    pub fn new(block_start: T, block_end: T) -> Self{ Parser { nodes: vec![], block_start, block_end } }

    pub fn parse(&self, mut tokens:&[Token<T>]) -> ParsingResult<T>{
        let mut abstract_syntax_forest:Vec<AST<T>> = vec![];
        let mut errors:Vec<ParsingError<T>> = vec![];

        while !tokens.is_empty(){

            if &tokens[0].kind == &self.block_start{
                match self.slice_block(tokens){
                    Err(e) => {
                        errors.push(e);
                        tokens = &tokens[1..];
                    },
                    Ok(tok) => {
                        let mut block = AST{ kind: self.block_start, children: vec![] };

                        match self.parse(&tok[1..tok.len()-1]){
                            ParsingResult::Err(errs) =>{
                                for e in errs { errors.push(e); }

                            },

                            ParsingResult::Ok(frst) => {
                                block.children = frst;
                                block.children.push(AST { kind: self.block_end, children: vec![] });

                                abstract_syntax_forest.push(block);
                            }
                        }

                        tokens = &tokens.get(tok.len()..).unwrap_or_default();
                    }
                }
                //continue;
            }

            for node in &self.nodes{
                
                if let Some(result) = node.parse(&mut tokens, self){
                    match result{
                        ParsingResult::Ok(frst) => {
                            for ast in frst{
                                abstract_syntax_forest.push(ast);
                            }
                        },
                        ParsingResult::Err(errs) => {
                            for e in errs{ errors.push(e); }

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

    /// Slices a block out of the tokens for further parsing.
    pub fn slice_block<'a>(&self, tokens:&'a[Token<T>]) -> Result<&'a[Token<T>], ParsingError<T>>{

        let mut open_blocks = 1;
        let mut i = 1;
        let mut last_block_end = 0;

        if tokens.is_empty() { return Ok(&[]); }

        if &tokens[0].kind != &self.block_start { return Err(
            ParsingError::UnexpectedToken{
                expected: self.block_start,
                got: tokens[0].kind,
                location: tokens[0].location.clone()
            });
        }

        while i<tokens.len() && open_blocks != 0{
            let token = &tokens[i];

            if token.kind == self.block_start { open_blocks += 1; }
            else if token.kind == self.block_end {
                open_blocks -=1;

                if open_blocks == 0 { last_block_end = i; }
            }

            i += 1;
        }

        if open_blocks == 0{
            Ok(&tokens[0..last_block_end+1])
        }else{
            Err(ParsingError::UnclosedBlock(tokens[0].location.clone()))
        }

    }

}
