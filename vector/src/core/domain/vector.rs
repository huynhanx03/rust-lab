#![allow(dead_code, unused_imports, dead_code)]

use core::{
    mem,
    ops::{Deref, DerefMut},
    ptr::{self},
    slice::{Iter, IterMut},
};

use super::into_iter::IntoIter;
use crate::{println, shared};
use shared::allocator::{allocate_block, deallocate_block, drop_contents};

pub struct MyVector<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> MyVector<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let ptr = if capacity == 0 {
            ptr::null_mut()
        } else {
            allocate_block(capacity)
        };
        Self {
            ptr,
            len: 0,
            capacity,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.len == self.capacity {
            self.grow();
        }

        unsafe {
            ptr::write(self.ptr.add(self.len), item);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        unsafe { Some(ptr::read(self.ptr.add(self.len))) }
    }

    pub fn insert(&mut self, index: usize, item: T) {
        if index > self.len {
            panic!("Index out of bounds");
        }

        if self.len == self.capacity {
            self.grow();
        }

        unsafe {
            ptr::copy(
                self.ptr.add(index),
                self.ptr.add(index + 1),
                self.len - index,
            );

            ptr::write(self.ptr.add(index), item);
        }

        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!("Index out of bounds");
        }

        unsafe {
            let item = ptr::read(self.ptr.add(index));

            ptr::copy(
                self.ptr.add(index + 1),
                self.ptr.add(index),
                self.len - index - 1,
            );

            self.len -= 1;
            item
        }
    }

    pub fn extend(&mut self, iter: impl IntoIterator<Item = T>) {
        let iter = iter.into_iter();
        let count = iter.size_hint().0;
        self.reserve(count);

        for item in iter {
            self.push(item);
        }
    }

    fn reallocate(&mut self, new_capacity: usize) {
        let new_ptr = if new_capacity == 0 {
            ptr::null_mut()
        } else {
            allocate_block(new_capacity)
        };

        if self.capacity > 0 {
            unsafe {
                ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
            }

            deallocate_block(self.ptr, self.capacity);
        }

        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity * 2
        };

        self.reallocate(new_capacity);
    }

    pub fn reserve(&mut self, additional: usize) {
        let required_capacity = self.len + additional;
        if required_capacity > self.capacity {
            self.reallocate(required_capacity.next_power_of_two());
        }
    }

    pub fn shrink_to_fit(&mut self) {
        let new_capacity = if self.len == 0 {
            0
        } else {
            self.len.next_power_of_two()
        };

        if new_capacity < self.capacity {
            self.reallocate(new_capacity);
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }

    pub fn clear(&mut self) {
        drop_contents(self.ptr, self.len);
        self.len = 0;
    }

    fn clean_up(&mut self) {
        if self.ptr.is_null() {
            return;
        }

        drop_contents(self.ptr, self.len);
        deallocate_block(self.ptr, self.capacity);
    }
}

impl<T> IntoIterator for MyVector<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let len = self.len;
        let capacity = self.capacity;
        let ptr = self.ptr;

        mem::forget(self);
        IntoIter::new(ptr, len, capacity)
    }
}

impl<'a, T> IntoIterator for &'a MyVector<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut MyVector<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Deref for MyVector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T> DerefMut for MyVector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<T> Drop for MyVector<T> {
    fn drop(&mut self) {
        println!("Vector dropped");
        Self::clean_up(self);
    }
}
