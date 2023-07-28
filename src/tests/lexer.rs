use crate::{lexer::*, regex::*};

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
    let node = LexerNode::new(
        Regex::<char>::new().then(RegexElement::Set('0', '9', Quantifier::OneOrMany)),
        TokenType::UINT
    );

    let virtual_location = Location{ file: "virtual_file".to_string(), line:0, column:0};

    let candidate1 = "hello world".chars().collect::<Vec<char>>();
    let candidate2 = " ".chars().collect::<Vec<char>>();
    let candidate3 = "-10°C".chars().collect::<Vec<char>>();
    let candidate4 = "1256 + 359".chars().collect::<Vec<char>>();
    let candidate5 = "30_cobra () func let i".chars().collect::<Vec<char>>();

    let result1:(&[char], Option<Token<TokenType>>) = (&['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd'], None);
    let result2:(&[char], Option<Token<TokenType>>) = (&[' '], None);
    let result3:(&[char], Option<Token<TokenType>>) = (&['-', '1', '0', '°', 'C'], None);
    let result4:(&[char], Option<Token<TokenType>>) = (&[' ', '+', ' ', '3', '5', '9'], Some(Token{location: virtual_location.clone(), kind: TokenType::UINT, literal: "1256".to_string()}) );
    let result5:(&[char], Option<Token<TokenType>>) = (&['_', 'c', 'o', 'b', 'r', 'a', ' ', '(', ')', ' ', 'f', 'u', 'n', 'c', ' ', 'l', 'e', 't', ' ', 'i'], Some(Token{location: virtual_location.clone(), kind: TokenType::UINT, literal: "30".to_string()}) );

    assert_eq!(node.tokenize(&candidate1, &virtual_location), result1);
    assert_eq!(node.tokenize(&candidate2, &virtual_location), result2);
    assert_eq!(node.tokenize(&candidate3, &virtual_location), result3);
    assert_eq!(node.tokenize(&candidate4, &virtual_location), result4);
    assert_eq!(node.tokenize(&candidate5, &virtual_location), result5);

}

#[test]
fn file_lexing(){
    let mut lexer = Lexer::<TokenType>::new();

    let uint_node = LexerNode::new(
        Regex::new().then(RegexElement::Set('0', '9', Quantifier::OneOrMany)),
        TokenType::UINT
    );

    let plus_node = LexerNode::new(
        Regex::new().then(RegexElement::Item('+', Quantifier::Exactly(1))),
        TokenType::PLUS
    );

    let minus_node = LexerNode::new(
        Regex::new().then(RegexElement::Item('-', Quantifier::Exactly(1))),
        TokenType::MINUS
    );

    let times_node = LexerNode::new(
        Regex::new().then(RegexElement::Item('*', Quantifier::Exactly(1))),
        TokenType::TIMES
    );

    let divide_node = LexerNode::new(
        Regex::new().then(RegexElement::Item('/', Quantifier::Exactly(1))),
        TokenType::DIVIDE
    );

    lexer.register(uint_node);
    lexer.register(plus_node);
    lexer.register(minus_node);
    lexer.register(times_node);
    lexer.register(divide_node);

    let result1 = lexer.tokenize_content(include_str!("empty.txt").to_string(), "empty.txt");
    let result2 = lexer.tokenize_content(include_str!("invalid.txt").to_string(), "invalid.txt");
    let result3 = lexer.tokenize_content(include_str!("basic_math_sheet.txt").to_string(), "basic_math_sheet.txt");

    match result1 {
        LexingResult::Ok(tokens) => assert!(tokens.is_empty()),
        LexingResult::Err(_) => assert!(false)
    }
    
    match result2{
        LexingResult::Ok(_) => assert!(false),
        LexingResult::Err(errors) => {
            assert_eq!(errors.len(), 8);
            assert_eq!(errors, vec![
                LexingError{ location:Location { file: "invalid.txt".to_string(), line: 2, column: 2 } },
                LexingError{ location:Location { file: "invalid.txt".to_string(), line: 2, column: 3 } },
                LexingError{ location:Location { file: "invalid.txt".to_string(), line: 2, column: 4 } },
                LexingError{ location:Location { file: "invalid.txt".to_string(), line: 2, column: 5 } },
                LexingError{ location:Location { file: "invalid.txt".to_string(), line: 2, column: 6 } },
                LexingError{ location:Location { file: "invalid.txt".to_string(), line: 2, column: 7 } },
                LexingError{ location:Location { file: "invalid.txt".to_string(), line: 2, column: 8 } },
                LexingError{ location:Location { file: "invalid.txt".to_string(), line: 2, column: 9 } }
            ]);
        }
    }

    match result3{
        LexingResult::Ok(tokens) => {
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
        LexingResult::Err(_) => assert!(false)
    }

}