use std::thread::panicking;

pub mod lexer;
pub mod parser;

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
    pub fn visualize(&self){
        println!("digraph ast {{");
        println!("    node [shape = box]");
        for (i,statement) in self.statements.iter().enumerate(){
            println!("   {} [label=\"{:?}\"];",i,statement);
        }
        println!("}}");
    }
}



trait ASTVisitor {

    fn visit_statement(&mut self,statement:&ASTStatement){
        match &statement.kind {
            ASTStatementKind::Expression(expr) => {
                self.visit_expression(expr);
            }
        }
    }
    fn visit_expression(&self, expr:&ASTExpression){
        match &expr.kind {
            ASTExpressionKind::Number(number) =>{
                println!("Number: {}",number);
            }
        }
    }
}

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
pub enum ASTExpressionKind{
    Number(i64),
}

pub struct ASTExpression{
    kind:ASTExpressionKind
}

impl ASTExpression {
    pub fn new(kind:ASTExpressionKind) -> Self{
        ASTExpression {kind}
    }
    pub fn number(number:i64) -> Self{
        ASTExpression::new(ASTExpressionKind::Number(number))
    }
}