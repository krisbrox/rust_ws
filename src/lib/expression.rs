use crate::syntax::{Node, SyntaxTree};
use crate::token::{Token, Values};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
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
            Ok((expression, _)) => {
                println!("{}", expression);
                Ok(expression)
            }

            Err(msg) => Err(msg),
        }
    }

    pub fn evaluate(self) -> Values {
        let tree = self.syntax_tree;
        let root = tree.get(tree.root.unwrap()).unwrap();

        Self::evaluate_inner(root, &tree).to_owned()
    }

    fn evaluate_inner(node: &Node, tree: &SyntaxTree<Token>) -> Values {
        match node.value {
            Token::Val(value) => value,
            Token::Operator(op) => {
                // TODO handle Unary operators
                let left_node_idx = node.left.expect(
                    format!("invalid syntax tree: missing left child for {}", node.value).as_str(),
                );
                let left_node = tree.get(left_node_idx).expect(
                    format!(
                        "invalid syntax tree: missing left child node for {} at idx {}",
                        node.value, left_node_idx
                    )
                    .as_str(),
                );

                let right_node_idx = node.right.expect(
                    format!(
                        "invalid syntax tree: missing right child for {}",
                        node.value
                    )
                    .as_str(),
                );
                let right_node = tree.get(right_node_idx).expect(
                    format!(
                        "invalid syntax tree: missing right child node for {} at idx {}",
                        node.value, right_node_idx
                    )
                    .as_str(),
                );
                let left_val = Self::evaluate_inner(left_node, tree);
                let right_val = Self::evaluate_inner(right_node, tree);
                op.apply(left_val, right_val)
            }
        }
    }

    fn apply(self, rest: Vec<&str>) -> Result<(Self, Vec<&str>), String> {
        let next = rest.first();

        match next {
            None => Ok((self, vec![])),
            Some(next) => {
                let mut tree = self.syntax_tree;
                let token =
                    Token::from_str(*next).expect(format!("Failed to parse {}", *next).as_str());

                if tree.is_empty() {
                    match token {
                        Token::Operator(_) => tree.set_root(token),
                        Token::Val(_) => {
                            return Err("Syntax tree root must be an operator".to_string());
                        }
                    }
                } else {
                    let previous_idx = tree.last_idx();
                    match Self::insert(&mut tree, &token, previous_idx) {
                        None => {
                            return Err(format!("Failed to insert {} into {:?}", token, tree));
                        }
                        Some(_) => {}
                    }
                }
                let new = Expression { syntax_tree: tree };
                let rest = Vec::from(&rest[1..]);

                new.apply(rest)
            }
        }
    }

    fn insert(tree: &mut SyntaxTree<Token>, token: &Token, at: usize) -> Option<usize> {
        let mut idx = None;
        let mut at_opt = Some(at);

        while idx.is_none() && at_opt.is_some() {
            let at = at_opt.unwrap();
            idx = Self::insert_at(tree, &token, at);
            if idx.is_none() {
                let node = tree.get(at).unwrap();
                at_opt = node.parent;
            }
        }

        idx
    }

    fn insert_at(tree: &mut SyntaxTree<Token>, token: &Token, at: usize) -> Option<usize> {
        let at_node = tree.get(at).unwrap();

        match &at_node.value {
            Token::Operator(_) => {
                if at_node.left.is_none() {
                    Some(tree.insert(token.clone(), at, true))
                } else if at_node.right.is_none() {
                    Some(tree.insert(token.clone(), at, false))
                } else if let Some(right) = at_node.right {
                    match tree.get(right).unwrap().value {
                        Token::Operator(_) => Self::insert_at(tree, token, right),
                        Token::Val(_) => None,
                    }
                } else {
                    None
                }
            }
            Token::Val(_) => None,
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let node_strs: Vec<String> = self
            .syntax_tree
            .nodes
            .iter()
            .enumerate()
            .map(|(idx, node)| format!("id: {:>3}, {}", idx, node))
            .collect();
        write!(f, "\nnodes: \n{}", node_strs.join("\n"))
    }
}

impl Display for Node<Token> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "value: {:>3}, parent: {:>3}, left: {:>3}, right: {:>3}",
            self.value.to_string(),
            self.parent
                .map(|p| p.to_string())
                .unwrap_or("  ".to_string()),
            self.left.map(|p| p.to_string()).unwrap_or("  ".to_string()),
            self.right
                .map(|p| p.to_string())
                .unwrap_or("  ".to_string()),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::expression::Expression;

    #[test]
    fn simple() {
        let res = Expression::from("+ 2 3");
        match res {
            Ok(_) => {}
            Err(_) => {}
        }
        assert!(res.is_ok())
    }

    #[test]
    fn more_significant() {
        let exprs = ["+ 2 + 1 1"];

        for expr in exprs {
            let res = Expression::from(expr);

            match res {
                Ok(_) => {}
                Err(_) => {}
            }
            assert!(res.is_ok())
        }
    }

    #[test]
    fn more_significant_2() {
        let exprs = ["- * / 15 - 7 + 1 1 3 + 2 + 1 1"];

        for expr in exprs {
            let res = Expression::from(expr);

            match res {
                Ok(_) => {}
                Err(_) => {}
            }
            assert!(res.is_ok())
        }
    }
}
