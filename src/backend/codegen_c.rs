use crate::frontend::{ast::*, token::Token};
use std::fmt::{self, Write};

pub struct CTranspiler {
    output: String, indent_level: usize,
}

impl CTranspiler {
    pub fn new() -> Self { Self { output: String::new(), indent_level: 1 } }

    pub fn transpile(&mut self, program: Program) -> Result<String, fmt::Error> {
        writeln!(&mut self.output, "#include <stdio.h>")?;
        writeln!(&mut self.output, "#include <stdint.h>")?;
        writeln!(&mut self.output, "#include <stdbool.h>\n")?;
        writeln!(&mut self.output, "int main() {{")?;
        for stmt in program { self.transpile_statement(&stmt)?; }
        writeln!(&mut self.output, "    return 0;")?;
        writeln!(&mut self.output, "}}")?;
        Ok(self.output.clone())
    }

    fn indent(&mut self) -> fmt::Result { write!(&mut self.output, "{}", "    ".repeat(self.indent_level)) }

    fn transpile_statement(&mut self, stmt: &Statement) -> fmt::Result {
        self.indent()?;
        match stmt {
            Statement::Let { name, value } => {
                write!(&mut self.output, "int64_t {} = ", name)?;
                self.transpile_expression(value)?;
                writeln!(&mut self.output, ";")?;
            }
            Statement::Return(expr) => {
                write!(&mut self.output, "return ")?;
                self.transpile_expression(expr)?;
                writeln!(&mut self.output, ";")?;
            }
            Statement::Expression(expr) => {
                self.transpile_expression(expr)?;
                writeln!(&mut self.output, ";")?;
            }
        }
        Ok(())
    }
    
    fn transpile_expression(&mut self, expr: &Expression) -> fmt::Result {
        match expr {
            Expression::Identifier(name) => write!(&mut self.output, "{}", name)?,
            Expression::IntegerLiteral(val) => write!(&mut self.output, "{}", val)?,
            Expression::Boolean(val) => write!(&mut self.output, "{}", val)?,
            Expression::Infix { op, left, right } => {
                write!(&mut self.output, "(")?;
                self.transpile_expression(left)?;
                write!(&mut self.output, " {} ", Self::op_to_c(op))?;
                self.transpile_expression(right)?;
                write!(&mut self.output, ")")?;
            }
            Expression::If { condition, consequence, alternative } => {
                write!(&mut self.output, "if (")?;
                self.transpile_expression(condition)?;
                writeln!(&mut self.output, ") {{")?;
                self.indent_level += 1;
                for stmt in consequence { self.transpile_statement(stmt)?; }
                self.indent_level -= 1;
                self.indent()?;
                write!(&mut self.output, "}}")?;
                if let Some(alt) = alternative {
                    writeln!(&mut self.output, " else {{")?;
                    self.indent_level += 1;
                    for stmt in alt { self.transpile_statement(stmt)?; }
                    self.indent_level -= 1;
                    self.indent()?;
                    write!(&mut self.output, "}}")?;
                }
            }
            Expression::Call { function, arguments } => {
                 if let Expression::Identifier(name) = &**function {
                    if name == "print" {
                        write!(&mut self.output, "printf(\"%lld\\n\", ")?;
                        if let Some(arg) = arguments.get(0) { self.transpile_expression(arg)?; }
                        write!(&mut self.output, ")")?;
                    } else {
                        write!(&mut self.output, "{}(", name)?;
                        // In a real version, we'd loop through arguments here
                        write!(&mut self.output, ")")?;
                    }
                }
            },
            _ => write!(&mut self.output, "/* unhandled expression */")?,
        }
        Ok(())
    }

    fn op_to_c(op: &Token) -> &str {
        match op {
            Token::Plus => "+", Token::Minus => "-", Token::Asterisk => "*",
            Token::Slash => "/", Token::Eq => "==", Token::NotEq => "!=",
            Token::Lt => "<", Token::Gt => ">",
            _ => "/* unhandled op */"
        }
    }
}
