use core::ptr;

extern crate alloc;

use alloc::alloc::{Layout, alloc, dealloc, handle_alloc_error};

pub fn allocate_block<T>(size: usize) -> *mut T {
    let layout = Layout::array::<T>(size).unwrap();
    unsafe {
        let ptr = alloc(layout) as *mut T;
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        ptr
    }
}

pub fn drop_contents<T>(ptr: *mut T, size: usize) {
    for i in 0..size {
        unsafe {
            ptr::drop_in_place(ptr.add(i));
        }
    }
}

pub fn deallocate_block<T>(ptr: *mut T, size: usize) {
    let layout = Layout::array::<T>(size).unwrap();
    unsafe {
        dealloc(ptr as *mut u8, layout);
    }
}
