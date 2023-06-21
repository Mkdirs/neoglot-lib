use std::path::{PathBuf, Path};

use crate::{lexer::*, regex::*};

#[derive(PartialEq, Copy, Clone, Debug)]
enum TokenType{
    UINT,
    PLUS,
    MINUS,
    TIMES,
    DIVIDE
}

#[test]
fn node_lexing(){
    let node = Lexernode::new(
        Regex::<char>::new().then(RegexElement::Set('0', '9', Quantifier::OneOrMany)),
        TokenType::UINT
    );

    let virtual_location = Location{ file: Path::new("virtual_file").to_path_buf(), line:0, column:0};

    let candidate1 = "hello world".chars().collect::<Vec<char>>();
    let candidate2 = " ".chars().collect::<Vec<char>>();
    let candidate3 = "-10°C".chars().collect::<Vec<char>>();
    let candidate4 = "1256 + 359".chars().collect::<Vec<char>>();
    let candidate5 = "30_cobra () func let i".chars().collect::<Vec<char>>();

    let result1:(&[char], Option<Token<TokenType>>) = (&['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd'], None);
    let result2:(&[char], Option<Token<TokenType>>) = (&[' '], None);
    let result3:(&[char], Option<Token<TokenType>>) = (&['-', '1', '0', '°', 'C'], None);
    let result4:(&[char], Option<Token<TokenType>>) = (&[' ', '+', ' ', '3', '5', '9'], Some(Token{location: virtual_location.clone(), kind: TokenType::UINT, literal: String::from("1256")}) );
    let result5:(&[char], Option<Token<TokenType>>) = (&['_', 'c', 'o', 'b', 'r', 'a', ' ', '(', ')', ' ', 'f', 'u', 'n', 'c', ' ', 'l', 'e', 't', ' ', 'i'], Some(Token{location: virtual_location.clone(), kind: TokenType::UINT, literal: String::from("30")}) );

    assert_eq!(node.tokenize(&candidate1, &virtual_location), result1);
    assert_eq!(node.tokenize(&candidate2, &virtual_location), result2);
    assert_eq!(node.tokenize(&candidate3, &virtual_location), result3);
    assert_eq!(node.tokenize(&candidate4, &virtual_location), result4);
    assert_eq!(node.tokenize(&candidate5, &virtual_location), result5);

}

#[test]
fn file_lexing(){
    let mut lexer = Lexer::<TokenType>::new();

    let uint_node = Lexernode::new(
        Regex::new().then(RegexElement::Set('0', '9', Quantifier::OneOrMany)),
        TokenType::UINT
    );

    let plus_node = Lexernode::new(
        Regex::new().then(RegexElement::Item('+', Quantifier::Exactly(1))),
        TokenType::PLUS
    );

    let minus_node = Lexernode::new(
        Regex::new().then(RegexElement::Item('-', Quantifier::Exactly(1))),
        TokenType::MINUS
    );

    let times_node = Lexernode::new(
        Regex::new().then(RegexElement::Item('*', Quantifier::Exactly(1))),
        TokenType::TIMES
    );

    let divide_node = Lexernode::new(
        Regex::new().then(RegexElement::Item('/', Quantifier::Exactly(1))),
        TokenType::DIVIDE
    );

    lexer.register(uint_node);
    lexer.register(plus_node);
    lexer.register(minus_node);
    lexer.register(times_node);
    lexer.register(divide_node);

    let result1 = lexer.tokenize_content(include_str!("empty.txt").to_string(), Some(Path::new("empty.txt").to_path_buf()));
    let result2 = lexer.tokenize_content(include_str!("invalid.txt").to_string(), Some(Path::new("invalid.txt").to_path_buf()));
    let result3 = lexer.tokenize_content(include_str!("basic_math_sheet.txt").to_string(), Some(Path::new("basic_math_sheet.txt").to_path_buf()));

    assert!(result1.unwrap().is_empty());
    
    assert_eq!(result2, Err(LexingError{ location:Location { file: Path::new("invalid.txt").to_path_buf(), line: 2, column: 2 } }));
    
    assert_eq!(result3.unwrap(), vec![
        Token{ location:Location { file: Path::new("basic_math_sheet.txt").to_path_buf(), line: 0, column: 0 },
            kind: TokenType::UINT, literal: String::from("10")
        },

        Token{ location:Location { file: Path::new("basic_math_sheet.txt").to_path_buf(), line: 0, column: 2 },
            kind: TokenType::PLUS, literal: String::from("+")
        },

        Token{ location:Location { file: Path::new("basic_math_sheet.txt").to_path_buf(), line: 0, column: 3 },
            kind: TokenType::UINT, literal: String::from("53")
        },

        Token{ location:Location { file: Path::new("basic_math_sheet.txt").to_path_buf(), line: 1, column: 0 },
            kind: TokenType::UINT, literal: String::from("3")
        },

        Token{ location:Location { file: Path::new("basic_math_sheet.txt").to_path_buf(), line: 1, column: 2 },
            kind: TokenType::MINUS, literal: String::from("-")
        },

        Token{ location:Location { file: Path::new("basic_math_sheet.txt").to_path_buf(), line: 1, column: 4 },
            kind: TokenType::UINT, literal: String::from("125")
        }

    ]);

}