use std::fmt::format;
use std::thread::panicking;
use crate::ast::lexer::{TextSpan, Token, TokenKind};

pub mod lexer;
pub mod parser;
pub mod evaluator;

pub struct Ast{
    pub statements: Vec<ASTStatement>
}

impl Ast {
    pub fn new() -> Self{
        Self{statements: Vec::new()}
    }
    pub fn add_statement(&mut self,statement:ASTStatement){
        self.statements.push(statement);
    }
    pub fn visit(&self,visitor:&mut dyn ASTVisitor){
        for statement in &self.statements{
            visitor.visit_statement(statement);
        }
    }
    pub fn visualize(&self){
        let mut printer = ASTPrinter{indent:0};
        self.visit(&mut printer);
    }
}



pub trait ASTVisitor {

    fn do_visit_statement(&mut self,statement:&ASTStatement){
        match &statement.kind {
            ASTStatementKind::Expression(expr) => {
                self.visit_expression(expr);
            }
        }
    }
    fn do_visit_expression(&mut self, expr:&ASTExpression){
        match &expr.kind {
            ASTExpressionKind::Number(number) =>{
                self.visit_number(number);
            }
            ASTExpressionKind::Binary(binary) => {
                self.visit_binary_expression(binary);
            }
            ASTExpressionKind::Error(span) =>{
                self.visit_error(span);
            }
        }
    }
    fn visit_statement(&mut self,statement:&ASTStatement);
    fn visit_expression(&mut self,expr:&ASTExpression);
    fn visit_number(&mut self,number:&ASTNumberExpression);
    fn visit_binary_expression(&mut self,binary:&ASTBinaryExpression);
    fn visit_error(&mut self,span:&TextSpan);
}

pub struct ASTPrinter{
    indent : usize,
}
const LEVEL_INDENT: usize = 2;
impl ASTVisitor for ASTPrinter {
    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.print_with_indent("Statement:");
        self.indent+=LEVEL_INDENT;
        self.do_visit_statement(statement);
        self.indent-=LEVEL_INDENT;

    }

    fn visit_expression(&mut self, expr: &ASTExpression) {
        self.print_with_indent("Expression:");
        self.indent+=LEVEL_INDENT;
        self.do_visit_expression(expr);
        self.indent-=LEVEL_INDENT;
    }
    fn visit_number(&mut self, number: &ASTNumberExpression) {
        self.print_with_indent(&format!("Number: {}",number.value));
    }

    fn visit_binary_expression(&mut self, binary: &ASTBinaryExpression) {
        self.print_with_indent("Binary Expression:");
        self.indent+=LEVEL_INDENT;
        self.print_with_indent(&format!("Operator: {:?}",binary.operator.kind));
        self.do_visit_expression(&binary.left);
        self.do_visit_expression(&binary.right);
        self.indent-=LEVEL_INDENT;
    }
    fn visit_error(&mut self, span: &TextSpan) {
        self.print_with_indent(&format!("Error: {}",span));
    }
}
impl ASTPrinter{
    fn print_with_indent(&mut self,text:&str){
        println!("{}{}"," ".repeat(self.indent),text)
    }
}
#[derive(Debug)]
pub enum ASTStatementKind{
    Expression(ASTExpression),
}
#[derive(Debug)]
pub struct ASTStatement{
    kind:ASTStatementKind,
}

impl ASTStatement {
    pub fn new(kind:ASTStatementKind) -> Self{
        ASTStatement {kind}
    }
    pub fn expression(expr:ASTExpression) -> Self{
        ASTStatement::new(ASTStatementKind::Expression(expr))
    }
}
#[derive(Debug)]
pub enum ASTExpressionKind{
    Number(ASTNumberExpression),
    Binary(
        ASTBinaryExpression
    ),
    Error(
        TextSpan
    )
}
#[derive(Debug)]
pub struct ASTExpression{
    kind:ASTExpressionKind
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
}
#[derive(Debug)]
pub struct ASTNumberExpression{
    value:i64
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
}

#[derive(Debug,Clone)]
pub struct ASTBinaryOperator{
    kind:ASTBinaryOperatorKind,
    token:Token,
}

impl ASTBinaryOperator {
    pub fn new(kind:ASTBinaryOperatorKind,token:Token) -> Self{
        ASTBinaryOperator { kind, token }
    }
    pub fn precedence(&self) -> u8{
        match self.kind{
            ASTBinaryOperatorKind::Add => 1,
            ASTBinaryOperatorKind::Sub => 1,
            ASTBinaryOperatorKind::Mul => 2,
            ASTBinaryOperatorKind::Div => 2,
        }
    }
}

#[derive(Debug)]
pub struct ASTBinaryExpression{
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
    operator:ASTBinaryOperator,
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
