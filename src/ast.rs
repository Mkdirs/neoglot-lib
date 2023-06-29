use std::{fmt::{Debug, Display}, error::Error};

use crate::{lexer::{TokenKind, Token}, regex::Regex};

/// A trait representing the type of an [AST]
pub trait ASTKind : PartialEq+Debug{}

#[derive(Debug, PartialEq)]
/// An Abstract Syntax Tree is a semantical unit generated from [tokens](Token)
/// 
/// children: The children of this node
/// 
/// [kind](ASTKind): The type of this AST 
pub struct AST<Kind: ASTKind>{
    pub children:Vec<AST<Kind>>,
    pub kind: Kind
}

#[derive(Debug)]
/// Error type of the parsing process
pub struct ParsingError{message: String}
impl Display for ParsingError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
impl Error for ParsingError{}

/// A ParserNode match a set of [tokens](Token) into one type of [AST]
/// 
/// [regex](Regex) is the matching sequence
/// 
/// [kind](ASTKind) the type of ast to work with
/// 
/// [parser](Fn) the closure that transforms the [tokens](Token) into an [AST]
/// 
/// # Exemples
/// ```rust
/// use crate::neoglot_lib::{lexer::*, regex::*, ast::*};
/// use std::path::Path;
/// 
/// #[derive(PartialEq, PartialOrd, Eq, Hash, Debug, Copy, Clone)]
/// enum TokenType{A, B}
/// 
/// impl Symbol for TokenType{}
/// impl TokenKind for TokenType{}
/// 
/// #[derive(PartialEq, Debug)]
/// enum ASTType{A, B}
/// 
/// impl ASTKind for ASTType{}
/// 
/// let mut tokens = vec![
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
///             kind: ASTType::A,
///             parser: Box::new(|tokens| Ok(AST{ children: vec![], kind: ASTType::A }))
///         }
///     ),
/// 
///     Box::new(
///         ParserNode{
///             regex: Regex::new().then(RegexElement::Item(TokenType::B, Quantifier::Exactly(1))),
///             kind: ASTType::B,
///             parser: Box::new(|tokens| Ok(AST{ children: vec![], kind: ASTType::B }))
///         }
///     )
/// ];
/// 
/// let parser = Parser{ nodes };
/// 
/// let result = parser.parse(tokens);
/// assert!(result.is_ok());
/// assert_eq!(result.expect("Unexpected error"), vec![
///         AST{ children: vec![], kind: ASTType::A },
///         AST{ children: vec![], kind: ASTType::A },
///         AST{ children: vec![], kind: ASTType::B },
///         AST{ children: vec![], kind: ASTType::B }
///     ]
/// );
/// 
/// ```

pub struct ParserNode<TokenT: TokenKind, ASTT: ASTKind>{
    pub regex: Regex<TokenT>,
    pub kind: ASTT,
    pub parser: Box<dyn Fn(Vec<Token<TokenT>>) -> Result<AST<ASTT>, ParsingError>>
}



impl<TokenT: TokenKind, ASTT: ASTKind> ParserNode<TokenT, ASTT>{

    pub fn parse(&self, tokens: &mut Vec<Token<TokenT>>) -> Option<Result<AST<ASTT>, ParsingError>>{
        let token_types = tokens.iter().map(|e| e.kind).collect::<Vec<TokenT>>();
        let (matched, _) = self.regex.split_first(&token_types);


        let result = if matched.is_empty(){
            None
        }else{
            Some((self.parser)(tokens[0..matched.len()].to_vec()))
        };

        for i in 0..matched.len(){
            tokens.remove(i);
        }

        result


    }
}

/// Parse a set of [tokens](Token) into a list of [AST]
/// 
/// [nodes](ParserNode) the parsing modules
pub struct Parser<TokenT: TokenKind, ASTT: ASTKind>{
    pub nodes: Vec<Box<ParserNode<TokenT, ASTT>>>
}

impl<TokenT: TokenKind, ASTT:ASTKind> Parser<TokenT, ASTT>{

    pub fn parse(&self, mut tokens:Vec<Token<TokenT>>) -> Result<Vec<AST<ASTT>>, ParsingError>{
        let mut abstract_syntaxe_forest:Vec<AST<ASTT>> = vec![];

        while !tokens.is_empty(){
            for node in &self.nodes{
                if let Some(result) = node.parse(&mut tokens){
                    match result{
                        Ok(ast) => abstract_syntaxe_forest.push(ast),
                        Err(e) => return Err(e)
                    }
                }
            }
        }
        

        Ok(abstract_syntaxe_forest)
    }
}
