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
        assert!(capacity > 0, "Capacity must be greater than 0");
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
                ptr::drop_in_place(self.ptr.add(self.head));
                ptr::write(self.ptr.add(self.head), item);
            }
        }
        self.head = (self.head + 1) % self.capacity;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        // Calculate the actual index, taking into account wraparound
        let actual_index = if self.len == self.capacity {
            // Buffer is full, start from oldest
            (self.head + index) % self.capacity
        } else {
            // Buffer is not full, start from beginning
            index
        };

        unsafe { Some(&*self.ptr.add(actual_index)) }
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
            cv: self,
            current_pos: 0,
            back_pos: self.len as isize - 1,
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
}

impl<'a, T> Iterator for CircularVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos > self.back_pos {
            return None;
        }
        let it = self.cv.get(self.current_pos.try_into().unwrap());
        self.current_pos += 1;
        it
    }
}

impl<'a, T> DoubleEndedIterator for CircularVecIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back_pos < self.current_pos {
            return None;
        }
        let it = self.cv.get(self.back_pos.try_into().unwrap());
        self.back_pos -= 1;
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

    #[test]
    fn test_with_vectors() {
        let mut cv: CircularVec<Vec<i32>> = CircularVec::new(3);
        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5, 6];
        let v3 = vec![7, 8, 9];

        cv.push((&v1).to_owned());
        cv.push((&v2).to_owned());
        cv.push((&v3).to_owned());

        assert_eq!(cv.get(0), Some(&vec![1, 2, 3]));
        assert_eq!(cv.get(1), Some(&vec![4, 5, 6]));
        assert_eq!(cv.get(2), Some(&vec![7, 8, 9]));
        assert_eq!(
            cv.iter().collect::<Vec<_>>(),
            vec![&vec![1, 2, 3], &vec![4, 5, 6], &vec![7, 8, 9]]
        );

        let v4 = vec![10, 11, 12];
        cv.push((&v4).to_owned());

        assert_eq!(cv.get(0), Some(&vec![4, 5, 6]));
        assert_eq!(cv.get(1), Some(&vec![7, 8, 9]));
        assert_eq!(cv.get(2), Some(&vec![10, 11, 12]));
        assert_eq!(
            cv.iter().collect::<Vec<_>>(),
            vec![&vec![4, 5, 6], &vec![7, 8, 9], &vec![10, 11, 12]]
        );

        let v5 = vec![13, 14, 15];
        cv.push((&v5).to_owned());

        assert_eq!(cv.get(0), Some(&vec![7, 8, 9]));
        assert_eq!(cv.get(1), Some(&vec![10, 11, 12]));
        assert_eq!(cv.get(2), Some(&vec![13, 14, 15]));
        assert_eq!(
            cv.iter().collect::<Vec<_>>(),
            vec![&vec![7, 8, 9], &vec![10, 11, 12], &vec![13, 14, 15]]
        );
    }
}
