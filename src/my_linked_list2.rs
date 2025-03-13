use std::error::Error;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;
use crate::project_errors::{NotValidIndexError, OutOfIndexError, EmptyList};

struct Node<T> {
    data: Option<T>,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new_with(data: T) -> Self {
        Self {
            data: Some(data),
            next: None,
            prev: None,
        }
    }

    fn new() -> Self {
        Self {
            data: None,
            next: None,
            prev: None,
        }
    }

    fn into_val(self) -> Option<T> {
        self.data
    }

    fn get_ref(&self) -> Option<&T> {
        self.data.as_ref()
    }

    fn take(&mut self) -> Option<T> {
        self.data.take()
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data.as_ref().expect("The data in current node is None.")
    }
}

pub struct MyLinkedList2<T> {
    size: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    _marker: PhantomData<T>,
}

impl<T> MyLinkedList2<T> {
    pub fn new() -> Self {
        let head_sentinel = NonNull::new(Box::into_raw(Box::new(Node::new())));
        let tail_sentinel = NonNull::new(Box::into_raw(Box::new(Node::new())));

        Self {
            size: 0,
            head: head_sentinel,
            tail: tail_sentinel,
            _marker: PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    fn check_available_index(&self, idx: usize) -> Result<(), Box<dyn Error>> {
        if !_is_available_index(idx, self.size) {
            return Err(Box::new(NotValidIndexError {}));
        }
        Ok(())
    }

    fn check_element_index(&self, idx: usize) -> Result<(), Box<dyn Error>> {
        if !_is_element_index(idx, self.size) {
            return Err(Box::new(NotValidIndexError {}));
        }
        Ok(())
    }

    pub fn pop_front(&mut self) -> Result<T, Box<dyn Error>> {
        if self.is_empty() {
            return Err(Box::new(EmptyList {}));
        }

        unsafe {
            let mut front = self.head.unwrap().as_ref().next;
            let mut helper = front.unwrap().as_ref().next;

            let front_box = Box::from_raw(front.unwrap().as_ptr());
            let return_data = front_box.data;

            self.head.unwrap().as_mut().next = helper;
            helper.unwrap().as_mut().prev = self.head;

            self.size -= 1;
            Ok(return_data)
        }
    }

    pub fn pop_back(&mut self) -> Result<T, Box<dyn Error>> {
        if self.is_empty() {
            return Err(Box::new(EmptyList {}));
        }

        unsafe {
            let mut back = self.tail.unwrap().as_ref().next;
            let mut helper = back.unwrap().as_ref().next;

            let back_box = Box::from_raw(back.unwrap().as_ptr());
            let return_data = back_box.data;

            self.tail.unwrap().as_mut().prev = helper;
            helper.unwrap().as_mut().next = self.tail;

            self.size -= 1;
            Ok(return_data)
        }
    }
}

struct IntoIter<T> {
    list: MyLinkedList2<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}

struct Iter<'a, T> {
    size: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a T>,
}

struct IterMut<'a, T> {
    size: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a mut T>,
}

fn _is_available_index(idx: usize, size: usize) -> bool {
    idx >= 0 && idx <= size
}

fn _is_element_index(idx: usize, size: usize) -> bool {
    idx >= 0 && idx < size
}