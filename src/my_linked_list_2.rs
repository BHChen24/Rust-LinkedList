use crate::project_errors::{EmptyList, NotValidIndexError };
use std::error::Error;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::mem;

#[derive(PartialEq, Eq)]
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

    #[allow(dead_code)]
    fn into_val(self) -> Option<T> {
        self.data
    }

    fn get_ref(&self) -> Option<&T> {
        self.data.as_ref()
    }

    fn get_mut(&mut self) -> Option<&mut T> {
        self.data.as_mut()
    }

    #[allow(dead_code)]
    fn take(&mut self) -> Option<T> {
        self.data.take()
    }

    fn set(&mut self, data: T) {
        self.data = Some(data);
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data
            .as_ref()
            .expect("The data in current node is None.")
    }
}

impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.as_mut().expect("The data in current node is None.")
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

        unsafe {
            head_sentinel.unwrap().as_mut().next = tail_sentinel;
            tail_sentinel.unwrap().as_mut().prev = head_sentinel;
        }

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

    fn _get_front(&self) -> Option<NonNull<Node<T>>> {
        unsafe { self.head.unwrap().as_ref().next }
    }

    fn _get_back(&self) -> Option<NonNull<Node<T>>> {
        unsafe { self.tail.unwrap().as_ref().prev }
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
            let front = self._get_front();
            let helper = front.unwrap().as_ref().next;

            let front_box = Box::from_raw(front.unwrap().as_ptr());
            let return_data = front_box.data;

            self.head.unwrap().as_mut().next = helper;
            helper.unwrap().as_mut().prev = self.head;

            self.size -= 1;
            Ok(return_data.expect("Data unwrapped with None"))
        }
    }

    pub fn pop_back(&mut self) -> Result<T, Box<dyn Error>> {
        if self.is_empty() {
            return Err(Box::new(EmptyList {}));
        }

        unsafe {
            let back = self._get_back();
            let helper = back.unwrap().as_ref().prev;

            let back_box = Box::from_raw(back.unwrap().as_ptr());
            let return_data = back_box.data;

            self.tail.unwrap().as_mut().prev = helper;
            helper.unwrap().as_mut().next = self.tail;

            self.size -= 1;
            Ok(return_data.expect("Data unwrapped with None"))
        }
    }

    pub fn push_front(&mut self, data: T) {
        let mut new_node = Box::new(Node::new_with(data));

        let front = self._get_front();
        new_node.next = front;
        new_node.prev = self.head;

        unsafe {
            let new_node_ptr = NonNull::new(Box::into_raw(new_node));
            front.unwrap().as_mut().prev = new_node_ptr;
            self.head.unwrap().as_mut().next = new_node_ptr;

            self.size += 1;
        }
    }

    pub fn push_back(&mut self, data: T) {
        let mut new_node = Box::new(Node::new_with(data));

        let back = self._get_back();
        new_node.prev = back;
        new_node.next = self.tail;

        unsafe {
            let new_node_ptr = NonNull::new(Box::into_raw(new_node));
            back.unwrap().as_mut().prev = new_node_ptr;
            self.tail.unwrap().as_mut().next = new_node_ptr;

            self.size += 1;
        }
    }


    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        let front = self._get_front();
        unsafe {
            let node_ref = front.unwrap().as_ref();
            node_ref.data.as_ref()
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() { return None; }
        let front = self._get_front();
        unsafe {
            let node_mut = front.unwrap().as_mut();
            node_mut.data.as_mut()
        }
    }

    pub fn peek_back(&self) -> Option<&T> {
        if self.is_empty() {return None ;}
        let back = self._get_back();
        unsafe {
            let node_ref = back.unwrap().as_ref();
            node_ref.data.as_ref()
        }
    }

    pub fn peek_back_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() { return None; }
        let back = self._get_back();
        unsafe {
            let node_mut = back.unwrap().as_mut();
            node_mut.data.as_mut()
        }
    }

    fn _get_index_cur(&self, index: usize) -> Option<NonNull<Node<T>>> {
        let _ = self.check_element_index(index);

        unsafe {
            let mut cur = self._get_front();
            for _ in 0..index {
                cur = cur.unwrap().as_mut().next;
            }
            cur
        }
    }

    pub fn get(&self, index: usize) -> Result<Option<&T>, Box<dyn Error>> {
        if self.is_empty() {
            return Err(Box::new(EmptyList {}));
        }

        self.check_element_index(index)?;

        let data_option = unsafe { self._get_index_cur(index).unwrap().as_ref().get_ref() };
        Ok(data_option)
    }

    pub fn get_mut(&mut self, index: usize) -> Result<Option<&mut T>, Box<dyn Error>> {
        if self.is_empty() {
            return Err(Box::new(EmptyList {}));
        }

        self.check_element_index(index)?;

        let data_option = unsafe { self._get_index_cur(index).unwrap().as_mut().get_mut() };

        Ok(data_option)

    }

    pub fn set(&mut self, index: usize, data: T) -> Result<(), Box<dyn Error>> {
        if self.is_empty() { return Err(Box::new(EmptyList {})) }

        self.check_element_index(index)?;

        let cur = self._get_index_cur(index);
        unsafe {
            cur.unwrap().as_mut().set(data);
            Ok(())
        }
    }

    pub fn remove(&mut self, index: usize) -> Result<Option<T>, Box<dyn Error>> {
        if self.is_empty() { return Err(Box::new(EmptyList {})); }
        self.check_element_index(index)?;

        let cur = self._get_index_cur(index);
        unsafe {
            let cur_next = cur.unwrap().as_mut().next;
            let cur_prev = cur.unwrap().as_mut().prev;

            cur_prev.unwrap().as_mut().next = cur_next;
            cur_next.unwrap().as_mut().prev = cur_prev;
            self.size -= 1;

            let cur_box = Box::from_raw(cur.unwrap().as_ptr());
            Ok(cur_box.data)
        }
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }

    fn iter(&self) -> Iter<'_, T> {
        Iter {
            front: self._get_front(),
            back: self._get_back(),
            size: self.size,
            _marker: PhantomData,
        }
    }

    fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            front: self._get_front(),
            back: self._get_back(),
            size: self.size,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for MyLinkedList2<T> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut MyLinkedList2<T>);

        impl<'a, T> Drop for DropGuard<'a, T> {
            fn drop(&mut self) {
                while self.0.pop_front().ok().is_some() {}
            }
        }

        while let Some(node) = self.pop_front().ok() {
            let guard = DropGuard(self);
            drop(node);
            mem::forget(guard);
        }
    }
}

struct IntoIter<T> {
    list: MyLinkedList2<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front().ok()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.size, Some(self.list.size))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back().ok()
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

struct Iter<'a, T> {
    size: usize,
    front: Option<NonNull<Node<T>>>,
    back: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        unsafe {
            let data_ref = &*self.front.unwrap().as_ref();
            self.front = self.front.unwrap().as_ref().next;
            self.size -= 1;
            Some(data_ref)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back == self.front {
            return None;
        }
        unsafe {
            let data_ref = &*self.back.unwrap().as_ref();
            self.back = self.back.unwrap().as_ref().prev;
            self.size -= 1;
            Some(data_ref)
        }
    }
}

struct IterMut<'a, T> {
    size: usize,
    front: Option<NonNull<Node<T>>>,
    back: Option<NonNull<Node<T>>>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        unsafe {
            let data_ref = &mut *self.front.unwrap().as_mut();
            self.front = self.front.unwrap().as_ref().next;
            self.size -= 1;
            Some(data_ref)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back == self.front {
            return None;
        }
        unsafe {
            let data_ref = &mut *self.back.unwrap().as_mut();
            self.back = self.back.unwrap().as_ref().prev;
            self.size -= 1;
            Some(data_ref)
        }
    }
}

fn _is_available_index(idx: usize, size: usize) -> bool {
    idx <= size
}

fn _is_element_index(idx: usize, size: usize) -> bool {
    idx < size
}

#[cfg(test)]
mod test {
    use super::MyLinkedList2;

    #[test]
    fn test_compile() {}

    #[test]
    fn initialize_list() {
        let my_list: MyLinkedList2<i32> = MyLinkedList2::new();
    }

    #[test]
    fn test_string_list() {
        let mut string_list: MyLinkedList2<String> = MyLinkedList2::new();
        string_list.push_front(String::from("a"));
        string_list.push_front(String::from("b"));
        string_list.push_front(String::from("c"));
        for i in string_list.into_iter() {
            println!("{:?}", i)
        }
    }
}