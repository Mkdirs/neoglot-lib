use pattern_matcher::{AtLeast, Symbol, WithQuantifier};

use crate::lexer::*;

#[derive(PartialEq, PartialOrd, Eq, Hash, Copy, Clone, Debug)]
enum TokenType{
    UINT,
    PLUS,
    MINUS,
    TIMES,
    DIVIDE
}

impl Symbol for TokenType{}
impl TokenKind for TokenType{}

#[test]
fn node_lexing(){
    let node = LexerModule::new(TokenType::UINT, |pipeline| {
        Ok(
            pipeline
            .with_quantifier(AtLeast(1), |p| p.expect_any_of(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))?
            .terminate()
        )
    });

    let virtual_location = Location{ file: "virtual_file".to_string(), line:0, column:0};

    let candidate1 = "hello world";
    let candidate2 = " ";
    let candidate3 = "-10°C";
    let candidate4 = "1256 + 359";
    let candidate5 = "30_cobra () func let i";

    let result1 = None;
    let result2 = None;
    let result3 = None;
    let result4 = Some( (Token{location: virtual_location.clone(), kind: TokenType::UINT, literal: "1256".to_string()}, 4 ) );
    let result5 = Some( (Token{location: virtual_location.clone(), kind: TokenType::UINT, literal: "30".to_string()}, 2) );

    assert_eq!(node.tokenize(&candidate1, &virtual_location), result1);
    assert_eq!(node.tokenize(&candidate2, &virtual_location), result2);
    assert_eq!(node.tokenize(&candidate3, &virtual_location), result3);
    assert_eq!(node.tokenize(&candidate4, &virtual_location), result4);
    assert_eq!(node.tokenize(&candidate5, &virtual_location), result5);

}

#[test]
fn file_lexing(){
    let mut lexer = Lexer::<TokenType>::new();

    let uint_node = LexerModule::new(TokenType::UINT, |pipeline| {
        Ok(
            pipeline
            .with_quantifier(AtLeast(1), |p| p.expect_any_of(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))?
            .terminate()
        )
    });

    let plus_node = LexerModule::new(TokenType::PLUS, |pipeline| Ok(pipeline.expect_symbol(&'+')?.terminate()));

    let minus_node = LexerModule::new(TokenType::MINUS, |pipeline| Ok(pipeline.expect_symbol(&'-')?.terminate()));

    let times_node = LexerModule::new(TokenType::TIMES, |pipeline| Ok(pipeline.expect_symbol(&'*')?.terminate()));

    let divide_node = LexerModule::new(TokenType::DIVIDE, |pipeline| Ok(pipeline.expect_symbol(&'/')?.terminate()));

    lexer.register(uint_node);
    lexer.register(plus_node);
    lexer.register(minus_node);
    lexer.register(times_node);
    lexer.register(divide_node);

    let result1 = lexer.tokenize_content(include_str!("empty.txt").to_string(), "empty.txt");
    let result2 = lexer.tokenize_content(include_str!("invalid.txt").to_string(), "invalid.txt");
    let result3 = lexer.tokenize_content(include_str!("basic_math_sheet.txt").to_string(), "basic_math_sheet.txt");

    match result1 {
        Ok(tokens) => assert!(tokens.is_empty()),
        Err(_) => assert!(false)
    }
    
    match result2{
        Ok(_) => assert!(false),
        Err(e) => {
            assert_eq!(e, LexingError{ location:Location { file: "invalid.txt".to_string(), line: 2, column: 2 } });
        }
    }

    match result3{
        Ok(tokens) => {
            assert_eq!(tokens, vec![
                Token{ location:Location { file: "basic_math_sheet.txt".to_string(), line: 0, column: 0 },
                    kind: TokenType::UINT, literal: "10".to_string()
                },

                Token{ location:Location { file: "basic_math_sheet.txt".to_string(), line: 0, column: 2 },
                    kind: TokenType::PLUS, literal: "+".to_string()
                },

                Token{ location:Location { file: "basic_math_sheet.txt".to_string(), line: 0, column: 3 },
                    kind: TokenType::UINT, literal: "53".to_string()
                },

                Token{ location:Location { file: "basic_math_sheet.txt".to_string(), line: 1, column: 0 },
                    kind: TokenType::UINT, literal: "3".to_string()
                },

                Token{ location:Location { file: "basic_math_sheet.txt".to_string(), line: 1, column: 2 },
                    kind: TokenType::MINUS, literal: "-".to_string()
                },

                Token{ location:Location { file: "basic_math_sheet.txt".to_string(), line: 1, column: 4 },
                    kind: TokenType::UINT, literal: "125".to_string()
                }

            ]);
        },
        Err(_) => assert!(false)
    }

}