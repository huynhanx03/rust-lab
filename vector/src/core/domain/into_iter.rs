#![allow(dead_code, unused_imports, dead_code)]

use core::{iter::FusedIterator, ptr};

use crate::shared::allocator::{deallocate_block, drop_contents};

pub struct IntoIter<T> {
    buf: *mut T,
    ptr: *const T,
    end: *const T,
    capacity: usize,
}

impl<T> IntoIter<T> {
    pub fn new(ptr: *mut T, len: usize, capacity: usize) -> Self {
        let end = unsafe {
            if core::mem::size_of::<T>() == 0 {
                (ptr as usize + len) as *mut T
            } else {
                ptr.add(len)
            }
        };

        Self {
            buf: ptr,
            ptr,
            end,
            capacity,
        }
    }

    pub fn len(&self) -> usize {
        if core::mem::size_of::<T>() == 0 {
            self.end as usize - self.ptr as usize
        } else {
            (self.end as usize - self.ptr as usize) / core::mem::size_of::<T>()
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            return None;
        }

        unsafe {
            if core::mem::size_of::<T>() == 0 {
                self.ptr = (self.ptr as usize + 1) as *const T;
                return Some(core::mem::zeroed());
            }

            let item = ptr::read(self.ptr);
            self.ptr = self.ptr.add(1);
            Some(item)
        }
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            return None;
        }

        unsafe {
            if core::mem::size_of::<T>() == 0 {
                self.end = (self.end as usize - 1) as *const T;
                return Some(core::mem::zeroed());
            }

            let item = ptr::read(self.end.sub(1));
            self.end = self.end.sub(1);
            Some(item)
        }
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        Self::len(self)
    }
}

impl<T> FusedIterator for IntoIter<T> {}

unsafe impl<T: Send> Send for IntoIter<T> {}

unsafe impl<T: Sync> Sync for IntoIter<T> {}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        drop_contents(self.ptr as *mut T, Self::len(self));

        if self.capacity > 0 && core::mem::size_of::<T>() > 0 {
            deallocate_block(self.buf, self.capacity);
        }
    }
}