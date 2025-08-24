use crate::math::{Addition, Division, Float, Int, Multiplication, Subtraction};
use crate::token::BinaryOperators::{DivideBy, Minus, Plus, Times};
use crate::token::Token::{Operator, Val};
use crate::token::Values::{FloatingPoint, Integer};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, Copy)]
pub enum Token {
    Operator(BinaryOperators),
    Val(Values),
}

impl Token {
    pub fn from_str(value: &str) -> Result<Self, String> {
        match value {
            "+" => Ok(Operator(Plus(Addition))),
            "-" => Ok(Operator(Minus(Subtraction))),
            "*" => Ok(Operator(Times(Multiplication))),
            "/" => Ok(Operator(DivideBy(Division))),

            x if i64::from_str(x).is_ok() => {
                let val = i64::from_str(x).unwrap();
                Ok(Val(Integer(Int(val))))
            }

            x if f64::from_str(x).is_ok() => {
                let val = f64::from_str(x).unwrap();
                Ok(Val(FloatingPoint(Float(val))))
            }
            _ => Err(
                "Failed to parse element as operator, integer or floating point number".to_string(),
            ),
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum BinaryOperators {
    Plus(Addition),
    Minus(Subtraction),
    Times(Multiplication),
    DivideBy(Division),
}

#[derive(Clone, Debug, Copy)]
pub enum Values {
    Integer(Int),
    FloatingPoint(Float),
    // Rational(Rational64),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator(op) => op.fmt(f),
            Val(val) => val.fmt(f),
        }
    }
}

impl Display for BinaryOperators {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Plus(_) => write!(f, "{}", "+"),
            Minus(_) => write!(f, "{}", "-"),
            Times(_) => write!(f, "{}", "*"),
            DivideBy(_) => write!(f, "{}", "/"),
        }
    }
}

impl Display for Values {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer(num) => {
                write!(f, "{}", num.0)
            }
            FloatingPoint(num) => {
                write!(f, "{}", num.0)
            } /* Rational(num) => { write!(f, "{}", num) } */
        }
    }
}
