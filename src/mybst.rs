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
            data: data,
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
    fn new() -> Self {
        Self {
            root: None,
        }
    }
}
