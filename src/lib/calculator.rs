use binary_tree::count::CountTree;
use crate::syntax::{Element, Value};

pub struct Calculator {
    tree: CountTree<Element>
}

impl Calculator {
    pub fn from(args: &[String]) -> Self {
        println!("{:?}", args);

        todo!()
    }

    pub fn calculate(&self) -> Value {

        todo!()
    }
}
