use std::io::{Read, Result, Write};

pub struct ReadStats<R>{
    inner: R,
    bytes: usize,
    ops: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(inner: R) -> ReadStats<R> {
        Self { inner, bytes: 0, ops: 0 }
    }

    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.ops
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.inner.read(buf);
        if let Ok(ref size) = result {
            self.bytes += size;
            self.ops += 1;
        }
        result
    }
}

pub struct WriteStats<W> {
    inner: W,
    bytes: usize,
    ops: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(inner: W) -> WriteStats<W> {
        Self { inner, bytes: 0, ops: 0 }
    }

    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.ops
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let result = self.inner.write(buf);
        if let Ok(ref size) = result {
            self.bytes += size;
            self.ops += 1;
        }
        result        
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

