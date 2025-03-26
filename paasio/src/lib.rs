use std::io::{Read, Result, Write};
use std::marker::PhantomData;

pub struct ReadStats<R> {
    wrapped: R,
    bytes_read: usize,
    read_count: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            bytes_read: 0,
            read_count: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.read_count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.wrapped.read(buf)?;
        self.bytes_read += bytes;
        self.read_count += 1;
        Ok(bytes)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    bytes_wrote: usize,
    write_count: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            bytes_wrote: 0,
            write_count: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_wrote
    }

    pub fn writes(&self) -> usize {
        self.write_count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = self.wrapped.write(buf)?;
        self.bytes_wrote += bytes;
        self.write_count += 1;
        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
