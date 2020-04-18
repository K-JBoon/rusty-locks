use crate::expr::*;

pub struct AstPrinter { }

impl AstPrinter {
    pub fn new() -> Self {
        Self { }
    }

    fn parenthesize(&mut self, name: String, exprs: Vec<&dyn ExprConstraint>) -> String {
        let mut builder = format!("({}", name);

        for expr in exprs {
            builder.push(' ');
            builder = format!("{}{}", builder, self.print(expr));
        }

        builder.push(')');

        builder
    }

    pub fn print (&mut self, expr: &dyn ExprConstraint) -> String {
        let (str_value, vec) = expr.ast_printer();
        self.parenthesize(str_value, vec) 
    }
}
