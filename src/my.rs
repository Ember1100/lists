#[derive(Debug,PartialEq, Eq)]

pub struct TreeNode<T> {
    val: T,
    next: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        TreeNode { val, next: None }
    }

    pub fn link(&mut self, node: Box<TreeNode<T>>) {
        self.next = Some(node)
    }
}
