use crate::expression::Expression;
use crate::token::Values;

pub struct Calculator {}

impl Calculator {
    pub fn calculate(expression: Expression) -> Values {
        expression.evaluate()
    }
}
