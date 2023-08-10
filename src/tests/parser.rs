use std::vec;

use crate::{lexer::*, parser::{*, expression::{ExpressionParser, Expr}}, regex::{Symbol, Regex, RegexElement, Quantifier}};

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

#[derive(Debug, Hash, Clone, Copy, PartialOrd, PartialEq, Eq)]
enum ExprTok{
    LParen,
    RParen,
    Plus, Minus, Mul,
    A,B, C, D, E, F, G, H
}

impl Symbol for ExprTok{}
impl TokenKind for ExprTok{}

#[test]
fn parse_expr(){
    let mut parser = ExpressionParser::new();

    parser.add_operator(ExprTok::Plus, 1);
    parser.add_operator(ExprTok::Minus, 1);

    parser.add_operator(ExprTok::Mul, 2);

    parser.set_high_priority_group(ExprTok::LParen, ExprTok::RParen);

    // (a-b) * (c-d) + (e-f) * (g-h);

    let raw_expr = vec![
        ExprTok::LParen, ExprTok::A, ExprTok::Minus, ExprTok::B, ExprTok::RParen, ExprTok::Mul, ExprTok::LParen, ExprTok::C, ExprTok::Minus, ExprTok::D, ExprTok::RParen,
        ExprTok::Plus,
        ExprTok::LParen, ExprTok::E, ExprTok::Minus, ExprTok::F, ExprTok::RParen, ExprTok::Mul, ExprTok::LParen, ExprTok::G, ExprTok::Minus, ExprTok::H, ExprTok::RParen
    ];

    let expr = raw_expr.into_iter().map(|e| {
        let literal = match e{
            ExprTok::A => "A",
            ExprTok::B => "B",
            ExprTok::C => "C",
            ExprTok::D => "D",
            ExprTok::E => "E",
            ExprTok::F => "F",
            ExprTok::G => "G",
            ExprTok::H => "H",
            ExprTok::LParen => "(",
            ExprTok::RParen => ")",
            ExprTok::Plus => "+",
            ExprTok::Minus => "-",
            ExprTok::Mul => "*"
        };
        let loc = Location{file: String::new(), line: 0, column: 0};
        Token{kind: e, location: loc, literal: String::from(literal)}
    }).collect::<Vec<Token<ExprTok>>>();

    let result = parser.parse(&expr);

    assert!(result.is_some());

    let result = result.unwrap();
    let location = Location{file: String::new(), line: 0, column: 0};
    

    match result{
        Ok(ast) => {
            assert_eq!(ast, AST{kind:  Expr::Operator(Token{kind: ExprTok::Plus, location: location.clone(), literal: "+".to_string()}), children: vec![
                AST{ kind: Expr::Operator(Token{kind: ExprTok::Mul, location: location.clone(), literal: "*".to_string()}), children: vec![
                    AST{ kind: Expr::Operator(Token{kind: ExprTok::Minus, location: location.clone(), literal: "-".to_string()}), children: vec![
                        AST{ kind: Expr::Operand(Token{kind: ExprTok::A, location: location.clone(), literal: "A".to_string()}), children: vec![] },
                        AST{ kind: Expr::Operand(Token{kind: ExprTok::B, location: location.clone(), literal: "B".to_string()}), children: vec![] }
                    ] },

                    AST{ kind: Expr::Operator(Token{kind: ExprTok::Minus, location: location.clone(), literal: "-".to_string()}), children: vec![
                        AST{ kind: Expr::Operand(Token{kind: ExprTok::C, location: location.clone(), literal: "C".to_string()}), children: vec![] },
                        AST{ kind: Expr::Operand(Token{kind: ExprTok::D, location: location.clone(), literal: "D".to_string()}), children: vec![] }
                    ] }
                ] },

                AST{ kind: Expr::Operator(Token{kind: ExprTok::Mul, location: location.clone(), literal: "*".to_string()}), children: vec![
                    AST{ kind: Expr::Operator(Token{kind: ExprTok::Minus, location: location.clone(), literal: "-".to_string()}), children: vec![
                        AST{ kind: Expr::Operand(Token{kind: ExprTok::E, location: location.clone(), literal: "E".to_string()}), children: vec![] },
                        AST{ kind: Expr::Operand(Token{kind: ExprTok::F, location: location.clone(), literal: "F".to_string()}), children: vec![] }
                    ] },

                    AST{ kind: Expr::Operator(Token{kind: ExprTok::Minus, location: location.clone(), literal: "-".to_string()}), children: vec![
                        AST{ kind: Expr::Operand(Token{kind: ExprTok::G, location: location.clone(), literal: "G".to_string()}), children: vec![] },
                        AST{ kind: Expr::Operand(Token{kind: ExprTok::H, location: location.clone(), literal: "H".to_string()}), children: vec![] }
                    ] }
                ] }

                
            ]});
        },

        Err(e) => assert!(false, "{:?}", e)
    }

}



#[test]
fn parse_expr_nested(){
    let mut parser = ExpressionParser::new();

    parser.add_operator(ExprTok::Plus, 1);
    parser.add_operator(ExprTok::Minus, 1);

    parser.add_operator(ExprTok::Mul, 2);

    parser.set_high_priority_group(ExprTok::LParen, ExprTok::RParen);

    // (a - b * (c + (d * g) - h ) )
    

    let raw_expr = vec![
        ExprTok::LParen,
        ExprTok::A, ExprTok::Minus, ExprTok::B, ExprTok::Mul,
        
        ExprTok::LParen,
        ExprTok::C, ExprTok::Plus,
        
        ExprTok::LParen,
        ExprTok::D, ExprTok::Mul, ExprTok::G,
        ExprTok::RParen,

        ExprTok::Minus, ExprTok::H,
        ExprTok::RParen,

        ExprTok::RParen
    ];

    let expr = raw_expr.into_iter().map(|e| {
        let literal = match e{
            ExprTok::A => "A",
            ExprTok::B => "B",
            ExprTok::C => "C",
            ExprTok::D => "D",
            ExprTok::E => "E",
            ExprTok::F => "F",
            ExprTok::G => "G",
            ExprTok::H => "H",
            ExprTok::LParen => "(",
            ExprTok::RParen => ")",
            ExprTok::Plus => "+",
            ExprTok::Minus => "-",
            ExprTok::Mul => "*"
        };
        let loc = Location{file: String::new(), line: 0, column: 0};
        Token{kind: e, location: loc, literal: String::from(literal)}
    }).collect::<Vec<Token<ExprTok>>>();

    let result = parser.parse(&expr);

    assert!(result.is_some());

    let result = result.unwrap();
    let loc = Location{file: String::new(), line: 0, column: 0};
    

    match result{
        Ok(ast) => {
            assert_eq!(ast, AST{ kind: Expr::Operator(Token{kind: ExprTok::Minus, location: loc.clone(), literal: "-".to_string()}), children: vec![
                AST{ kind: Expr::Operand(Token{kind: ExprTok::A, location: loc.clone(), literal: "A".to_string()}), children: vec![] },
        
                AST{ kind: Expr::Operator(Token{kind: ExprTok::Mul, location: loc.clone(), literal: "*".to_string()}), children: vec![
                    AST{ kind: Expr::Operand(Token{kind: ExprTok::B, location: loc.clone(), literal: "B".to_string()}), children: vec![] },
        
                    AST{ kind: Expr::Operator(Token{kind: ExprTok::Minus, location: loc.clone(), literal: "-".to_string()}), children: vec![
                        AST{ kind: Expr::Operator(Token{kind: ExprTok::Plus, location: loc.clone(), literal: "+".to_string()}), children: vec![
                            AST{ kind: Expr::Operand(Token{kind: ExprTok::C, location: loc.clone(), literal: "C".to_string()}), children: vec![] },
        
                            AST{ kind: Expr::Operator(Token{kind: ExprTok::Mul, location: loc.clone(), literal: "*".to_string()}), children: vec![
                                AST{ kind: Expr::Operand(Token{kind: ExprTok::D, location: loc.clone(), literal: "D".to_string()}), children: vec![] },
                                AST{ kind: Expr::Operand(Token{kind: ExprTok::G, location: loc.clone(), literal: "G".to_string()}), children: vec![] }
                            ] }
                        ] },
        
                        AST{ kind: Expr::Operand(Token{kind: ExprTok::H, location: loc.clone(), literal: "H".to_string()}), children: vec![] }
                    ] }
        
                    
                ] }
            ]});
        },

        Err(e) => assert!(false, "{:?}", e)
    }

}