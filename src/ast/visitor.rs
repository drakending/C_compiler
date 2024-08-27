use crate::ast::*;
use crate::ast::lexer::TextSpan;
use crate::ast::progranunit::*;
use crate::ast::statement::*;

pub trait ASTVisitor {
    
    fn do_visit_program_unit(&mut self,program_unit:&ASTProgramunit){
        match &program_unit.kind {
            ASTProgramunitKind::Function(function) => {
                self.visit_function(function);
            }
            ASTProgramunitKind::Declaration(declaration_list) => {
                self.visit_declaration_list(declaration_list);
            }
        }
    }
    fn do_visit_statement(&mut self,statement:&ASTStatement){
        match &statement.kind {
            ASTStatementKind::Expression(expr) => {
                self.visit_expression(expr);
            }
            ASTStatementKind::Declaration(declaration_list) => {
                self.visit_declaration_list(declaration_list);
            }
            ASTStatementKind::Return(expr)=>{
                self.visit_return(expr);
            }
            ASTStatementKind::EmptyReturn=>{
                self.visit_empty_return();
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
            ASTExpressionKind::Variable(variable)=>{
                self.visit_variable(variable);
            }
            ASTExpressionKind::Assignment(assignment) =>{
                self.visit_assignment(assignment);
            }
            ASTExpressionKind::Error(span) =>{
                self.visit_error(span);
            }
            ASTExpressionKind::FunctionCall(function_call) => {
                self.visit_function_call(function_call);
            }
        }
    }
    fn visit_statement(&mut self,statement:&ASTStatement);
    fn visit_expression(&mut self,expr:&ASTExpression);
    fn visit_assignment(&mut self,assignment:&ASTAssignment);
    fn visit_number(&mut self,number:&ASTNumberExpression);
    fn visit_binary_expression(&mut self,binary:&ASTBinaryExpression);
    fn visit_error(&mut self,span:&TextSpan);
    fn visit_variable(&mut self,variable:&ASTVariableExpression);
    fn visit_declaration_list(&mut self,declaration_list:&ASTDeclarationList);
    fn visit_declaration(&mut self,declaration:&ASTDeclaration);
    fn visit_program_unit(&mut self,program_unit:&ASTProgramunit);
    fn visit_function(&mut self,function:&ASTFunction);
    fn visit_return(&mut self,expr:&ASTExpression);
    fn visit_empty_return(&mut self);
    fn visit_function_call(&mut self,function_call:&ASTFuncionCall);
}