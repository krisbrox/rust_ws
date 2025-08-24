use crate::example::token::Token;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct SyntaxTree<T = Token>
where
    T: 'static,
    Node<T>: 'static,
{
    node_type: PhantomData<T>,

    pub root: Option<usize>,
    pub nodes: Vec<Node<T>>,
}

impl<T> SyntaxTree<T> {
    pub fn new() -> SyntaxTree<T> {
        SyntaxTree {
            node_type: PhantomData,
            root: None,
            nodes: vec![],
        }
    }

    pub fn set_root(&mut self, val: T) {
        let node = Node::new(val);
        self.nodes.push(node);
        self.root = Some(0);
    }

    pub fn insert(&mut self, val: T, parent: usize, left: bool) -> usize {
        let idx = self.nodes.len();
        let mut node = Node::new(val);
        node.parent = Some(parent);
        let parent = &mut self.nodes[parent];
        if left {
            parent.left = Some(idx);
        } else {
            parent.right = Some(idx);
        }

        self.nodes.push(node);
        idx
    }

    pub fn last_idx(&self) -> usize {
        self.nodes.len() - 1
    }

    pub fn get(&self, idx: usize) -> Option<&Node<T>> {
        self.nodes.get(idx)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

#[derive(Debug, Clone)]
pub struct Node<T = Token>
where
    T: 'static,
{
    data: PhantomData<T>,

    pub value: T,
    pub parent: Option<usize>,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Node {
            data: Default::default(),
            value: val,
            parent: None,
            left: None,
            right: None,
        }
    }
}
