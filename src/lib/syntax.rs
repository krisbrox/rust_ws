use num_rational::Rational64;

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
