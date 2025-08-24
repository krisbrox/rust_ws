use crate::example::token::{BinaryOperators, Values};

pub trait BinaryOperator<E> {
    fn apply(&self, left_operand: E, right_operand: E) -> E;
    fn static_apply(left_operand: E, right_operand: E) -> E;
}

impl BinaryOperator<Int> for Addition {
    fn apply(&self, left_operand: Int, right_operand: Int) -> Int {
        Self::static_apply(left_operand, right_operand)
    }

    fn static_apply(left_operand: Int, right_operand: Int) -> Int {
        let Int(left) = left_operand;
        let Int(right) = right_operand;
        Int(left + right)
    }
}

impl BinaryOperators {
    pub fn apply(&self, left: Values, right: Values) -> Values {
        match (self, left, right) {
            (BinaryOperators::Plus(add), Values::Integer(left), Values::Integer(right)) => {
                let res = add.apply(left, right);
                Values::Integer(res)
            }

            (BinaryOperators::Minus(_), _, _) => todo!(),
            (BinaryOperators::Times(_), _, _) => todo!(),
            (BinaryOperators::DivideBy(_), _, _) => todo!(),
            (BinaryOperators::Plus(Addition), Values::FloatingPoint(_), _) => todo!(),
            (BinaryOperators::Plus(Addition), Values::Integer(_), Values::FloatingPoint(_)) => {
                todo!()
            }
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Addition;

#[derive(Clone, Debug, Copy)]
pub struct Subtraction;

#[derive(Clone, Debug, Copy)]
pub struct Multiplication;

#[derive(Clone, Debug, Copy)]
pub struct Division;

#[derive(Clone, Debug, Copy)]
pub struct Int(pub i64);

#[derive(Clone, Debug, Copy)]
pub struct Float(pub f64);
