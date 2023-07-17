use std::collections::{HashSet, HashMap};

use crate::lexer::{TokenKind, Token};

use super::{ASTKind, AST, ParsingError, ParsingResult};

#[derive(Debug, PartialEq)]
/// An [AST] for expression trees
pub enum ExprAST<T:TokenKind>{
    Operator(T),
    Operand(T)
}

impl<T:TokenKind> ASTKind for ExprAST<T>{}

/// A parser of expressions
/// 
/// # Exemples
/// ```rust
/// use crate::neoglot_lib::{regex::*, parser::{expression::*, *}, lexer::*};
/// use std::path::Path;
/// 
/// #[derive(Debug, Copy, Clone, Hash, PartialOrd, Eq, PartialEq)]
/// enum TokenType{A, B, ADD, SUB, MUL, OPEN_PAREN, CLOSED_PAREN}
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
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 0 },
///         kind: TokenType::A, literal: String::from("A")
///     },
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 1 },
///         kind: TokenType::ADD, literal: String::from("+")
///     },
/// 
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 2 },
///         kind: TokenType::B, literal: String::from("B")
///     }
/// ];
/// // A - B
/// let expr2 = &[
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 0 },
///         kind: TokenType::A, literal: String::from("A")
///     },
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 1 },
///         kind: TokenType::SUB, literal: String::from("+")
///     },
/// 
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 2 },
///         kind: TokenType::B, literal: String::from("B")
///     }
/// ];
/// 
/// // A +(A * B)
/// let expr3 = &[
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 0 },
///         kind: TokenType::A, literal: String::from("A")
///     },
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 1 },
///         kind: TokenType::ADD, literal: String::from("+")
///     },
/// 
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 2 },
///         kind: TokenType::OPEN_PAREN, literal: String::from("(")
///     },
/// 
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 3 },
///         kind: TokenType::A, literal: String::from("A")
///     },
/// 
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 4 },
///         kind: TokenType::MUL, literal: String::from("*")
///     },
/// 
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 5 },
///         kind: TokenType::B, literal: String::from("B")
///     },
/// 
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 6 },
///         kind: TokenType::CLOSED_PAREN, literal: String::from(")")
///     }
/// ];
/// 
/// // A - A*B
/// let expr4 = &[
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 0 },
///         kind: TokenType::A, literal: String::from("A")
///     },
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 1 },
///         kind: TokenType::SUB, literal: String::from("-")
///     },
/// 
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 2 },
///         kind: TokenType::A, literal: String::from("A")
///     },
/// 
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 3 },
///         kind: TokenType::MUL, literal: String::from("*")
///     },
/// 
///     Token{ 
///         location: Location{ file: Path::new("").to_path_buf(), line: 0, column: 4 },
///         kind: TokenType::B, literal: String::from("B")
///     },
/// 
/// ];
/// 
/// let result1 = AST{
///     kind: ExprAST::Operator(TokenType::ADD),
///     children: vec![
///         AST{ kind: ExprAST::Operand(TokenType::A), children: vec![] },
///         AST{ kind: ExprAST::Operand(TokenType::B), children: vec![] }
///     ]
/// };
/// 
/// let result2 = AST{
///     kind: ExprAST::Operator(TokenType::SUB),
///     children: vec![
///         AST{ kind: ExprAST::Operand(TokenType::A), children: vec![] },
///         AST{ kind: ExprAST::Operand(TokenType::B), children: vec![] }
///     ]
/// };
/// 
/// let result3 = AST{
///     kind: ExprAST::Operator(TokenType::ADD),
///     children: vec![
///         AST{ kind: ExprAST::Operand(TokenType::A), children: vec![] },
///         AST{ kind: ExprAST::Operator(TokenType::MUL), children: vec![
///             AST{ kind: ExprAST::Operand(TokenType::A), children: vec![] },
///             AST{ kind: ExprAST::Operand(TokenType::B), children: vec![] }
///         ] }
///     ]
/// };
/// 
/// let result4 = AST{
///     kind: ExprAST::Operator(TokenType::SUB),
///     children: vec![
///         AST{ kind: ExprAST::Operand(TokenType::A), children: vec![] },
///         AST{ kind: ExprAST::Operator(TokenType::MUL), children: vec![
///             AST{ kind: ExprAST::Operand(TokenType::A), children: vec![] },
///             AST{ kind: ExprAST::Operand(TokenType::B), children: vec![] }
///         ] }
///     ]
/// };
/// 
/// if let Some(result) = parser.parse(expr1){
///     match result{
///         ParsingResult::Ok(ast) => assert_eq!(ast, vec![result1]),
///         ParsingResult::Err(errs) => assert!(false)
///     }
/// }else { assert!(false); }
/// 
/// if let Some(result) = parser.parse(expr2){
///     match result{
///         ParsingResult::Ok(ast) => assert_eq!(ast, vec![result2]),
///         ParsingResult::Err(errs) => assert!(false)
///     }
/// }else { assert!(false); }
/// 
/// if let Some(result) = parser.parse(expr3){
///     match result{
///         ParsingResult::Ok(ast) => assert_eq!(ast, vec![result3]),
///         ParsingResult::Err(errs) => assert!(false)
///     }
/// }else { assert!(false); }
/// 
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
    fn find_min_priority(&self, candidates:&[T]) -> Option<usize>{
        let mut min_priority = None;
        let mut min_priority_indx = None;
        let mut priority_multiplier = 1;

        for i in 0..candidates.len(){
            let c = candidates[i];
            if let Some(priority) = self.priority.get(&c){

                // If we are inside a parenthesis-like bloc,
                // the priority must be multiplied
                // we also skip the bloc start/end
                if self.high_priority_group_start.is_some_and(|e| e == c){
                    priority_multiplier = priority_multiplier * 100;
                    continue;
                }else if self.high_priority_group_end.is_some_and(|e| e == c){
                    priority_multiplier = priority_multiplier / 100;
                    continue;
                }else{ priority_multiplier = 1 };

                match min_priority {
                    Some(min_p) =>{
                        if priority*priority_multiplier < min_p {
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
        let open_groups = candidates.iter().filter(|e| Some(e.kind) == self.high_priority_group_start).count();
        let closed_groups = candidates.iter().filter(|e| Some(e.kind) == self.high_priority_group_end).count();
        
        open_groups == closed_groups
    }

    /// Strips the leading and trailing groups token
    fn strip_group<'a>(&self, candidates:&'a[Token<T>]) -> Result<Option<&'a[Token<T>]>, ParsingError>{
        
        if self.high_priority_group_start.is_none()
        || self.high_priority_group_end.is_none()
        || candidates.is_empty(){
            return Ok(Some(candidates));
        }

        if !self.check_groups_validity(candidates){
            let loc = &candidates[0].location;
            return Err(ParsingError::InvalidGroups(loc.clone()))
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
    pub fn parse(&self, candidates:&[Token<T>]) -> Option<ParsingResult<ExprAST<T>>>
    {
        if candidates.is_empty(){ return None; }

        if candidates.len() == 1{
            return Some(ParsingResult::Ok(vec![AST{ kind: ExprAST::Operand(candidates[0].kind), children: vec![] }]));
        }


        let min_indx = self.find_min_priority(&candidates.iter().map(|c| c.kind).collect::<Vec<T>>());
        
        let result = if let Some(min_indx) = min_indx{
            let operator = ExprAST::Operator(candidates[min_indx].kind);

            let mut errors:Vec<ParsingError> = vec![];
            let mut children = vec![];


            let left_sub_expr = self.strip_group(candidates.get(0..min_indx).unwrap_or_default());
            let right_sub_expr = self.strip_group(candidates.get(min_indx+1..candidates.len()).unwrap_or_default());

            match left_sub_expr{
                Ok(opt) => {
                    if let Some(left) = self.parse(opt.unwrap_or_default()){
                        match left {
                            ParsingResult::Ok(ast) => ast.into_iter().for_each(|e| children.push(e)),
                            ParsingResult::Err(e) => e.into_iter().for_each(|e| errors.push(e))
                        }
                    }
                },
                Err(e) => errors.push(e)
            }

            match right_sub_expr{
                Ok(opt) => {
                    if let Some(right) = self.parse(opt.unwrap_or_default()){
                        match right {
                            ParsingResult::Ok(ast) => ast.into_iter().for_each(|e| children.push(e)),
                            ParsingResult::Err(e) => e.into_iter().for_each(|e| errors.push(e))
                        }
                        
                    }
                },
                Err(e) => errors.push(e)
            }




            if !errors.is_empty(){
                Some(ParsingResult::Err(errors))
            }else{
                Some(ParsingResult::Ok(vec![AST{ kind: operator, children }]))
            }
        }else{
            let mut errors = vec![];
            for c in candidates{
                errors.push(ParsingError::UnparsedToken(c.literal.clone(), c.location.clone()))
            }
            Some(ParsingResult::Err(errors))
        };

        result
    }

    /// Transforms an [ExprAST] into a more general [AST]
    /// 
    /// expr: The expression tree to normalize
    pub fn normalize<F, DestType: ASTKind>(expr:AST<ExprAST<T>>, transformer:F) -> AST<DestType>
    where F: Fn(ExprAST<T>) -> DestType
    {
        let normalized_kind = (transformer)(expr.kind);
        let normalized_children:Vec<AST<DestType>> = expr.children.into_iter().map(|c| Self::normalize(c, &transformer)).collect();


        AST{ kind: normalized_kind, children: normalized_children }
    }


}