use std::fmt::Display;
use crate::ast::expression::*;
use crate::ast::statement::*;
use crate::ast::visitor::*;
use crate::ast::printer::*;
use crate::ast::progranunit::ASTProgramunit;

pub mod lexer;
pub mod parser;

pub mod statement;
pub mod visitor;
pub mod printer;
mod expression;
mod progranunit;

pub struct Ast{
    pub program_units: Vec<ASTProgramunit>
}

impl Ast {
    pub fn new() -> Self{
        Self{program_units: Vec::new()}
    }
    pub fn add_program_unit(&mut self,program_unit:ASTProgramunit){
        self.program_units.push(program_unit);
    }
    pub fn visit(&self,visitor:&mut dyn ASTVisitor){
        for program_unit in &self.program_units{
            visitor.visit_program_unit(program_unit);
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
