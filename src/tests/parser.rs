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

    let mut parser = Parser::new(tokens);

    let nodes = vec![
        Box::new(
            ParserNode{
                regex: Regex::new().then(RegexElement::Item(TokenType::A, Quantifier::Exactly(1))),
                parser: Box::new(|_| Ok(AST{ kind:TokenType::A, children: vec![] }))
            }
        ),

        Box::new(
            ParserNode{
                regex: Regex::new().then(RegexElement::Item(TokenType::B, Quantifier::Exactly(1))),
                parser: Box::new(|_| Ok(AST{ kind:TokenType::B, children: vec![] }))
            }
        )
    ];

    parser.nodes = nodes;
    let mut last_error:Option<ParsingError<TokenType>> = None;
    while !parser.finished(){
        match parser.parse_with_node(){
            Ok(_ast) => {},
            Err(e) => {
                last_error = Some(e);
                parser.skip(1);
            }
        }
    }

    assert_eq!(last_error, Some(ParsingError::UnparsedSequence(
        Location { file: "".to_string(), line: 1, column: 3 }
    )));
    /*let result = parser.parse(tokens);

    match result{
        ParsingResult::Ok(_) => assert!(false),
        ParsingResult::Err(errs) => {
            assert_eq!(errs, vec![
                ParsingError::UnexpectedToken { expected: None, got: Some(TokenType::BlockEnd), location: Location { file: "".to_string(), line: 1, column: 3 } },
                ParsingError::UnparsedSequence(Location { file: "".to_string(), line: 1, column: 3 })
            ])
        }
    }*/
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


    fn init_nodes() -> Vec<Box<ParserNode<TokenType, TokenType>>>{
        vec![
            Box::new(
                ParserNode{
                    regex: Regex::new().then(RegexElement::Item(TokenType::A, Quantifier::Exactly(1))),
                    parser: Box::new(|_| Ok(AST{ kind:TokenType::A, children: vec![] }))
                }
            ),

            Box::new(
                ParserNode{
                    regex: Regex::new().then(RegexElement::Item(TokenType::B, Quantifier::Exactly(1))),
                    parser: Box::new(|_| Ok(AST{ kind:TokenType::B, children: vec![] }))
                }
            )
        ]
    }


    fn parse(mut parser:Parser<TokenType, TokenType>) -> Result<Vec<AST<TokenType>>, Vec<ParsingError<TokenType>>>{
        let mut forest:Vec<AST<TokenType>> = vec![];
        let mut errors:Vec<ParsingError<TokenType>> = vec![];

        parser.nodes = init_nodes();

        while !parser.finished(){
            if parser.on_token(TokenType::BlockBegin){
                match parser.slice_block(TokenType::BlockBegin, TokenType::BlockEnd) {
                    Ok(tok) => {
                        match parse(Parser::new(tok)){
                            Ok(frst) => {
                                let mut block = AST{ kind: TokenType::BlockBegin, children: frst };
                                block.children.push(AST { kind: TokenType::BlockEnd, children: vec![] });

                                forest.push(block);
                            },

                            Err(errs) =>{
                                for e in errs{ errors.push(e) }
                            }
                        }
                        parser.skip(tok.len()+2);
                    },
                    Err(e) => {
                        errors.push(e);
                        parser.skip(1);
                    }
                }

            }else{
                match parser.parse_with_node(){
                    Ok(ast) => forest.push(ast),
                    Err(e) => {
                        errors.push(e);
                        parser.skip(1);
                    }
                }
            }
        }

        if !errors.is_empty(){ Err(errors) }
        else { Ok(forest) }
    }

    
    let result = parse(Parser::new(tokens));

    match result{
        Err(_) => assert!(false),
        Ok(forest) => {
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