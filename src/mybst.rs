use std::cmp::Ordering;
use std::error::Error;

struct BSTNode<T>
where
    T: Ord,
{
    data: T,
    left: Option<Box<BSTNode<T>>>,
    right: Option<Box<BSTNode<T>>>,
}

impl<T> BSTNode<T>
where
    T: Ord,
{
    fn new(data: T) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }

    fn get_ref(&self) -> &T {
        &self.data
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.data
    }

    fn take(self) -> T {
        self.data
    }
}

pub struct MyBST<T>
where
    T: Ord,
{
    root: Option<BSTNode<T>>,
}

impl<T> MyBST<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    fn _search_node(node: &BSTNode<T>, target: &T) -> bool {
        match target.cmp(&node.data) {
            Ordering::Equal => {
                true
            },
            Ordering::Greater => {
                match &node.right {
                    Some(right) => {
                        Self::_search_node(right, target)
                    },
                    None => false,
                }
            },
            Ordering::Less => {
                match &node.left {
                    Some(left) => {
                        Self::_search_node(left, target)
                    },
                    None => false
                }
            }
        }
    }

    pub fn search(&self, target: &T) -> bool {
        match &self.root {
            Some(node) => Self::_search_node(node, target),
            None => return false,
        }
    }
}

impl<T> Default for MyBST<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self { root: None }
    }
}
