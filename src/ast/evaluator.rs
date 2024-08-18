use crate::ast::{ASTBinaryExpression, ASTBinaryOperatorKind, ASTExpression, ASTNumberExpression, ASTStatement, ASTVisitor};
use crate::ast::lexer::TextSpan;

pub struct ASTEvaluator{
    pub last_value:Option<i64>,
}

impl ASTEvaluator {
    pub fn new() -> Self{
        Self{
            last_value:None
        }
    }
}

impl ASTVisitor for ASTEvaluator{
    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement);
    }

    fn visit_expression(&mut self, expr: &ASTExpression) {
        self.do_visit_expression(expr);
    }

    fn visit_number(&mut self, number: &ASTNumberExpression) {
        self.last_value = Some(number.value);
    }

    fn visit_binary_expression(&mut self, binary: &ASTBinaryExpression) {
        self.visit_expression(&binary.left);
        let left = self.last_value.unwrap();
        self.visit_expression(&binary.right);
        let right = self.last_value.unwrap();
        self.last_value = Some(match binary.operator.kind {
            ASTBinaryOperatorKind::Add => left + right,
            ASTBinaryOperatorKind::Sub => left - right,
            ASTBinaryOperatorKind::Mul => left * right,
            ASTBinaryOperatorKind::Div => left / right,
        })
    }
    fn visit_error(&mut self, span: &TextSpan) {
        self.last_value = Some(0);
    }
}