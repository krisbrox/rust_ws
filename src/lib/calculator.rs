use binary_tree::count::CountTree;
use crate::syntax::{Element, Value};

pub struct Calculator {
    tree: CountTree<Element>
}

impl Calculator {
    pub fn from(arg: &str) -> Self {
        println!("calculating: {:?}", arg);

        todo!()
    }

    pub fn calculate(&self) -> Value {

        todo!()
    }
}
