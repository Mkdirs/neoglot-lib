use std::collections::{HashSet, HashMap};

use crate::lexer::{TokenKind, Token};

use super::{AST, ParsingError};

#[derive(Debug, PartialEq, Clone)]
/// The nodes in an expression
pub enum Expr<'a, T:TokenKind>{
    /// An operator
    Operator(Token<T>),

    /// An operand
    Operand(Token<T>),

    /// An unknown sequence that could not be parsed
    /// Can be fed to a [Parser](super::Parser) for further processing
    Unknown(&'a[Token<T>])
}
/// A parser of expressions
/// 
/// # Exemples
/// ```rust
/// use crate::neoglot_lib::{regex::*, parser::{expression::*, *}, lexer::*};
/// use std::path::Path;
/// 
/// #[derive(Debug, Copy, Clone, Hash, PartialOrd, Eq, PartialEq)]
/// enum TokenType{A, B, C, ADD, SUB, MUL, OPEN_PAREN, CLOSED_PAREN}
/// 
/// impl Symbol for TokenType{}
/// impl TokenKind for TokenType{}
/// 
/// let mut parser = ExpressionParser::<TokenType>::new();
/// 
/// parser.add_operator(TokenType::ADD, 1);
/// parser.add_operator(TokenType::SUB, 1);
/// parser.add_operator(TokenType::MUL, 2);
/// 
/// parser.set_high_priority_group(TokenType::OPEN_PAREN, TokenType::CLOSED_PAREN);
/// 
/// // A + B
/// let expr1 = &[
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 0 },
///         kind: TokenType::A, literal: String::from("A")
///     },
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 1 },
///         kind: TokenType::ADD, literal: String::from("+")
///     },
/// 
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 2 },
///         kind: TokenType::B, literal: String::from("B")
///     }
/// ];
/// // A - B
/// let expr2 = &[
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 0 },
///         kind: TokenType::A, literal: String::from("A")
///     },
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 1 },
///         kind: TokenType::SUB, literal: String::from("+")
///     },
/// 
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 2 },
///         kind: TokenType::B, literal: String::from("B")
///     }
/// ];
/// 
/// // A +(A * B)
/// let expr3 = &[
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 0 },
///         kind: TokenType::A, literal: String::from("A")
///     },
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 1 },
///         kind: TokenType::ADD, literal: String::from("+")
///     },
/// 
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 2 },
///         kind: TokenType::OPEN_PAREN, literal: String::from("(")
///     },
/// 
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 3 },
///         kind: TokenType::A, literal: String::from("A")
///     },
/// 
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 4 },
///         kind: TokenType::MUL, literal: String::from("*")
///     },
/// 
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 5 },
///         kind: TokenType::B, literal: String::from("B")
///     },
/// 
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 6 },
///         kind: TokenType::CLOSED_PAREN, literal: String::from(")")
///     }
/// ];
/// 
/// // A - A*B
/// let expr4 = &[
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 0 },
///         kind: TokenType::A, literal: String::from("A")
///     },
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 1 },
///         kind: TokenType::SUB, literal: String::from("-")
///     },
/// 
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 2 },
///         kind: TokenType::A, literal: String::from("A")
///     },
/// 
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 3 },
///         kind: TokenType::MUL, literal: String::from("*")
///     },
/// 
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 4 },
///         kind: TokenType::B, literal: String::from("B")
///     },
/// 
/// ];
/// 
/// // A - B - C
/// let expr5 = &[
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 0 },
///         kind: TokenType::A, literal: String::from("A")
///     },
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 1 },
///         kind: TokenType::SUB, literal: String::from("-")
///     },
/// 
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 2 },
///         kind: TokenType::B, literal: String::from("B")
///     },
/// 
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 3 },
///         kind: TokenType::SUB, literal: String::from("-")
///     },
/// 
///     Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 4 },
///         kind: TokenType::C, literal: String::from("C")
///     }
/// ];
/// 
/// let result1 = AST{
///     kind: Expr::Operator(Token{ 
///             location: Location{ file: String::from(""), line: 0, column: 1 },
///             kind: TokenType::ADD, literal: String::from("+")
///         }),
///     children: vec![
///         AST{ kind: Expr::Operand(Token{ 
///             location: Location{ file: String::from(""), line: 0, column: 0 },
///             kind: TokenType::A, literal: String::from("A")
///         }), children: vec![] },
///         AST{ kind: Expr::Operand(Token{ 
///             location: Location{ file: String::from(""), line: 0, column: 2 },
///             kind: TokenType::B, literal: String::from("B")
///         }), children: vec![] }
///     ]
/// };
/// 
/// let result2 = AST{
///     kind: Expr::Operator(Token{ 
///             location: Location{ file: String::from(""), line: 0, column: 1 },
///             kind: TokenType::SUB, literal: String::from("+")
///         }),
///     children: vec![
///         AST{ kind: Expr::Operand(Token{ 
///             location: Location{ file: String::from(""), line: 0, column: 0 },
///             kind: TokenType::A, literal: String::from("A")
///         }), children: vec![] },
///         AST{ kind: Expr::Operand(Token{ 
///             location: Location{ file: String::from(""), line: 0, column: 2 },
///             kind: TokenType::B, literal: String::from("B")
///         }), children: vec![] }
///     ]
/// };
/// 
/// let result3 = AST{
///     kind: Expr::Operator(Token{ 
///         location: Location{ file: String::from(""), line: 0, column: 1 },
///         kind: TokenType::ADD, literal: String::from("+")
///     }),
///     children: vec![
///         AST{ kind: Expr::Operand(Token{ 
///             location: Location{ file: String::from(""), line: 0, column: 0 },
///             kind: TokenType::A, literal: String::from("A")
///         }), children: vec![] },
///         AST{ kind: Expr::Operator(Token{ 
///             location: Location{ file: String::from(""), line: 0, column: 4 },
///             kind: TokenType::MUL, literal: String::from("*")
///         }), children: vec![
///             AST{ kind: Expr::Operand(Token{ 
///             location: Location{ file: String::from(""), line: 0, column: 3 },
///             kind: TokenType::A, literal: String::from("A")
///         }), children: vec![] },
///             AST{ kind: Expr::Operand(Token{ 
///             location: Location{ file: String::from(""), line: 0, column: 5 },
///             kind: TokenType::B, literal: String::from("B")
///         }), children: vec![] }
///         ] }
///     ]
/// };
/// 
/// let result4 = AST{
///     kind: Expr::Operator(Token{ 
///             location: Location{ file: String::from(""), line: 0, column: 1 },
///             kind: TokenType::SUB, literal: String::from("-")
///         }),
///     children: vec![
///         AST{ kind: Expr::Operand(Token{ 
///             location: Location{ file: String::from(""), line: 0, column: 0 },
///             kind: TokenType::A, literal: String::from("A")
///         }), children: vec![] },
///         AST{ kind: Expr::Operator(Token{ 
///             location: Location{ file: String::from(""), line: 0, column: 3 },
///             kind: TokenType::MUL, literal: String::from("*")
///         }), children: vec![
///             AST{ kind: Expr::Operand(Token{ 
///                 location: Location{ file: String::from(""), line: 0, column: 2 },
///                 kind: TokenType::A, literal: String::from("A")
///             }), children: vec![] },
///             AST{ kind: Expr::Operand(Token{ 
///                 location: Location{ file: String::from(""), line: 0, column: 4 },
///                 kind: TokenType::B, literal: String::from("B")
///             }), children: vec![] }
///         ] }
///     ]
/// };
/// 
/// let result5 = AST{
///     kind: Expr::Operator(Token{ 
///             location: Location{ file: String::from(""), line: 0, column: 3 },
///             kind: TokenType::SUB, literal: String::from("-")
///         }),
///     children: vec![
///         AST{ kind: Expr::Operator(Token{ 
///             location: Location{ file: String::from(""), line: 0, column: 1 },
///             kind: TokenType::SUB, literal: String::from("-")
///         }), children: vec![
///             AST{ kind: Expr::Operand(Token{ 
///                 location: Location{ file: String::from(""), line: 0, column: 0 },
///                 kind: TokenType::A, literal: String::from("A")
///             }), children: vec![] },
///             AST{ kind: Expr::Operand(Token{ 
///                 location: Location{ file: String::from(""), line: 0, column: 2 },
///                 kind: TokenType::B, literal: String::from("B")
///             }), children: vec![] }
///         ] },
///         AST{ kind: Expr::Operand(Token{ 
///             location: Location{ file: String::from(""), line: 0, column: 4 },
///             kind: TokenType::C, literal: String::from("C")
///         }), children: vec![] }
///     ]
/// };
/// 
/// if let Some(result) = parser.parse(expr1){
///     match result{
///         Ok(ast) => assert_eq!(ast, result1),
///         Err(errs) => assert!(false)
///     }
/// }else { assert!(false); }
/// 
/// if let Some(result) = parser.parse(expr2){
///     match result{
///         Ok(ast) => assert_eq!(ast, result2),
///         Err(errs) => assert!(false)
///     }
/// }else { assert!(false); }
/// 
/// if let Some(result) = parser.parse(expr3){
///     match result{
///         Ok(ast) => assert_eq!(ast, result3),
///         Err(errs) => assert!(false)
///     }
/// }else { assert!(false); }
/// 
/// if let Some(result) = parser.parse(expr4){
///     match(result){
///         Ok(ast) => assert_eq!(ast, result4),
///         Err(errs) => assert!(false)
///     }
/// }else { assert!(false); }
/// 
/// if let Some(result) = parser.parse(expr5){
///     match(result){
///         Ok(ast) => assert_eq!(ast, result5),
///         Err(errs) => assert!(false)
///     }
/// }else { assert!(false); }
/// 
/// 
/// ```
pub struct ExpressionParser<T: TokenKind>{
    /// Set of known operators
    operators:HashSet<T>,

    /// [HashMap] of operators and their priority
    priority:HashMap<T, usize>,

    /// A [token](TokenKind) that acts like an open parenthesis on priority
    high_priority_group_start:Option<T>,

    /// A [token](TokenKind) that acts like a closed parenthesis on priority
    high_priority_group_end:Option<T>
}

impl<T:TokenKind> ExpressionParser<T>{
    pub fn new() -> Self {
        ExpressionParser {
            operators: HashSet::new(),
            priority: HashMap::new(),
            high_priority_group_start: None,
            high_priority_group_end: None
        }
    }
    
    /// Adds an operator to the list of known operators
    /// 
    /// operator: The operator to add
    /// 
    /// priority: Its priority
    pub fn add_operator(&mut self, operator:T, priority:usize){
        self.operators.insert(operator);
        self.priority.insert(operator, priority);
    }


    /// Assign the [tokens](TokenKind) used to modify the priority
    /// 
    /// start: The start of the new priority
    /// 
    /// end: The end of the new priority
    pub fn set_high_priority_group(&mut self, start:T, end:T){
        self.high_priority_group_start = Some(start);
        self.high_priority_group_end = Some(end);
    }

    /// Finds the operator with the least priority
    /// 
    /// candidates: An expression
    fn find_min_priority(&self, candidates:&[Token<T>]) -> Option<usize>{
        let mut min_priority = None;
        let mut min_priority_indx = None;
        let mut priority_multiplier = 1;

        for i in 0..candidates.len(){
            let c = &candidates[i];
            if let Some(priority) = self.priority.get(&c.kind){

                // If we are inside a parenthesis-like bloc,
                // the priority must be multiplied
                // we also skip the bloc start/end
                if self.high_priority_group_start.is_some_and(|e| e == c.kind){
                    priority_multiplier = priority_multiplier * 100;
                    continue;
                }else if self.high_priority_group_end.is_some_and(|e| e == c.kind){
                    priority_multiplier = priority_multiplier / 100;
                    continue;
                }else{ priority_multiplier = 1 };

                match min_priority {
                    Some(min_p) =>{
                        if priority*priority_multiplier <= min_p {
                            min_priority = Some(*priority * priority_multiplier);
                            min_priority_indx = Some(i);
                        }
                    },

                    None => {
                        min_priority = Some(priority * priority_multiplier);
                        min_priority_indx = Some(i);
                    }
                }
                
            }
        }

        min_priority_indx
    }

    /// Checks if the number of start_groups is equals to the number of end_groups
    fn check_groups_validity(&self, candidates:&[Token<T>]) -> bool{
        if self.high_priority_group_start.is_none() || self.high_priority_group_end.is_none(){
            return true;
        }
        let mut open_groups = 0;
        for c in candidates{
            if c.kind == self.high_priority_group_start.unwrap(){ open_groups += 1; }
            else if c.kind == self.high_priority_group_end.unwrap(){ open_groups -= 1; }

            if open_groups < 0{ break; }
        }
        
        open_groups == 0
    }

    /// Strips the leading and trailing groups token
    fn strip_group<'a>(&self, candidates:&'a[Token<T>]) -> Result<Option<&'a[Token<T>]>, ParsingError<T>>{
        
        if self.high_priority_group_start.is_none()
        || self.high_priority_group_end.is_none()
        || candidates.is_empty(){
            return Ok(Some(candidates));
        }

        if !self.check_groups_validity(candidates){
            let loc = candidates[0].location.clone();
            return Err(ParsingError::InvalidGroups(loc))
        }

        let mut left = 0;
        let mut right = 0;
        let mut search = true;
        let mut i = 0;

        while search && i < candidates.len() {
            let c = &candidates[i];

            if Some(c.kind) == self.high_priority_group_start{ left += 1; }
            else{ search = false; }

            i += 1;
        }

        search = true;
        i = candidates.len()-1;

        while search{
            let c = &candidates[i];

            if Some(c.kind) == self.high_priority_group_end { right +=1; }
            else { search = false; }

            if i == 0{ search = false;}
            else { i -= 1;}
        }

        Ok(candidates.get(left..candidates.len()-right))
    }


    /// Parse an expression
    pub fn parse<'a>(&self, candidates:&'a[Token<T>]) -> Option<Result<AST<Expr<'a, T>>, Vec<ParsingError<T>>>>
    {
        if candidates.is_empty(){ return None; }

        if candidates.len() == 1{
            return Some(Ok(AST{ kind: Expr::Operand(candidates[0].clone()), children: vec![] }));
        }


        let min_indx = self.find_min_priority(candidates);
        
        let result = if let Some(min_indx) = min_indx{
            let operator = &candidates[min_indx];

            let mut errors:Vec<ParsingError<T>> = vec![];
            let mut children = vec![];


            let left_sub_expr = self.strip_group(candidates.get(0..min_indx).unwrap_or_default());
            let right_sub_expr = self.strip_group(candidates.get(min_indx+1..candidates.len()).unwrap_or_default());

            match left_sub_expr{
                Ok(opt) => {
                    if let Some(left) = self.parse(opt.unwrap_or_default()){
                        match left {
                            Ok(ast) => children.push(ast),
                            Err(e) => {
                                for err in e { errors.push(err); }
                            }
                        }
                    }
                },
                Err(e) => errors.push(e)
            }

            match right_sub_expr{
                Ok(opt) => {
                    if let Some(right) = self.parse(opt.unwrap_or_default()){
                        match right {
                            Ok(ast) => children.push(ast),
                            Err(e) => {
                                for err in e { errors.push(err); }
                            }
                        }
                        
                    }
                },
                Err(e) => errors.push(e)
            }

            if !errors.is_empty(){
                Some(Err(errors))
            }else{
                Some(Ok(AST{ kind: Expr::Operator(operator.clone()), children }))
            }
            
        }else{
            Some(Ok(AST { kind: Expr::Unknown(candidates), children: vec![] }))
        };

        result
    }



}