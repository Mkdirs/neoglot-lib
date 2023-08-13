use std::vec;

use crate::{lexer::*, parser::{*, expression::{ExpressionParser, Expr, Operator, Position}}, regex::Symbol};

#[derive(Debug, Hash, Clone, Copy, PartialOrd, PartialEq, Eq)]
enum TokenType{
    BlockBegin,
    BlockEnd,
    A,B
}

impl Symbol for TokenType{}
impl TokenKind for TokenType{}


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



    fn parse(mut parser:Parser<TokenType>) -> Option<Vec<AST<TokenType>>>{
        let mut forest:Vec<AST<TokenType>> = vec![];
        let mut sucess = true;

        while !parser.finished(){
            if parser.on_token(TokenType::BlockBegin){
                match parser.slice_block(TokenType::BlockBegin, TokenType::BlockEnd) {
                    Some(tok) => {
                        match parse(Parser::new(tok)){
                            Some(frst) => {
                                let mut block = AST{ kind: TokenType::BlockBegin, children: frst };
                                block.children.push(AST { kind: TokenType::BlockEnd, children: vec![] });

                                forest.push(block);
                            },

                            None =>{
                                sucess = false;
                            }
                        }
                        parser.skip(tok.len()+2);
                    },
                    None => {
                        sucess = false;
                        parser.skip(1);
                    }
                }

            }else if parser.on_token(TokenType::A){
                forest.push(AST{kind: parser.pop().unwrap().kind, children: vec![]});

            }else if parser.on_token(TokenType::B){
                forest.push(AST{kind: parser.pop().unwrap().kind, children: vec![]});
            }
        }

        if !sucess{ None }
        else { Some(forest) }
    }

    
    let result = parse(Parser::new(tokens));

    match result{
        None=> assert!(false),
        Some(forest) => {
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
    Plus, Minus, Mul, Bang,
    A,B, C, D, E, F, G, H
}

impl Symbol for ExprTok{}
impl TokenKind for ExprTok{}

#[test]
fn parse_expr(){
    let mut parser = ExpressionParser::new();

    parser.add_operator(Operator{kind: ExprTok::Plus, position: Position::Infix} , 1);
    parser.add_operator(Operator{kind: ExprTok::Minus, position: Position::Infix}, 1);

    parser.add_operator(Operator{kind: ExprTok::Mul, position: Position::Infix}, 2);

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
            ExprTok::Mul => "*",
            ExprTok::Bang => "!"
        };
        let loc = Location{file: String::new(), line: 0, column: 0};
        Token{kind: e, location: loc, literal: String::from(literal)}
    }).collect::<Vec<Token<ExprTok>>>();

    let result = parser.parse(&expr);

    assert!(result.is_some());

    let result = result.unwrap();
    let location = Location{file: String::new(), line: 0, column: 0};
    

    assert_eq!(result, AST{kind:  Expr::Operator(Token{kind: ExprTok::Plus, location: location.clone(), literal: "+".to_string()}), children: vec![
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

}



#[test]
fn parse_expr_nested(){
    let mut parser = ExpressionParser::new();

    parser.add_operator(Operator{kind: ExprTok::Plus, position: Position::Infix} , 1);
    parser.add_operator(Operator{kind: ExprTok::Minus, position: Position::Infix}, 1);

    parser.add_operator(Operator{kind: ExprTok::Mul, position: Position::Infix}, 2);

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
            ExprTok::Mul => "*",
            ExprTok::Bang => "!"
        };
        let loc = Location{file: String::new(), line: 0, column: 0};
        Token{kind: e, location: loc, literal: String::from(literal)}
    }).collect::<Vec<Token<ExprTok>>>();

    let result = parser.parse(&expr);

    assert!(result.is_some());

    let result = result.unwrap();
    let loc = Location{file: String::new(), line: 0, column: 0};
    
    assert_eq!(result, AST{ kind: Expr::Operator(Token{kind: ExprTok::Minus, location: loc.clone(), literal: "-".to_string()}), children: vec![
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
}

#[test]
fn parse_prefix(){
    let mut parser = ExpressionParser::new();

    parser.add_operator(Operator { kind: ExprTok::Plus, position: Position::Prefix }, 1);
    parser.set_high_priority_group(ExprTok::LParen, ExprTok::RParen);

    let raw_expr1 = vec![
        ExprTok::Plus,
        ExprTok::A,
        ExprTok::Plus,
        ExprTok::A
    ];

    let expr1 = raw_expr1.into_iter().map(|e| {
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
            ExprTok::Mul => "*",
            ExprTok::Bang => "!"
        };
        let loc = Location{file: String::new(), line: 0, column: 0};
        Token{kind: e, location: loc, literal: String::from(literal)}
    }).collect::<Vec<Token<ExprTok>>>();

    let raw_expr2 = vec![
        ExprTok::Plus,
        ExprTok::B,
        ExprTok::A
    ];

    let expr2 = raw_expr2.into_iter().map(|e| {
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
            ExprTok::Mul => "*",
            ExprTok::Bang => "!"
        };
        let loc = Location{file: String::new(), line: 0, column: 0};
        Token{kind: e, location: loc, literal: String::from(literal)}
    }).collect::<Vec<Token<ExprTok>>>();

    assert_eq!(parser.parse(&expr1), None);

    let loc = Location{file: String::new(), line: 0, column: 0};

    assert_eq!(parser.parse(&expr2), Some(AST{
        kind: Expr::Operator(Token { location: loc.clone(), kind: ExprTok::Plus, literal: "+".to_string() }),
        children: vec![
            AST{ kind: Expr::Unknown(&[
                Token { location: loc.clone(), kind: ExprTok::B, literal: "B".to_string() },
                Token { location: loc.clone(), kind: ExprTok::A, literal: "A".to_string() }
                ]), children: vec![] }
        ]
    }));


}


#[test]
fn parse_sufix(){
    let mut parser = ExpressionParser::new();

    parser.add_operator(Operator { kind: ExprTok::Mul, position: Position::Sufix }, 1);
    parser.set_high_priority_group(ExprTok::LParen, ExprTok::RParen);

    let raw_expr1 = vec![
        ExprTok::A,
        ExprTok::Mul,
        ExprTok::A,
        ExprTok::Mul
    ];

    let expr1 = raw_expr1.into_iter().map(|e| {
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
            ExprTok::Mul => "*",
            ExprTok::Bang => "!"
        };
        let loc = Location{file: String::new(), line: 0, column: 0};
        Token{kind: e, location: loc, literal: String::from(literal)}
    }).collect::<Vec<Token<ExprTok>>>();

    let raw_expr2 = vec![
        ExprTok::C,
        ExprTok::Mul,
        ExprTok::Mul
    ];

    let expr2 = raw_expr2.into_iter().map(|e| {
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
            ExprTok::Mul => "*",
            ExprTok::Bang => "!"
        };
        let loc = Location{file: String::new(), line: 0, column: 0};
        Token{kind: e, location: loc, literal: String::from(literal)}
    }).collect::<Vec<Token<ExprTok>>>();

    assert_eq!(parser.parse(&expr1), None);

    let loc = Location{file: String::new(), line: 0, column: 0};

    assert_eq!(parser.parse(&expr2), Some(AST{
        kind: Expr::Operator(Token { location: loc.clone(), kind: ExprTok::Mul, literal: "*".to_string() }),
        children: vec![
            AST{
                kind: Expr::Operator(Token { location: loc.clone(), kind: ExprTok::Mul, literal: "*".to_string() }),
                children: vec![
                    AST{
                        kind: Expr::Operand(Token { location: loc.clone(), kind: ExprTok::C, literal: "C".to_string() }),
                        children: vec![]
                     }
                ] }
        ]
    }));


}

#[test]
fn parse_mixed(){
    let mut parser = ExpressionParser::new();


    parser.add_operator(Operator { kind: ExprTok::Plus, position: Position::Prefix }, 1);
    parser.add_operator(Operator { kind: ExprTok::Mul, position: Position::Infix }, 2);
    parser.add_operator(Operator { kind: ExprTok::Bang, position: Position::Sufix }, 3);

    parser.set_high_priority_group(ExprTok::LParen, ExprTok::RParen);

    // + a b * c!
    let raw_expr1 = vec![
        ExprTok::Plus,
        ExprTok::A,
        ExprTok::B,
        ExprTok::Mul,
        ExprTok::C,
        ExprTok::Bang
    ];

    // (+ a b) * c!
    let raw_expr2 = vec![
        ExprTok::LParen,
        ExprTok::Plus,
        ExprTok::A,
        ExprTok::B,
        ExprTok::RParen,
        ExprTok::Mul,
        ExprTok::C,
        ExprTok::Bang
    ];

    let expr1 = raw_expr1.into_iter().map(|e| {
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
            ExprTok::Mul => "*",
            ExprTok::Bang => "!"
        };
        let loc = Location{file: String::new(), line: 0, column: 0};
        Token{kind: e, location: loc, literal: String::from(literal)}
    }).collect::<Vec<Token<ExprTok>>>();

    let expr2 = raw_expr2.into_iter().map(|e| {
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
            ExprTok::Mul => "*",
            ExprTok::Bang => "!"
        };
        let loc = Location{file: String::new(), line: 0, column: 0};
        Token{kind: e, location: loc, literal: String::from(literal)}
    }).collect::<Vec<Token<ExprTok>>>();

    let loc = Location{file: String::new(), line: 0, column: 0};

    assert_eq!(parser.parse(&expr1), Some(AST{
        kind: Expr::Operator(Token { location: loc.clone(), kind: ExprTok::Plus, literal: "+".to_string() }),
        children: vec![
            AST{
                kind: Expr::Operator(Token { location: loc.clone(), kind: ExprTok::Mul, literal: "*".to_string() }),
                children: vec![
                    AST{
                        kind: Expr::Unknown(&[
                            Token { location: loc.clone(), kind: ExprTok::A, literal: "A".to_string() },
                            Token { location: loc.clone(), kind: ExprTok::B, literal: "B".to_string() }
                        ]),
                        children: vec![]
                    },

                    AST{
                        kind: Expr::Operator(Token { location: loc.clone(), kind: ExprTok::Bang, literal: "!".to_string() }),
                        children: vec![
                            AST { kind: Expr::Operand(Token { location: loc.clone(), kind: ExprTok::C, literal: "C".to_string() }), children: vec![] }
                        ]
                    }
                ]
            }
        ]
    }));

    assert_eq!(parser.parse(&expr2), Some(AST{
        kind: Expr::Operator(Token { location: loc.clone(), kind: ExprTok::Mul, literal: "*".to_string() }),
        children: vec![
            AST{
                kind: Expr::Operator(Token { location: loc.clone(), kind: ExprTok::Plus, literal: "+".to_string() }),
                children: vec![
                    AST{
                        kind: Expr::Unknown(&[
                            Token { location: loc.clone(), kind: ExprTok::A, literal: "A".to_string() },
                            Token { location: loc.clone(), kind: ExprTok::B, literal: "B".to_string() }
                        ]),
                        children: vec![]
                    }
                ]
            },

            AST{
                kind: Expr::Operator(Token { location: loc.clone(), kind: ExprTok::Bang, literal: "!".to_string() }),
                children: vec![
                    AST{ kind: Expr::Operand(Token { location: loc.clone(), kind: ExprTok::C, literal: "C".to_string() }), children: vec![] }
                ]
            }
        ]
    }));

}