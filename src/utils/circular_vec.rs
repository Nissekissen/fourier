#![allow(unused)]

use std::ptr;

pub struct CircularVec<T> {
    ptr: *mut T,
    capacity: usize,
    head: usize,
    len: usize,
}

impl<T> CircularVec<T> {
    pub fn new(capacity: usize) -> Self {
        let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) as *mut T };

        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }

        Self {
            ptr,
            capacity,
            head: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len < self.capacity {
                ptr::write(self.ptr.add(self.head), item);
                self.len += 1;
            } else {
                // Drop the overwritten item
                ptr::drop_in_place(self.ptr.add(self.head));
                ptr::write(self.ptr.add(self.head), item);
            }
        }
        self.head = (self.head + 1) % self.capacity;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            let real_index = (self.head + index) % self.capacity;
            unsafe { Some(&*self.ptr.add(real_index)) }
        } else {
            None
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

    pub fn iter(&self) -> CircularVecIter<T> {
        CircularVecIter {
            cv: &self,
            index: 0,
        }
    }
}

impl<'a, T> Drop for CircularVec<T> {
    fn drop(&mut self) {
        let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
        unsafe {
            // Drop the allocated items before deallocating
            for i in 0..self.len {
                ptr::drop_in_place(self.ptr.add(i));
            }
            std::alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }
}

pub struct CircularVecIter<'a, T> {
    cv: &'a CircularVec<T>,
    index: usize,
}

impl<'a, T> Iterator for CircularVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let it = self.cv.get(self.index);
        self.index += 1;
        it
    }
}

#[cfg(test)]
mod tests {
    use super::CircularVec;

    #[test]
    fn test_empty() {
        let cv: CircularVec<i32> = CircularVec::new(3);
        assert_eq!(cv.capacity(), 3);
        assert_eq!(cv.len(), 0);
        assert_eq!(cv.is_empty(), true);
        assert_eq!(cv.get(0), None);
    }

    #[test]
    fn test_push_and_get() {
        let mut cv: CircularVec<i32> = CircularVec::new(3);
        cv.push(0);
        cv.push(1);
        cv.push(2);
        assert_eq!(cv.get(0), Some(&0));
        assert_eq!(cv.get(1), Some(&1));
        assert_eq!(cv.get(2), Some(&2));
        assert_eq!(cv.len(), 3);
        assert_eq!(cv.is_full(), true);

        cv.push(3);
        assert_eq!(cv.get(0), Some(&1));
        assert_eq!(cv.get(1), Some(&2));
        assert_eq!(cv.get(2), Some(&3));
        assert_eq!(cv.len(), 3);

        cv.push(4);
        assert_eq!(cv.get(0), Some(&2));
        assert_eq!(cv.get(1), Some(&3));
        assert_eq!(cv.get(2), Some(&4));
        assert_eq!(cv.len(), 3);
    }
}
