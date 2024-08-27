use std::collections::HashMap;
use log::debug;
use crate::ast::{ASTBinaryExpression, ASTBinaryOperatorKind, ASTExpression, ASTNumberExpression, ASTStatement, ASTVariableExpression, ASTVisitor, GrammarVartype, ASTAssignment, LeftValue};
use crate::ast::expression::ASTFuncionCall;
use crate::ast::lexer::TextSpan;
use crate::ast::progranunit::{ASTFunction, ASTProgramunit, ASTProgramunitKind};
use crate::ast::statement::{ASTDeclaration, ASTDeclarationList};

pub struct ASTEvaluator {
    pub last_value: Option<i64>,
    variable_map: HashMap<String, Option<i64>>,
    function_map: HashMap<String, ASTFunction>,
    argument_transition: Vec<i64>,
}

impl<'a> ASTEvaluator {
    pub fn new() -> Self {
        Self {
            last_value: None,
            variable_map: HashMap::new(),
            function_map: HashMap::new(),
            argument_transition: Vec::new(),
        }
    }
}

impl<'a> ASTVisitor for ASTEvaluator {
    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.last_value = Some(0);
        self.do_visit_statement(statement);
    }



    fn visit_expression(&mut self, expr: &ASTExpression) {
        self.do_visit_expression(expr);
    }

    fn visit_assignment(&mut self, assignment: &ASTAssignment) {
        self.do_visit_expression(&assignment.expr);
        match &assignment.name {
            LeftValue::Variable(name)=> {
                self.variable_map.insert(name.clone(),self.last_value);
            }
        }
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
            ASTBinaryOperatorKind::Equal => right,
        })
    }
    fn visit_error(&mut self, span: &TextSpan) {
        self.last_value = Some(0);
    }

    fn visit_variable(&mut self, variable: &ASTVariableExpression) {
        self.last_value= Some(self.variable_map.get(&variable.name).unwrap().expect("Variable not found"));
    }

    fn visit_declaration_list(&mut self, declaration_list: &ASTDeclarationList) {
        declaration_list.declare_list.iter().for_each(|decl| self.visit_declaration(decl));
    }

    fn visit_declaration(&mut self, declaration: &ASTDeclaration) {
        match declaration {
            ASTDeclaration::VariableDeclareDirect(name) => {
                self.variable_map.insert(name.clone(), None);
            }
            ASTDeclaration::VariableDeclareWithInit(name,expr) => {
                self.do_visit_expression(expr);
                self.variable_map.insert(name.clone(),self.last_value);
            }
        }

    }

    fn visit_program_unit(&mut self, program_unit: &ASTProgramunit) {
        match &program_unit.kind {
            ASTProgramunitKind::Function(function) => {
                self.visit_function(function);
            }
            ASTProgramunitKind::Declaration(declaration_list) => {
                self.visit_declaration_list(declaration_list);
            }
        }
    }

    fn visit_function(&mut self, function:&ASTFunction) {
        self.function_map.insert(function.name.clone(),function.clone());
        if(function.name=="main"){
            self.visit_intepret_function(function);
        }
    }

    fn visit_return(&mut self, expr: &ASTExpression) {
        self.visit_expression(expr);
    }

    fn visit_empty_return(&mut self) {
        self.last_value = None;
    }

    fn visit_function_call(&mut self, function_call: &ASTFuncionCall) {
        function_call.param_list.iter().for_each(|expr|{
            self.last_value = None;
            self.do_visit_expression(expr);
            self.argument_transition.push(self.last_value.unwrap());
        });
        let func : ASTFunction;
        {
            func = self.function_map.get(&function_call.name).unwrap().clone();
        }
        self.visit_intepret_function(&func);
    }
}

impl ASTEvaluator{
    fn visit_intepret_function(&mut self, function: &ASTFunction) {
        for param in function.params.iter().rev() {
            if let Some(arg) = self.argument_transition.pop() {
                self.variable_map.insert(param.name.clone(), Some(arg));
            }
        }
        function.statements.iter().for_each(|x| self.visit_statement(x));
    }
}