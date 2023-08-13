use std::collections::{HashSet, HashMap};

use crate::lexer::{TokenKind, Token};

use super::AST;

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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Position{
    Prefix,
    Infix,
    Sufix
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Operator<T:TokenKind>{
    pub kind:T,
    pub position: Position
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
/// parser.add_operator(Operator{kind: TokenType::ADD, position: Position::Infix}, 1);
/// parser.add_operator(Operator{kind: TokenType::SUB, position: Position::Infix}, 1);
/// parser.add_operator(Operator{kind: TokenType::MUL, position: Position::Infix}, 2);
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
///     assert_eq!(result, result1);
/// }else { assert!(false); }
/// 
/// if let Some(result) = parser.parse(expr2){
///     assert_eq!(result, result2);
/// }else { assert!(false); }
/// 
/// if let Some(result) = parser.parse(expr3){
///     assert_eq!(result, result3);
/// }else { assert!(false); }
/// 
/// if let Some(result) = parser.parse(expr4){
///     assert_eq!(result, result4);
/// }else { assert!(false); }
/// 
/// if let Some(result) = parser.parse(expr5){
///     assert_eq!(result, result5);
/// }else { assert!(false); }
/// 
/// 
/// ```
pub struct ExpressionParser<T: TokenKind>{
    /// Set of known operators
    operators:HashSet<Operator<T>>,

    /// [HashMap] of operators and their priority
    priority:HashMap<Operator<T>, usize>,

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
    pub fn add_operator(&mut self, operator:Operator<T>, priority:usize){
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

            // If we are inside a parenthesis-like bloc,
            // the priority must be multiplied
            // we also skip the bloc start/end
            if self.high_priority_group_start.is_some_and(|e| e == c.kind){
                priority_multiplier = priority_multiplier * 100;
                continue;
            }else if self.high_priority_group_end.is_some_and(|e| e == c.kind){
                priority_multiplier = priority_multiplier / 100;
                continue;
            }/*else{ priority_multiplier = 1 };*/

            if let Some(operator) = self.operators.iter().find(|e| e.kind == c.kind){
                let priority = self.priority.get(&operator).unwrap();

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
        if self.high_priority_group_start.is_none()
        || self.high_priority_group_end.is_none()
        || candidates.is_empty(){
            return false;
        }
        let mut open_groups = 0;
        for c in candidates{
            if c.kind == self.high_priority_group_start.unwrap(){ open_groups += 1; }
            else if c.kind == self.high_priority_group_end.unwrap(){ open_groups -= 1; }

            if open_groups < 0{ break; }
        }
        
        open_groups == 0
    }

    /// Checks if candidates is in the form '(...)'
    fn is_in_group(&self, candidates:&[Token<T>]) -> bool{
        if self.high_priority_group_start.is_none()
        || self.high_priority_group_end.is_none()
        || candidates.is_empty(){
            return false;
        }


        let mut open_groups = 0;
        let mut in_group = true;

        for token in candidates{
            if token.kind == self.high_priority_group_start.unwrap(){
                open_groups += 1;
                continue;
            }
            if token.kind == self.high_priority_group_end.unwrap(){
                open_groups -= 1;
                continue;
            }

            if open_groups <= 0 {
                in_group = false;
                break;
            }
        }



        in_group
    }

    /// Strips leading and trailing group
    fn strip_group<'a>(&self, candidates:&'a[Token<T>]) -> Option<&'a[Token<T>]>{
        candidates.get(1..candidates.len()-1)
    }


    /// Parse an expression
    pub fn parse<'a>(&self, candidates:&'a[Token<T>]) -> Option<AST<Expr<'a, T>>>
    {
        if candidates.is_empty(){ return None; }

        if candidates.len() == 1{
            // Do not accept operators without operands
            if let Some(_) = self.operators.iter().find(|e| e.kind == candidates[0].kind){
                return None;
            }else{
                return Some(AST{ kind: Expr::Operand(candidates[0].clone()), children: vec![] });
            }
        }

        if !self.check_groups_validity(candidates){
            return None;
        }

        if self.is_in_group(candidates){
            return self.parse(self.strip_group(candidates).unwrap_or_default());
        }

        let min_indx = self.find_min_priority(candidates);
        
        let result = if let Some(min_indx) = min_indx{
            let operator_token = &candidates[min_indx];
            let operator = self.operators.iter().find(|e| e.kind == operator_token.kind).unwrap();

            let mut sucess = true;
            let mut children = vec![];


            let left_sub_expr = candidates.get(0..min_indx).unwrap_or_default();
            let right_sub_expr = candidates.get(min_indx+1..).unwrap_or_default();

            match operator.position{
                Position::Prefix => {
                    if min_indx != 0{ sucess = false; }
                    else{
                        if let Some(right) = self.parse(right_sub_expr){
                            children.push(right);
                        }else{ sucess = false; }
                    }
                },

                Position::Infix => {
                    let left = self.parse(left_sub_expr);
                    let right = self.parse(right_sub_expr);

                    if left.is_none() && right.is_none(){
                        sucess = false;
                    }else{
                        if let Some(left) = left { children.push(left); }
        
                        if let Some(right) = right { children.push(right); }
                    }
                },

                Position::Sufix => {
                    if min_indx != candidates.len()-1 { sucess = false; }
                    else{
                        if let Some(left) = self.parse(left_sub_expr){
                            children.push(left);
                        }else{ sucess = false; }
                    }
                }
            }

            

            


            if !sucess{
                None
            }else{
                Some(AST{ kind: Expr::Operator(operator_token.clone()), children })
            }
            
        }else{
            Some(AST { kind: Expr::Unknown(candidates), children: vec![] })
        };

        result
    }



}