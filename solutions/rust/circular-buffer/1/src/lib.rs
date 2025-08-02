use std::iter::repeat_with;

pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    read: usize,
    write: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: repeat_with(|| None).take(capacity).collect(),
            read: 0,
            write: 0
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.data[self.write].is_some() {
            Err(Error::FullBuffer)
        } else {
            self.overwrite(element);
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if let Some(element) = self.data[self.read].take() {
            self.read = self.next(self.read);
            Ok(element)
        } else {
            Err(Error::EmptyBuffer)
        }
    }

    pub fn clear(&mut self) {
        for element in self.data.iter_mut() {
            *element = None;
        }
        self.read = 0;
        self.write = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        if self.data[self.write].replace(element).is_some() {
            self.read = self.next(self.read);
        }
        self.write = self.next(self.write);
    }

    fn next(&self, index: usize) -> usize {
        (index + 1) % self.data.len()
    }
}
 