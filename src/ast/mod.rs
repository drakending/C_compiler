use std::fmt::Display;
use crate::ast::expression::*;
use crate::ast::statement::*;
use crate::ast::visitor::*;
use crate::ast::printer::*;
pub mod lexer;
pub mod parser;
pub mod evaluator;

pub mod statement;
pub mod visitor;
pub mod printer;
mod expression;

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

#[derive(Debug)]
pub enum GrammarVartype{
    Direct(lexer::VartypeKind),
}
impl GrammarVartype{
    pub fn new( vartype:&lexer::VartypeKind) -> Self{
        GrammarVartype::Direct(vartype.clone())
    }
}

#[derive(Debug)]
pub enum LeftValue {
    Variable(String),
}

impl LeftValue {
    pub fn variable(name:ASTExpression) -> Self{
        match name.kind {
            ASTExpressionKind::Variable(variable) => LeftValue::Variable(variable.name),
            _ => unreachable!()
        }
    }
}

impl Display for LeftValue{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            LeftValue::Variable(name) => write!(f, "{}", name),
        }
    }
}
