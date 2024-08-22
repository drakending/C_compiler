use std::fmt::Display;
use crate::ast::LeftValue;
use crate::ast::lexer::{TextSpan, Token};
#[derive(Debug)]
pub enum ASTExpressionKind{
    Number(ASTNumberExpression),
    Variable(ASTVariableExpression),
    Binary(ASTBinaryExpression),
    Error(TextSpan),
    Assignment(ASTAssignment),
}

#[derive(Debug)]
pub struct ASTExpression{
    pub(crate) kind:ASTExpressionKind
}

impl ASTExpression {
    pub fn new(kind:ASTExpressionKind) -> Self{
        ASTExpression {kind}
    }
    pub fn number(number:i64) -> Self{
        ASTExpression::new(ASTExpressionKind::Number(ASTNumberExpression::new(number)))
    }
    pub fn binary( operator:ASTBinaryOperator,left:ASTExpression,right:ASTExpression) -> Self{
        ASTExpression::new(ASTExpressionKind::Binary(ASTBinaryExpression::new(left,right,operator)))
    }
    pub fn variable(name:String) -> Self{
        ASTExpression::new(ASTExpressionKind::Variable(ASTVariableExpression::new(name)))
    }
    pub fn assignment(name:ASTExpression,expr:ASTExpression) -> Self{
        ASTExpression::new(ASTExpressionKind::Assignment(ASTAssignment::new(name,expr)))
    }
    pub fn error(span:TextSpan) -> Self {
        ASTExpression::new(ASTExpressionKind::Error(span))
    }
}

#[derive(Debug)]
pub struct ASTVariableExpression{
    pub(crate) name:String
}

impl ASTVariableExpression {
    pub fn new(name:String) -> Self {
        ASTVariableExpression {name}
    }
}

#[derive(Debug)]
pub struct ASTNumberExpression{
    pub(crate) value:i64
}

impl ASTNumberExpression {
    pub fn new(value:i64) -> Self {
        ASTNumberExpression { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}


#[derive(Debug,Clone)]
pub enum ASTBinaryOperatorKind{
    Add,
    Sub,
    Mul,
    Div,
    Equal
}

#[derive(Debug,Clone)]
pub struct ASTBinaryOperator{
    pub(crate) kind:ASTBinaryOperatorKind,
    token:Token,
}

impl ASTBinaryOperator {
    pub fn new(kind:ASTBinaryOperatorKind,token:Token) -> Self{
        ASTBinaryOperator { kind, token }
    }
    pub fn precedence(&self) -> u8{
        match self.kind{
            ASTBinaryOperatorKind::Add => 2,
            ASTBinaryOperatorKind::Sub => 2,
            ASTBinaryOperatorKind::Mul => 3,
            ASTBinaryOperatorKind::Div => 3,
            ASTBinaryOperatorKind::Equal => 1,
        }
    }
    pub fn right_combined(&self) -> bool{
        match self.kind {
            ASTBinaryOperatorKind::Equal => true,
            _ => false
        }
    }
}

#[derive(Debug)]
pub struct ASTBinaryExpression{
    pub(crate) left: Box<ASTExpression>,
    pub(crate) right: Box<ASTExpression>,
    pub(crate) operator:ASTBinaryOperator,
}

impl ASTBinaryExpression {
    fn new(left:ASTExpression,right:ASTExpression,operator:ASTBinaryOperator) -> Self{
        ASTBinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        }
    }
}



#[derive(Debug)]
pub struct ASTAssignment{
    pub(crate) name: LeftValue,
    pub(crate) expr: Box<ASTExpression>,
}

impl ASTAssignment{
    pub fn new(name:ASTExpression,expr:ASTExpression) -> Self{
        ASTAssignment {
            name: LeftValue::variable(name),
            expr: Box::new(expr),
        }
    }
}