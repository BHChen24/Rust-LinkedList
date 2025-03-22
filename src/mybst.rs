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
        Self {
            root: None,
        }
    }

    pub fn search(&self, target: &T) -> bool {
        match &self.root {
            Some(node) => {
                match target.cmp(&node.data) {
                    Ordering::Equal => {
                        true
                    },
                    Ordering::Greater => {
                        match &node.left {
                            Some(left_node) => {
                                
                            },
                            None => {false},
                        }
                    },
                    Ordering::Less => {}
                }
            }
            None => return false
        }
    }
}

impl<T> Default for MyBST<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self {
            root: None,
        }
    }
}