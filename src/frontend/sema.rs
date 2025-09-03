use super::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Type { Int, Bool, Function, Unknown }

pub struct SemanticAnalyzer {
    symbol_table: HashMap<String, Type>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut symbols = HashMap::new();
        symbols.insert("print".to_string(), Type::Function);
        Self { symbol_table: symbols }
    }

    pub fn analyze(&mut self, program: &mut Program) -> Result<(), String> {
        for stmt in program { self.check_statement(stmt)?; }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Statement) -> Result<Type, String> {
        match stmt {
            Statement::Let { name, value } => {
                let val_type = self.check_expression(value)?;
                self.symbol_table.insert(name.clone(), val_type);
                Ok(Type::Unknown)
            }
            Statement::Return(expr) => self.check_expression(expr),
            Statement::Expression(expr) => self.check_expression(expr),
        }
    }

    fn check_expression(&mut self, expr: &Expression) -> Result<Type, String> {
        match expr {
            Expression::IntegerLiteral(_) => Ok(Type::Int),
            Expression::Boolean(_) => Ok(Type::Bool),
            Expression::Identifier(name) => self.symbol_table.get(name).cloned().ok_or_else(|| format!("Undeclared variable: {}", name)),
            Expression::Infix { op:_, left, right } => {
                let left_type = self.check_expression(left)?;
                let right_type = self.check_expression(right)?;
                if left_type != Type::Int || right_type != Type::Int {
                    return Err(format!("Cannot perform arithmetic on non-integers. Left is {:?}, Right is {:?}", left_type, right_type));
                }
                Ok(Type::Int)
            },
            Expression::If { condition, consequence, .. } => {
                if self.check_expression(condition)? != Type::Bool {
                    return Err("If condition must be a boolean".to_string());
                }
                for stmt in consequence { self.check_statement(stmt)?; }
                Ok(Type::Unknown)
            },
            Expression::Call { function, arguments } => {
                 if let Expression::Identifier(name) = &**function {
                    let func_type = self.symbol_table.get(name).ok_or_else(|| format!("Undeclared function: {}", name))?;
                    if *func_type != Type::Function { return Err(format!("'{}' is not a function", name)); }
                    for arg in arguments { self.check_expression(arg)?; }
                    Ok(Type::Unknown)
                } else { Err("Can only call named functions".to_string()) }
            },
            _ => Err("Unsupported expression type".to_string())
        }
    }
}
