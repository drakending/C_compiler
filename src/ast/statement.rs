use crate::ast::{ASTExpression, GrammarVartype};

#[derive(Debug,Clone)]
pub enum ASTStatementKind{
    Expression(ASTExpression),
    Declaration(ASTDeclarationList),
    Return(ASTExpression),
    EmptyReturn,
}
#[derive(Debug,Clone)]
pub struct ASTStatement{
    pub(crate) kind:ASTStatementKind,
}

impl ASTStatement {
    pub fn new(kind: ASTStatementKind) -> Self {
        ASTStatement { kind }
    }
    pub fn expression(expr: ASTExpression) -> Self {
        ASTStatement::new(ASTStatementKind::Expression(expr))
    }
    pub fn declaration(declaration_list: ASTDeclarationList) -> Self {
        ASTStatement::new(ASTStatementKind::Declaration(declaration_list))
    }
    pub fn return_statement(expr: ASTExpression) -> Self {
        ASTStatement::new(ASTStatementKind::Return(expr))
    }
    pub fn empty_return() -> Self {
        ASTStatement::new(ASTStatementKind::EmptyReturn)
    }
}


#[derive(Debug,Clone)]
pub enum ASTDeclaration{
    VariableDeclareDirect(String),
    VariableDeclareWithInit(String,ASTExpression),
}

#[derive(Debug,Clone)]
pub struct ASTDeclarationList{
    pub(crate) vartype:GrammarVartype,
    pub(crate) declare_list:Vec<ASTDeclaration>
}

impl ASTDeclarationList{
    pub fn new(vartype:GrammarVartype) -> Self{
        Self{vartype,declare_list:Vec::new()}
    }
}


