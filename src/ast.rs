use std::fmt::Debug;

/// A trait representing the type of an [AST]
pub trait ASTKind : PartialEq+Debug{}

#[derive(Debug, PartialEq)]
/// An Abstract Syntax Tree is a semantical unit generated from [tokens](Token)
/// 
/// children: The children of this node
/// 
/// [kind](ASTKind): The type of this AST 
pub struct AST<Kind: ASTKind>{
    pub children:Vec<AST<Kind>>,
    pub kind: Kind
}
