use crate::expression::Expression;
use crate::token::Value;

pub struct Calculator {}

impl Calculator {
    pub fn calculate(expression: Expression) -> Value {
        expression.evaluate()
    }
}
