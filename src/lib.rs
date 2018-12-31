use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::mem;
use std::ptr;
use std::slice;

pub struct MemArena {
    ptr: *mut u8,
    layout: Layout,
    offset: usize,
    capacity: usize,
}

impl MemArena {
    pub fn alloc(capacity: usize) -> Self {
        let align = mem::align_of::<u8>();
        let layout = Layout::from_size_align(capacity, align).unwrap();
        let ptr = unsafe { alloc_zeroed(layout) };

        Self {
            ptr,
            layout,
            offset: 0,
            capacity,
        }
    }

    pub fn push<'a, T: 'a>(&mut self) -> &'a mut T {
        let size = mem::size_of::<T>();
        if self.offset + size > self.capacity {
            panic!("out of memory");
        }

        let result = unsafe { mem::transmute(self.ptr.add(self.offset)) };
        self.offset += size;
        result
    }

    pub fn push_array<'a, T: 'a>(&mut self, count: usize) -> &'a mut [T] {
        let size = mem::size_of::<T>();
        if self.offset + (size * count) > self.capacity {
            panic!("out of memory");
        }

        let ptr = self.ptr as *mut T;
        let result = unsafe { slice::from_raw_parts_mut(ptr.add(self.offset), count) };
        self.offset += size * count;
        result
    }

    pub fn len(&self) -> usize {
        self.offset
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn reset(&mut self) {
        self.offset = 0
    }

    pub fn clear(&mut self) {
        unsafe { ptr::write_bytes(self.ptr, 0, self.capacity) };
        self.offset = 0;
    }

    fn dealloc(&mut self) {
        unsafe {
            dealloc(self.ptr, self.layout);
        }
        self.ptr = std::ptr::null_mut();
        self.capacity = 0;
        self.offset = 0;
    }
}

impl Drop for MemArena {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            self.dealloc();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut arena = MemArena::alloc(8);
        assert_eq!(arena.len(), 0);
        assert_eq!(arena.capacity(), 8);

        let a = arena.push::<u16>();
        assert_eq!(arena.len(), 2);
        assert_eq!(a, &0u16);

        *a = 12u16;
        assert_eq!(a, &12u16);

        arena.reset();
        assert_eq!(arena.len(), 0);

        arena.dealloc();
        assert_eq!(arena.capacity(), 0);
    }

    #[test]
    fn test_reset_clear() {
        let mut arena = MemArena::alloc(8);
        assert_eq!(arena.len(), 0);
        assert_eq!(arena.capacity(), 8);

        let a = arena.push::<u32>();
        assert_eq!(arena.len(), 4);
        assert_eq!(a, &0u32);

        *a = 12u32;
        assert_eq!(a, &12u32);

        arena.reset();
        let a = arena.push::<u32>();
        assert_eq!(arena.len(), 4);
        assert_eq!(a, &12u32);

        arena.clear();
        assert_eq!(arena.len(), 0);
        assert_eq!(a, &0u32);
    }

    #[test]
    fn test_push_array() {
        let mut arena = MemArena::alloc(8);
        assert_eq!(arena.len(), 0);
        assert_eq!(arena.capacity(), 8);

        let a = arena.push_array::<u16>(3);
        assert_eq!(arena.len(), 3 * 2);
        assert_eq!(a, &[0, 0, 0]);

        a[0] = 13;
        a[1] = 56;
        a[2] = 34;

        assert_eq!(a, &[13, 56, 34]);

        arena.reset();
        assert_eq!(arena.len(), 0);
        let a = arena.push_array::<u16>(3);
        assert_eq!(arena.len(), 3 * 2);
        assert_eq!(a, &[13, 56, 34]);

        arena.clear();
        assert_eq!(arena.len(), 0);
        let a = arena.push_array::<u16>(3);
        assert_eq!(arena.len(), 3 * 2);
        assert_eq!(a, &[0, 0, 0]);
    }
}
