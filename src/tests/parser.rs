use std::vec;

use crate::{lexer::*, parser::*, regex::{Symbol, Regex, RegexElement, Quantifier}};

#[derive(Debug, Hash, Clone, Copy, PartialOrd, PartialEq, Eq)]
enum TokenType{
    BlockBegin,
    BlockEnd,
    A,B
}

impl Symbol for TokenType{}
impl TokenKind for TokenType{}

#[test]
fn dangling_block_end(){
    let tokens = &[
        Token{
            kind:TokenType::A,
            literal: "A".to_string(),
            location: Location { file: "".to_string(), line: 0, column: 0 }
        },

        Token{
            kind:TokenType::B,
            literal: "B".to_string(),
            location: Location { file: "".to_string(), line: 1, column: 2 }
        },

        Token{
            kind:TokenType::BlockEnd,
            literal: "}".to_string(),
            location: Location { file: "".to_string(), line: 1, column: 3 }
        }
    ];

    let mut parser = Parser::new(TokenType::BlockBegin, TokenType::BlockEnd);

    let nodes = vec![
        Box::new(
            ParserNode{
                regex: Regex::new().then(RegexElement::Item(TokenType::A, Quantifier::Exactly(1))),
                parser: Box::new(|_, _| ParsingResult::Ok(vec![AST{ kind:TokenType::A, children: vec![] }]))
            }
        ),

        Box::new(
            ParserNode{
                regex: Regex::new().then(RegexElement::Item(TokenType::B, Quantifier::Exactly(1))),
                parser: Box::new(|_, _| ParsingResult::Ok(vec![AST{ kind:TokenType::B, children: vec![] }]))
            }
        )
    ];

    parser.nodes = nodes;
    let result = parser.parse(tokens);

    match result{
        ParsingResult::Ok(_) => assert!(false),
        ParsingResult::Err(errs) => {
            assert_eq!(errs, vec![
                ParsingError::UnexpectedToken { expected: None, got: Some(TokenType::BlockEnd), location: Location { file: "".to_string(), line: 1, column: 3 } },
                ParsingError::UnparsedSequence(Location { file: "".to_string(), line: 1, column: 3 })
            ])
        }
    }
}


#[test]
// Note: Compter les ouvertures/fermetures de block au lieu d'utiliser des regex
fn block_parsing(){

    let tokens = &[
       Token{
            kind:TokenType::A,
            literal: "A".to_string(),
            location: Location { file: "".to_string(), line: 0, column: 0 }
        },

        Token{
            kind:TokenType::B,
            literal: "B".to_string(),
            location: Location { file: "".to_string(), line: 0, column: 2 }
        },

        Token{
            kind:TokenType::BlockBegin,
            literal: "{".to_string(),
            location: Location { file: "".to_string(), line: 0, column: 3 }
        },

        Token{
            kind:TokenType::A,
            literal: "A".to_string(),
            location: Location { file: "".to_string(), line: 1, column: 0 }
        },

        Token{
            kind:TokenType::B,
            literal: "B".to_string(),
            location: Location { file: "".to_string(), line: 1, column: 2 }
        },

        Token{
            kind:TokenType::BlockBegin,
            literal: "{".to_string(),
            location: Location { file: "".to_string(), line: 1, column: 3 }
        },

        Token{
            kind:TokenType::B,
            literal: "B".to_string(),
            location: Location { file: "".to_string(), line: 1, column: 4 }
        },


        Token{
            kind:TokenType::BlockEnd,
            literal: "}".to_string(),
            location: Location { file: "".to_string(), line: 1, column: 5 }
        },

        Token{
            kind:TokenType::A,
            literal: "A".to_string(),
            location: Location { file: "".to_string(), line: 2, column: 0 }
        },

        Token{
            kind:TokenType::BlockEnd,
            literal: "}".to_string(),
            location: Location { file: "".to_string(), line: 3, column: 0 }
        },
    ];

    let mut parser = Parser::new(TokenType::BlockBegin, TokenType::BlockEnd);

    let nodes = vec![
        Box::new(
            ParserNode{
                regex: Regex::new().then(RegexElement::Item(TokenType::A, Quantifier::Exactly(1))),
                parser: Box::new(|_, _| ParsingResult::Ok(vec![AST{ kind:TokenType::A, children: vec![] }]))
            }
        ),

        Box::new(
            ParserNode{
                regex: Regex::new().then(RegexElement::Item(TokenType::B, Quantifier::Exactly(1))),
                parser: Box::new(|_, _| ParsingResult::Ok(vec![AST{ kind:TokenType::B, children: vec![] }]))
            }
        )
    ];

    parser.nodes = nodes;
    let result = parser.parse(tokens);

    match result{
        ParsingResult::Err(_) => assert!(false),
        ParsingResult::Ok(forest) => {
            assert_eq!(forest, vec![
                AST{ kind: TokenType::A, children: vec![] },
                AST{ kind: TokenType::B, children: vec![] },
                AST{ kind: TokenType::BlockBegin, children: vec![
                    AST{ kind: TokenType::A, children: vec![] },
                    AST{ kind: TokenType::B, children: vec![] },
                    AST{ kind: TokenType::BlockBegin, children: vec![
                        AST{ kind: TokenType::B, children: vec![] },
                        AST{ kind: TokenType::BlockEnd, children:vec![] }
                    ] },
                    AST{ kind: TokenType::A, children: vec![] },
                    AST{ kind: TokenType::BlockEnd, children: vec![] }
                ] }
            ], "left is: {:#?}", forest);
        }
    }

}