use crate::token::BinaryOperator::{DivideBy, Minus, Plus, Times};
use crate::token::Token::{Operator, Val};
use crate::token::Value::{Float, Integer, Rational};
use num_rational::Rational64;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Token {
    Operator(BinaryOperator),
    Val(Value),
}

impl Token {
    pub fn from_str(value: &str) -> Result<Self, String> {
        match value {
            "+" => Ok(Operator(Plus)),
            "-" => Ok(Operator(Minus)),
            "*" => Ok(Operator(Times)),
            "/" => Ok(Operator(DivideBy)),
            x if i64::from_str(x).is_ok() => Ok(Val(Integer(i64::from_str(x).unwrap()))),
            x if f64::from_str(x).is_ok() => Ok(Val(Float(f64::from_str(x).unwrap()))),
            _ => Err(
                "Failed to parse element as operator, integer or floating point number".to_string(),
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Times,
    DivideBy,
}

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i64),
    Rational(Rational64),
    Float(f64),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator(op) => op.fmt(f),
            Val(val) => val.fmt(f),
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Plus => write!(f, "{}", "+"),
            Minus => write!(f, "{}", "-"),
            Times => write!(f, "{}", "*"),
            DivideBy => write!(f, "{}", "/"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer(num) => {
                write!(f, "{}", num)
            }
            Rational(num) => {
                write!(f, "{}", num)
            }
            Float(num) => {
                write!(f, "{}", num)
            }
        }
    }
}
