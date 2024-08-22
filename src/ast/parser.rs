use std::cell::Cell;
use std::env::var;
use std::ops::Deref;
use crate::ast::{ASTAssignment, ASTBinaryExpression, ASTBinaryOperator, ASTBinaryOperatorKind, ASTExpression, ASTNumberExpression, ASTPrinter, ASTStatementKind, ASTVariableExpression, GrammarVartype};
use crate::ast::lexer::TokenKind;
use crate::ast::lexer::TokenKind::Eof;
use crate::diagnostics::{DiagnosticBag, DiagnosticsBagCell};
use super::ASTStatement;
use super::lexer::{Lexer,Token};
pub struct Counter{
    count:Cell<usize>,
}

impl Counter {
    pub fn new() -> Self{
        Self{count:Cell::new(0)}
    }
    pub fn increment(& self){
        self.count.set(self.count.get()+1);
    }
    pub fn count(&self) -> usize{
        self.count.get()
    }
}

pub struct Parser{
    tokens: Vec<Token>,
    current: Counter,
    diagnostics_bag: DiagnosticsBagCell,
}

impl Parser {
    pub fn new(tokens:Vec<Token>,diagnostics_bag: DiagnosticsBagCell) ->Self{
        Self{
            tokens:tokens.iter().filter(
                |token| token.kind!=TokenKind::WhiteSpace
            ).map(|token| token.clone()).collect(),
            current:Counter::new(),
            diagnostics_bag,
        }
    }

    pub fn next_statement(&mut self) -> Option<ASTStatement>{
        if self.is_at_end() {
            return None;
        }
        Some(self.parse_statement())
    }
    fn is_at_end(&self) -> bool{
        self.current().kind == TokenKind::Eof
    }
    fn parse_statement(&mut self) -> ASTStatement{
        let token = self.current();
        match &token.kind { 
            TokenKind::VarType(vartype) =>{
                let assignment =  self.parse_assignment();
                ASTStatement::assignment(assignment)
            },
            _ => {
                let expr = self.parse_expression();
                self.consume();

                ASTStatement::expression(expr)
            }
        }

    }
    fn parse_expression(&mut self) -> ASTExpression{
        self.parse_binary_expression(0)
    }
    
    fn parse_vartype(&mut self) -> Option<GrammarVartype>{
        let token = self.current();
        self.consume();
        match &token.kind {
            TokenKind::VarType(vartype) =>{
               Some(GrammarVartype::new(vartype))
            },
            _ => {
                None
            }
        }
    }
    fn parse_assignment(&mut self) -> ASTAssignment{
        let vartype = self.parse_vartype().unwrap();
        let mut token = self.current().clone();
        self.consume();
        let name = if let TokenKind::Identifier(identifier) = token.kind {
            identifier
        } else {
            self.diagnostics_bag.borrow_mut().report_unexpected_token(&TokenKind::Identifier("".to_string()),&token);
            "bad identifer".to_string()
        };
        token = self.current().clone();

        self.consume();
        let expr = self.parse_expression();
        self.consume();
        ASTAssignment::new(vartype,name,expr)
    }
    fn parse_binary_operator(&mut self) -> Option<ASTBinaryOperator>{
        let token = self.current();
        let kind = match token.kind {
            TokenKind::Plus => Some(ASTBinaryOperatorKind::Add),
            TokenKind::Minus => Some(ASTBinaryOperatorKind::Sub),
            TokenKind::Asterisk=> Some(ASTBinaryOperatorKind::Mul),
            TokenKind::Slash => Some(ASTBinaryOperatorKind::Div),
            _ => {None}
        };
        kind.map(|kind|ASTBinaryOperator::new(kind,token.clone()))
        
    }

    fn parse_binary_expression(&mut self,precedence:u8) -> ASTExpression {
        let mut left = self.parse_primary_expression();
        while let Some(operator) = self.parse_binary_operator() {
            let operator_precedence = operator.precedence();
            if operator_precedence<=precedence{
                break;
            }
            self.consume();
            let right = self.parse_binary_expression(operator_precedence);
            left = ASTExpression::binary(operator,left,right)
        }
        left
    }
    fn parse_primary_expression(&mut self) -> ASTExpression{
        let token = self.current();
        match &token.kind {
            TokenKind::Number(number) => {
                self.consume();
                ASTExpression::number(*number)
            }
            TokenKind::LeftParen => {
                self.consume();
                let expr = self.parse_expression();
                self.consume();
                expr
            }
            TokenKind::Identifier(name) =>{
                self.consume();
                ASTExpression::variable(name.clone())
            }
            _ => {
                self.diagnostics_bag.borrow_mut().report_expected_expression(token);
                self.consume();
                ASTExpression::error(
                    token.span.clone()
                )
            }
        }
    }
    fn peek(&self,offset:i32)->&Token{
        let mut index = (self.current.count() as i32 +offset) as usize;
        if index >= self.tokens.len(){
            index = self.tokens.len()-1;
        }
        self.tokens.get(index).unwrap()
    }
    fn current(&self) ->&Token{
        self.peek(0)
    }
    fn consume(&self) -> Option<&Token>{
        self.current.increment();
        let token = self.peek(-1);
        return Some(token);
    }
    fn consume_and_check(&mut self,kind:TokenKind) -> &Token{

        let token = self.consume().unwrap();

        if token.kind!=kind {
            self.diagnostics_bag.borrow_mut().report_unexpected_token(
                &kind,
                token,
            )
            ;
        }
        token
    }

}