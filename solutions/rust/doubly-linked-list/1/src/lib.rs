use std::{marker::PhantomData, ptr::null_mut};

mod pre_implemented;

pub struct LinkedList<T> {
    front: *mut Node<T>,
    back: *mut Node<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: null_mut(),
            back: null_mut(),
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        let current = self.front;
        Cursor {
            list: self,
            current,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        let current = self.back;
        Cursor {
            list: self,
            current,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        let current = self.front;
        Iter {
            _phantom: &PhantomData,
            current,
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut curr = self.front;
        while !curr.is_null() {
            unsafe {
                let next = (*curr).next;
                drop(Box::from_raw(curr));
                curr = next;
            }
        }
    }
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

impl<T> Node<T> {
    fn new_ptr(data: T) -> *mut Self {
        Box::into_raw(Box::new(Self {
            data,
            next: null_mut(),
            prev: null_mut(),
        }))
    }
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    current: *mut Node<T>,
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            if self.current.is_null() {
                None
            } else {
                Some(&mut (*self.current).data)
            }
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            if self.current.is_null() || (*self.current).next.is_null() {
                None
            } else {
                self.current = (*self.current).next;
                Some(&mut (*self.current).data)
            }
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            if self.current.is_null() || (*self.current).prev.is_null() {
                None
            } else {
                self.current = (*self.current).prev;
                Some(&mut (*self.current).data)
            }
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        let curr = self.current;
        if curr.is_null() {
            None
        } else {
            unsafe {
                let Node { prev, next, .. } = *curr;
                self.current = null_mut();

                if prev.is_null() {
                    self.list.front = next;
                } else {
                    (*prev).next = next;
                    self.current = prev;
                }
                if next.is_null() {
                    self.list.back = prev;
                } else {
                    (*next).prev = prev;
                    self.current = next;
                }

                let data = std::ptr::read(&(*curr).data);
                drop(Box::from_raw(curr));

                self.list.len -= 1;
                Some(data)
            }
        }
    }

    pub fn insert_after(&mut self, element: T) {
        let new = Node::new_ptr(element);
        if self.current.is_null() {
            self.list.back = new;
            self.list.front = new;
            self.current = new;
        } else {
            unsafe {
                let next = (*self.current).next;
                if next.is_null() {
                    self.list.back = new;
                } else {
                    (*next).prev = new;
                    (*new).next = next;
                }
                (*new).prev = self.current;
                (*self.current).next = new;
            };
        };
        self.list.len += 1;
    }

    pub fn insert_before(&mut self, element: T) {
        let new = Node::new_ptr(element);
        if self.current.is_null() {
            self.list.back = new;
            self.list.front = new;
            self.current = new;
        } else {
            unsafe {
                let prev = (*self.current).prev;
                if prev.is_null() {
                    self.list.front = new;
                } else {
                    (*prev).next = new;
                    (*new).prev = prev;
                }
                (*new).next = self.current;
                (*self.current).prev = new;
            };
        };
        self.list.len += 1;
    }
}

pub struct Iter<'a, T> {
    current: *const Node<T>,
    _phantom: &'a PhantomData<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.current.is_null() {
            None
        } else {
            unsafe {
                let value = &(*self.current).data;
                self.current = (*self.current).next;
                Some(value)
            }
        }
    }
}
