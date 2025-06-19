use std::marker::PhantomData;

pub struct CircularBuffer<T: Clone> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
    phantom: PhantomData<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: vec![None; capacity],
            head: 0,
            tail: 0,
            size: 0,
            capacity,
            phantom: PhantomData,
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.size == self.capacity {
            return Err(Error::FullBuffer);
        }
        self.buffer[self.tail] = Some(_element);
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;

        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.size == 0 {
            return Err(Error::EmptyBuffer);
        }

        let element = self.buffer[self.head].take().unwrap();
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;

        Ok(element)
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.buffer.resize(self.capacity, None);
        self.head = 0;
        self.tail = 0;
        self.size = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        self.buffer[self.tail] = Some(element);
        self.tail = (self.tail + 1) % self.capacity;
        if self.size == self.capacity {
            self.head = (self.head + 1) % self.capacity;
            self.size = self.capacity;
        } else {
            self.size += 1;
        }
    }
}
