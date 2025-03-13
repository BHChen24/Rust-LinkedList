// Author: me
// Created: Feb 23, 2025
// Description: Linked list

use std::{error::Error, fmt::Debug, marker::PhantomData, mem, ptr::NonNull};

use crate::project_errors::OutOfIndexError;

struct Node<T> {
    data: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            data: value,
            next: None,
            prev: None,
        }
    }

    fn into_value(self: Box<Self>) -> T {
        self.data
    }
}

pub struct MyLinkedList<T> {
    size: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    _marker: PhantomData<Box<T>>,
}

impl<T> MyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
            tail: None,
            _marker: PhantomData,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.head == None
    }

    pub fn push_front(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        new_node.next = self.head;
        let ready_node = NonNull::new(Box::into_raw(new_node));

        match self.head {
            None => self.tail = ready_node,
            Some(head) => unsafe {
                (*head.as_ptr()).prev = ready_node;
            },
        }

        self.head = ready_node;
        self.size += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        new_node.prev = self.tail;
        let ready_node = NonNull::new(Box::into_raw(new_node));

        match self.tail {
            None => self.head = ready_node,
            Some(tail) => unsafe {
                (*tail.as_ptr()).next = ready_node;
            },
        }

        self.tail = ready_node;
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|content| {
            self.size -= 1;
            unsafe {
                let transfer_back_node = Box::from_raw(content.as_ptr());
                self.head = transfer_back_node.next;
                match self.head {
                    None => self.tail = None,
                    Some(head) => (*head.as_ptr()).prev = None,
                }
                transfer_back_node.into_value()
            }
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|content| unsafe {
            let transfer_back_node = Box::from_raw(content.as_ptr());
            self.tail = transfer_back_node.prev;

            match self.tail {
                None => {
                    self.head = None;
                }
                Some(tail) => {
                    (*tail.as_ptr()).next = None;
                }
            }

            self.size -= 1;
            transfer_back_node.into_value()
        })
    }

    pub fn peek_front(&self) -> Option<&T> {
        unsafe {
            // the order or the expression is &( (node.as_ref()) .data)
            // -> get the value first the take the ref
            self.head.as_ref().map(|node| &node.as_ref().data)
        }
    }

    pub fn peek_back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &node.as_ref().data) }
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.as_mut().data) }
    }

    pub fn peek_back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|node| &mut node.as_mut().data) }
    }

    pub fn get_by_index(&self, idx: usize) -> Result<Option<&T>, Box<dyn Error>> {
        let len = self.size;
        if idx >= len {
            return Err(Box::new(OutOfIndexError {}));
        }

        let offset_from_end = len - idx - 1;
        let mut cur;
        if idx <= offset_from_end {
            cur = self.head;
            for _ in 0..idx {
                match cur.take() {
                    None => {
                        cur = self.head;
                    }
                    Some(content) => unsafe {
                        cur = content.as_ref().next;
                    },
                }
            }
        } else {
            cur = self.tail;
            for _ in 0..idx {
                match cur.take() {
                    None => {
                        cur = self.tail;
                    }
                    Some(content) => unsafe {
                        cur = content.as_ref().prev;
                    },
                }
            }
        }
        unsafe { Ok(cur.as_ref().map(|content| &content.as_ref().data)) }
    }
    
    fn _get_by_index_mut(&self, idx: usize) -> Result<Option<NonNull<Node<T>>>, Box<dyn Error>> {
        let len = self.size;
        if idx > len {
            return Err(Box::new(OutOfIndexError {}));
        }

        let offset_from_end = len - idx - 1;
        let offset_from_start = idx;
        let mut cur;

        if offset_from_start <= offset_from_end {
            cur = self.head;
            for _ in 0..idx {
                match cur.take() {
                    None => { cur = self.head },
                    Some(node) => { 
                        unsafe {
                            cur = node.as_ref().next;
                        }
                    }
                }
            }
        } else {
            cur = self.tail;
            for _ in 0..idx {
                match cur.take() {
                    None => { cur = self.tail },
                    Some(node) => {
                        unsafe {
                            cur = node.as_ref().prev;
                        }
                    }
                }
            }
        }

        Ok(cur)
        
    }

    pub fn get_by_index_mut(&self, idx: usize) -> Result<Option<&mut T>, Box<dyn Error>>{
        let mut cur = self._get_by_index_mut(idx)?;
        unsafe {
            Ok(cur.as_mut().map(|node| &mut node.as_mut().data))
        }
    }

    pub fn insert_by_index(&mut self, idx: usize, data: T) -> Result<(), Box<dyn Error>> {
        let len = self.size;

        if idx >= len {
            return Err(Box::new(OutOfIndexError {}));
        }

        if idx == 0 {
            return Ok(self.push_front(data) )
        } else if idx == len {
            return Ok(self.push_back(data))
        }

        unsafe {
            let mut new_node = Box::new(Node::new(data));
            let helper_before = self._get_by_index_mut(idx -1)?;
            let helper_after = helper_before.unwrap().as_mut().next;
            new_node.next = helper_after;
            new_node.prev = helper_before;
            let new_node = NonNull::new(Box::into_raw(new_node));
            helper_before.unwrap().as_mut().next = new_node;
            helper_after.unwrap().as_mut().prev = new_node;
        }

        self.size += 1;

        Ok(())
    }

    pub fn remove_by_idx(&mut self, idx: usize) -> Result<T, Box<dyn Error>> {
        let len = self.size;
        if idx >= len {
            return Err(Box::new(OutOfIndexError {}));
        }

        if idx == 0 {
            return Ok(self.pop_front().unwrap());
        } else if idx == len {
            return Ok(self.pop_back().unwrap());
        }

        let cur = self._get_by_index_mut(idx)?.unwrap();
        self.unlink_node(cur);

        unsafe {
            let unlinked_node = Box::from_raw(cur.as_ptr());
            Ok(unlinked_node.data)
        }
    }

    #[inline]
    fn unlink_node(&mut self, mut node: NonNull<Node<T>>) {
        let node = unsafe {
            node.as_mut()
        };
        match node.prev {
            Some(prev) => unsafe {
                (*prev.as_ptr()).next = node.next
            },
            None => {
                self.head = node.next
            },
        }

        match node.next {
            Some(next) => unsafe {
                (*next.as_ptr()).prev = node.prev
            },
            None => {
                self.tail = node.prev
            }
        }

        self.size -= 1;
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter{
            list: self
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            size: self.size,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            size: self.size,
            _marker: PhantomData,
        }
    }

    pub fn contains(&self, elem: &T) -> bool 
    where
    T: PartialEq<T>,
    {
        self.iter().any(|x| x == elem)
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

impl<T> Default for MyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> MyLinkedList<T> {
    pub fn traverse(&self) {
        print!("{{ ");
        for (idx, x) in self.iter().enumerate() {
            print!(" [{}: {:?}]", idx, x);
        }
        print!(" }}");
    }
}

impl<T> Drop for MyLinkedList<T> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut MyLinkedList<T>);

        impl<'a, T> Drop for DropGuard<'a, T> {
            fn drop(&mut self) {
                while self.0.pop_front().is_some() {}
            }
        }

        while let Some(node) = self.pop_front() {
            let guard = DropGuard(self);
            drop(node);
            mem::forget(guard);
        }

        println!("LinkedList dropped.")
    } 
}

pub struct IntoIter<T> {
    list: MyLinkedList<T>,
}

pub struct Iter<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    size: usize,
    _marker: PhantomData<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    size: usize,
    _marker: PhantomData<&'a mut Node<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.size(), Some(self.list.size()))
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item =&'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            None
        } else {
            self.head.map(|x| {
                self.size -= 1;
                unsafe {
                    let node =  &*x.as_ptr();
                    self.head = node.next;
                    &node.data
                }
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }

    #[inline]
    fn last(mut self) -> Option<&'a T> {
        self.next_back()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            None
        } else {
            self.tail.map(| node | {
                self.size -= 1;
                unsafe {
                    let node = &*node.as_ptr();
                    self.tail = node.prev;
                    &node.data
                }
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item =&'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            None
        } else {
            self.head.map(|x| {
                self.size -= 1;
                unsafe {
                    let node =  &mut *x.as_ptr();
                    self.head = node.next;
                    &mut node.data
                }
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }

    #[inline]
    fn last(mut self) -> Option<&'a mut T> {
        self.next_back()
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            None
        } else {
            self.tail.map(| node | {
                self.size -= 1;
                unsafe {
                    let node = &mut *node.as_ptr();
                    self.tail = node.prev;
                    &mut node.data
                }
            })
        }
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]

    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}




#[cfg(test)]
mod test {

    #[test]
    fn this_is_a_test() {}
}
