use crate::syntax::SyntaxTree;
use crate::token::{Token, Value};

pub struct Expression {
    syntax_tree: SyntaxTree<Token>,
}

impl Expression {
    pub fn from(expr_str: &str) -> Result<Self, String> {
        let expr = Expression {
            syntax_tree: SyntaxTree::new(),
        };

        let args: Vec<&str> = expr_str.split(" ").filter(|e| !e.is_empty()).collect();
        let res = expr.apply(args);

        match res {
            Ok((expression, _)) => Ok(expression),

            Err(msg) => Err(msg),
        }
    }

    pub fn evaluate(&self) -> Value {
        todo!()
    }

    fn apply(self, rest: Vec<&str>) -> Result<(Self, Vec<&str>), String> {
        let next = rest.first();

        let tree = match next {
            None => {
                return Ok((self, vec![]));
            }
            Some(next) => {
                let mut elems = self.syntax_tree;
                let elem =
                    Token::from_str(*next).expect(format!("Failed to parse {}", *next).as_str());

                if elems.is_empty() {
                    match elem {
                        Token::Operator(_) => elems.set_root(elem),
                        Token::Val(_) => {
                            return Err("Syntax tree root must be an operator".to_string());
                        }
                    }
                } else {
                    todo!()
                }

                elems
            }
        };

        let new = Expression { syntax_tree: tree };
        let rest = Vec::from(&rest[1..]);

        new.apply(rest)
    }
}
