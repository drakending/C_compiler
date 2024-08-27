use crate::ast::*;
use crate::ast::lexer::TextSpan;
use crate::ast::progranunit::ASTFunction;
use crate::ast::statement::ASTStatement;
use crate::ast::visitor::ASTVisitor;

pub struct ASTPrinter{
    pub(crate) indent : usize,
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

    fn visit_assignment(&mut self, assignment: &ASTAssignment) {
        self.print_with_indent("Assignment:");
        self.indent+=LEVEL_INDENT;
        self.print_with_indent(&format!("Name: {}",assignment.name));
        self.do_visit_expression(&assignment.expr);
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
        self.print_with_indent(&format!("Error: {:?}",span));
    }

    fn visit_variable(&mut self, variable: &ASTVariableExpression) {
        self.print_with_indent(&format!("Variable: {}",variable.name));
    }
    fn visit_declaration_list(&mut self, declaration_list: &ASTDeclarationList) {
        self.print_with_indent("Declaration List:");
        self.indent+=LEVEL_INDENT;
        self.print_with_indent(&format!("Vartype: {:?}",declaration_list.vartype));
        declaration_list.declare_list.iter().for_each(|x|self.visit_declaration(x));
        self.indent-=LEVEL_INDENT;
    }

    fn visit_declaration(&mut self, declaration: &ASTDeclaration) {
        match declaration { 
            ASTDeclaration::VariableDeclareDirect(name) => {
                self.print_with_indent(&format!("Variable: {}",name));
            }
            ASTDeclaration::VariableDeclareWithInit(name,expr) => {
                self.print_with_indent(&format!("Variable: {}",name));
                self.do_visit_expression(expr);
            }
        }
    }

    fn visit_program_unit(&mut self, program_unit: &ASTProgramunit) {
        self.print_with_indent("Program Unit:");
        self.indent+=LEVEL_INDENT;
        self.do_visit_program_unit(program_unit);
        self.indent-=LEVEL_INDENT;
    }

    fn visit_function(&mut self, function: &ASTFunction) {
        self.print_with_indent(&format!("Function: {}",function.name));
        self.indent+=LEVEL_INDENT;
        function.statements.iter().for_each(|x|self.visit_statement(x));
        self.indent-=LEVEL_INDENT;
    }

    fn visit_return(&mut self, expr: &ASTExpression) {
        self.print_with_indent("Return:");
        self.indent+=LEVEL_INDENT;
        self.do_visit_expression(expr);
        self.indent-=LEVEL_INDENT;
    }

    fn visit_empty_return(&mut self) {
        self.print_with_indent("Empty Return");
    }

    fn visit_function_call(&mut self, function_call: &ASTFuncionCall) {
        self.print_with_indent(&format!("Function call: {}",function_call.name));
        self.indent+=LEVEL_INDENT;
        function_call.param_list.iter().for_each(|expr|self.do_visit_expression(expr));
        self.indent-=LEVEL_INDENT;
    }
}

impl ASTPrinter{
    fn print_with_indent(&mut self,text:&str){
        println!("{}{}"," ".repeat(self.indent),text)
    }
}