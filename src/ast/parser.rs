use std::cell::Cell;
use std::env::var;
use crate::ast::*;
use crate::ast::lexer::TokenKind;
use crate::ast::lexer::TokenKind::Plus;
use crate::ast::progranunit::*;
use crate::diagnostics::*;
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
            ).cloned().collect(),
            current:Counter::new(),
            diagnostics_bag,
        }
    }
    pub fn next_program_unit(&mut self) -> Option<ASTProgramunit>{
        if self.is_at_end() {
            return None;
        }
        Some(self.parse_program_unit())
    }
    pub fn parse_program_unit(&mut self) -> ASTProgramunit{
        let token2 = self.peek(2);
        match token2.kind { 
            TokenKind::LeftParen =>{
                let function = self.parse_function();
                ASTProgramunit::function(function)
            },
            _ => {
                let declaration_list  =  self.parse_declaration_list();
                ASTProgramunit::declaration(declaration_list)
            }
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
                let declaration_list  =  self.parse_declaration_list();
                ASTStatement::declaration(declaration_list)
            },
            TokenKind::Return => {
                self.consume();
                let next_token = self.current();
                if let TokenKind::SemiColon = next_token.kind{
                    self.consume();
                    return ASTStatement::empty_return();
                }
                let expr = self.parse_expression();
                self.consume();
                ASTStatement::return_statement(expr)
            }
            _ => {
                let expr = self.parse_expression();
                self.consume();
                ASTStatement::expression(expr)
            }
        }

    }
    fn parse_function_params(&mut self) -> ASTFunctonParam{
        let var_type = self.parse_vartype().unwrap();
        let name_token = self.consume().unwrap().clone();
        if let TokenKind::Identifier(name)=name_token.kind{
            ASTFunctonParam::new(var_type,name)
        } else{
            panic!()
        }
    }
    fn parse_function(&mut self) -> ASTFunction {
        let var_type = self.parse_vartype().unwrap();
        let mut function = ASTFunction::new(var_type);
        let name_token = self.consume().unwrap().clone();
        if let TokenKind::Identifier(name)=name_token.kind{
            function.name = name;
        } else{
            panic!()
        }
        let left_paren = self.consume().unwrap();
        if let TokenKind::LeftParen=left_paren.kind{
        } else{
            panic!()
        }
        loop {
            let current_token  = self.current();
            match current_token.kind { 
                TokenKind::RightParen =>{
                    self.consume();
                    break;
                },
                TokenKind::Comma => {
                    self.consume();
                    function.params.push(self.parse_function_params());

                }
                _ => {
                    function.params.push(self.parse_function_params());
                }
            }
        }
        let left_brace = self.consume().unwrap();
        if let TokenKind::LeftBrace=left_brace.kind{
        } else{
            panic!()
        }
        loop {
            let statement = self.parse_statement();
            function.statements.push(statement);
            let current = self.current();
            if(current.kind==TokenKind::RightBrace) {
                self.consume();
                break;
            }
        }
        function
    }
    fn parse_declaration_list(&mut self) -> ASTDeclarationList{
        let vartype = self.parse_vartype().unwrap();
        let mut declaration_list = ASTDeclarationList::new(vartype);
        
        loop{
            let declaration = self.parse_declararion();
            declaration_list.declare_list.push(declaration);
            let current = self.consume().unwrap();
            if(current.kind!=TokenKind::Comma) {
                break;
            }
        }
        
        declaration_list
    }
    
    fn parse_declararion(&mut self) -> ASTDeclaration{
        let var = self.consume().unwrap().clone();
        let ahead1 = self.current();
        match &ahead1.kind {
            TokenKind::Comma|TokenKind::SemiColon => {
                if let TokenKind::Identifier(name)=var.kind{
                    self.consume();
                    return  ASTDeclaration::VariableDeclareDirect(name);
                } else{
                    panic!()
                }
            },
            TokenKind::Equal => {
                self.consume();
                let expr = self.parse_expression();
                if let TokenKind::Identifier(name)=var.kind{
                    return  ASTDeclaration::VariableDeclareWithInit(name,expr);
                } else{
                    panic!()
                }
            },
            _ => {
                panic!()
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

    fn parse_binary_operator(&mut self) -> Option<ASTBinaryOperator>{
        let token = self.current();
        let kind = match token.kind {
            TokenKind::Plus => Some(ASTBinaryOperatorKind::Add),
            TokenKind::Minus => Some(ASTBinaryOperatorKind::Sub),
            TokenKind::Asterisk=> Some(ASTBinaryOperatorKind::Mul),
            TokenKind::Slash => Some(ASTBinaryOperatorKind::Div),
            TokenKind::Equal => Some(ASTBinaryOperatorKind::Equal),
            _ => {None}
        };
        kind.map(|kind|ASTBinaryOperator::new(kind,token.clone()))
        
    }

    fn parse_binary_expression(&mut self,precedence:u8) -> ASTExpression {
        let mut left = self.parse_primary_expression();
        while let Some(operator) = self.parse_binary_operator() {
            let operator_precedence = operator.precedence();
            if operator_precedence < precedence || (operator_precedence == precedence && !operator.right_combined()){
                break;
            }
            self.consume();
            let right = self.parse_binary_expression(operator_precedence);
            match operator.kind { 
                ASTBinaryOperatorKind::Equal => {
                    left = ASTExpression::assignment(left,right)
                }
                _ => {
                    left = ASTExpression::binary(operator,left,right)
                }
            }
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