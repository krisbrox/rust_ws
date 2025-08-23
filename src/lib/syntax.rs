use std::fmt::{Display, Formatter};
use num_rational::Rational64;
use crate::syntax::Value::Integer;

pub enum Element {
    Binary(BinaryOperator),
    Unary(UnaryOperator),
    Val(Value)
}

pub enum BinaryOperator {
    Plus,
    Times,
    DivideBy,
}

pub enum UnaryOperator {
    Minus,
}

pub enum Value {
    Integer(i64),
    Natural(usize),
    Rational(Rational64),
    Float(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer(num) => {
                write!(f, "{}", num)
            }
            Value::Natural(num) => {
                write!(f, "{}", num)
            }
            Value::Rational(num) => {
                write!(f, "{}", num)
            }
            Value::Float(num) => {
                write!(f, "{}", num)
            }
        }
        
    }
}