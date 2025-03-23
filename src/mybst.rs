use std::cmp::Ordering;
#[allow(unused_imports)]
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
    root: Option<Box<BSTNode<T>>>,
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
            }
            Ordering::Greater => {
                match &node.right {
                    Some(right) => {
                        Self::_search_node(right, target)
                    }
                    None => false,
                }
            }
            Ordering::Less => {
                match &node.left {
                    Some(left) => {
                        Self::_search_node(left, target)
                    }
                    None => false
                }
            }
        }
    }

    pub fn search(&self, target: &T) -> bool {
        match &self.root {
            Some(node) => Self::_search_node(node, target),
            None => false,
        }
    }
}

pub struct BSTIntoIter<T: Ord>
{
    tree: MyBST<T>,
    stack: Vec<Box<BSTNode<T>>>,
}

pub struct BSTIter<'a, T>
where
    T: Ord,
{
    stack: Vec<&'a BSTNode<T>>,
}

pub struct BSTIterMut<'a, T>
where
    T: Ord,
{
    stack: Vec<&'a mut BSTNode<T>>,
}

impl<T: Ord> BSTIntoIter<T> {
    fn new(mut tree: MyBST<T>) -> Self {
        let mut stack = Vec::new();
        let mut current = tree.root.take();
        while let Some(mut node) = current {
            current = node.left.take();
            stack.push(node);
        }

        BSTIntoIter { tree, stack }
    }
}

impl<T: Ord> IntoIterator for MyBST<T> {
    type Item = T;
    type IntoIter = BSTIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        BSTIntoIter::new(self)
    }
}

impl<T> Iterator for BSTIntoIter<T>
where
    T: Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.stack.pop()?;
        let ret = node.data;

        let mut current = node.right.take();
        while let Some(mut n) = current {
            current = n.left.take();
            self.stack.push(n);
        }

        Some(ret)
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
