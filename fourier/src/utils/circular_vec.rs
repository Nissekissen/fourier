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
        let current_pos = if self.len == self.capacity {
            self.head as isize
        } else {
            0
        };
        let back_pos = if self.head == 0 {
            self.len as isize - 1
        } else {
            self.head as isize - 1
        };

        dbg!(current_pos);
        dbg!(back_pos);
        dbg!(self.head);
        dbg!(self.len);
        dbg!(self.capacity);

        CircularVecIter {
            cv: self,
            current_pos,
            back_pos,
            items_left: self.len,
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
    current_pos: isize,
    back_pos: isize,
    items_left: usize,
}

impl<'a, T> Iterator for CircularVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        dbg!(self.current_pos);
        dbg!(self.back_pos);
        if self.items_left == 0 {
            return None;
        }
        let it = self.cv.get(self.current_pos.try_into().unwrap());
        self.current_pos = (self.current_pos + 1) % self.cv.capacity as isize;
        self.items_left -= 1;
        it
    }
}

impl<'a, T> DoubleEndedIterator for CircularVecIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        dbg!(self.current_pos);
        dbg!(self.back_pos);
        if self.items_left == 0 {
            return None;
        }
        let it = self.cv.get(self.back_pos.try_into().unwrap());
        self.back_pos = if self.back_pos == 0 {
            self.cv.len as isize - 1
        } else {
            self.back_pos - 1
        };
        self.items_left -= 1;
        it
    }
}

impl<'a, T> ExactSizeIterator for CircularVecIter<'a, T> {
    fn len(&self) -> usize {
        self.items_left
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

    #[test]
    fn test_double_ended_iterator() {
        let mut cv: CircularVec<i32> = CircularVec::new(3);
        cv.push(1);
        cv.push(2);
        cv.push(3);

        let mut iter = cv.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next_back(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);

        // Test collecting into a vector from both ends
        let collected: Vec<_> = cv.iter().collect();
        assert_eq!(collected, vec![&1, &2, &3]);

        // Test pure reverse iteration
        let reverse: Vec<_> = cv.iter().rev().collect();
        assert_eq!(reverse, vec![&3, &2, &1]);
    }

    #[test]
    fn test_with_strings() {
        let mut cv: CircularVec<String> = CircularVec::new(2);
        cv.push(String::from("hello"));
        cv.push(String::from("world"));
        assert_eq!(cv.get(0), Some(&String::from("hello")));
        assert_eq!(cv.get(1), Some(&String::from("world")));

        cv.push(String::from("rust"));
        assert_eq!(cv.get(0), Some(&String::from("world")));
        assert_eq!(cv.get(1), Some(&String::from("rust")));
    }
}
