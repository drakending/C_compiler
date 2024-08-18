use std::ops::Deref;
use crate::ast::{ASTBinaryExpression, ASTBinaryOperator, ASTBinaryOperatorKind, ASTExpression, ASTNumberExpression, ASTPrinter, ASTStatementKind};
use crate::ast::lexer::TokenKind;
use crate::ast::lexer::TokenKind::Eof;
use crate::diagnostics::{DiagnosticBag, DiagnosticsBagCell};
use super::ASTStatement;
use super::lexer::{Lexer,Token};
pub struct Parser{
    tokens: Vec<Token>,
    current: usize,
    diagnostics_bag: DiagnosticsBagCell,
}

impl Parser {
    pub fn new(tokens:Vec<Token>,diagnostics_bag: DiagnosticsBagCell) ->Self{
        Self{
            tokens:tokens.iter().filter(
                |token| token.kind!=TokenKind::WhiteSpace
            ).map(|token| token.clone()).collect(),
            current:0,
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
        let expr = self.parse_expression();
        ASTStatement::expression(expr)
    }
    fn parse_expression(&mut self) -> ASTExpression{
        self.parse_binary_expression(0)
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
        match token.kind {
            TokenKind::Number(number) => {
                self.consume();
                ASTExpression::number(number)
            }
            TokenKind::LeftParen => {
                self.consume();
                let expr = self.parse_expression();
                self.consume();
                expr
            }
            _ => {
                self.diagnostics_bag.borrow_mut().report_expected_expression(token);
                ASTExpression::error(
                    token.span.clone()
                )
            }
        }
    }
    fn peek(&self,offset:i32)->&Token{
        let mut index = (self.current as i32 +offset) as usize;
        if index >= self.tokens.len(){
            index = self.tokens.len()-1;
        }
        self.tokens.get(index).unwrap()
    }
    fn current(&self) ->&Token{
        self.peek(0)
    }
    fn consume(&mut self) -> Option<&Token>{
        self.current += 1;
        let token = self.peek(-1);
        return Some(token);
    }
    fn consume_and_check(&mut self,kind:TokenKind) -> &Token{
        let token = self.consume().unwrap();
        if token.kind!=kind {
            self.diagnostics_bag.borrow().report_unexpected_token(
                &kind,
                token,
            )
            ;
        }
        token
    }

}