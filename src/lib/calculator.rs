use crate::example::expression::Expression;

pub struct Calculator {}

impl Calculator {
    pub fn parse_expression(expr_str: &str) -> impl MathExpression {
        Expression::from(expr_str)
            .expect(format!("Failed to parse expression \"{}\"", expr_str).as_str())
    }
    pub fn calculate(expr_str: &str) -> String {
        let expression = Self::parse_expression(expr_str);
        expression.calculate()
    }
}

pub trait MathExpression {
    fn calculate(self) -> String;
}

impl MathExpression for Expression {
    fn calculate(self) -> String {
        self.evaluate()
    }
}
