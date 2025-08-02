use std::iter::FromIterator;

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    tail: Node<T>,
}

#[derive(Debug)]
enum Node<T> {
    Link(T, Box<Node<T>>),
    Empty,
}

use Node::*;

impl<T> Node<T> {
    fn is_empty(&self) -> bool {
        match self {
            Empty => true,
            _ => false,
        }
    }

    fn drop(self) -> Option<T> {
        match self {
            Link(data, _) => Some(data),
            Empty => None,
        }
    }

    fn next_mut(&mut self) -> &mut Node<T> {
        match self {
            Link(_, b) => b.as_mut(),
            _ => panic!(),
        }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { tail: Empty }
    }

    pub fn is_empty(&self) -> bool {
        self.tail.is_empty()
    }

    pub fn len(&self) -> usize {
        let mut cursor: &Node<T> = &self.tail;
        let mut len = 0;
        while let Link(_, next) = cursor {
            len += 1;
            cursor = next;
        }
        len
    }

    pub fn push(&mut self, data: T) {
        let old = std::mem::replace(&mut self.tail, Link(data, Box::new(Empty)));
        *self.tail.next_mut() = old;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let new_tail = std::mem::replace(self.tail.next_mut(), Empty);
            let old_tail = std::mem::replace(&mut self.tail, new_tail);
            old_tail.drop()
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.tail {
            Link(data, _) => Some(data),
            Empty => None,
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut items = Vec::from(self);
        SimpleLinkedList::from_iter(items.drain(..).rev())
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut l = SimpleLinkedList::new();
        for i in iter {
            l.push(i)
        }
        l
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(l: SimpleLinkedList<T>) -> Vec<T> {
        let mut v = Vec::new();
        let mut old = l;
        while let Some(data) = old.pop() {
            v.push(data)
        }
        v.reverse();
        v
    }
}
