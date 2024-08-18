use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::lexer::{TextSpan, Token, TokenKind};

pub enum DiagnosticKind {
    Error,
    Warning,
}

pub struct Diagnostic{
    pub message: String,
    pub span: TextSpan,
    pub kind: DiagnosticKind,
}

impl Diagnostic{
    pub fn new(message:String,span:TextSpan,kind:DiagnosticKind) -> Self {
        Self {
            message,
            span,
            kind
        }
    }
}

pub type DiagnosticsBagCell = Rc<RefCell<DiagnosticBag>>;
pub struct DiagnosticBag{
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBag{
    pub fn new() -> Self {
        Self { diagnostics: Vec::new() }
    }
    pub fn report_error(&mut self,message:String,span:TextSpan) {
        self.diagnostics.push(Diagnostic::new(message, span, DiagnosticKind::Error));
    }
    pub fn report_warning(&mut self,message:String,span:TextSpan) {
        self.diagnostics.push(Diagnostic::new(message, span, DiagnosticKind::Warning));
    }

    pub fn report_unexpected_token(&mut self, expected: &TokenKind, token: &Token){
        self.report_error(
            format!("Unexpected token: {:?} expected: {:?}",token.kind,expected),
            token.span.clone()
        )
    }
    pub fn report_expected_expression(&mut self, token: &Token) {
        self.report_error(
            format!("Expected expression, found: {:?}", token.kind),
            token.span.clone()
        )
    }
}