use std::{
    io::{Read, Result, Seek, SeekFrom, Write},
    os::unix::fs::FileExt,
    path::Path,
};

use parking_lot::RwLock;

pub struct Options {}

impl Options {
    pub fn read(&self, _: bool) -> &Self {
        self
    }

    pub fn write(&self, _: bool) -> &Self {
        self
    }

    pub fn custom_flags(&self, _: i32) -> &Self {
        self
    }

    pub fn create(&self, _: bool) -> &Self {
        self
    }

    pub fn open<P: AsRef<Path>>(&self, _: P) -> Result<File> {
        Ok(File::new())
    }
}

pub struct Metadata {
    len: usize,
}

impl Metadata {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[derive(Debug)]
pub struct File {
    data: RwLock<Vec<u8>>,
}

impl File {
    fn new() -> Self {
        File {
            data: RwLock::new(Vec::new()),
        }
    }

    pub fn open<P: AsRef<Path>>(_: P) -> Result<File> {
        Ok(File::new())
    }

    pub fn options() -> Options {
        Options {}
    }

    pub fn metadata(&self) -> Result<Metadata> {
        Ok(Metadata {
            len: self.data.read().len(),
        })
    }

    pub fn create_new<P: AsRef<Path>>(_: P) -> Result<File> {
        Ok(File::new())
    }

    pub fn set_len(&self, len: u64) -> Result<()> {
        self.data.write().truncate(len as usize);
        Ok(())
    }

    pub fn sync_all(&self) -> Result<()> {
        Ok(())
    }
}

impl Seek for File {
    fn seek(&mut self, _: SeekFrom) -> Result<u64> {
        Ok(0)
    }
}

impl Seek for &File {
    fn seek(&mut self, _: SeekFrom) -> Result<u64> {
        Ok(0)
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let data = self.data.read();
        let len = data.len();
        let read_len = usize::min(buf.len(), len);

        let s = &data[..read_len];
        buf[..read_len].copy_from_slice(s);

        Ok(read_len)
    }
}

impl Read for &File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let data = self.data.read();
        let len = data.len();
        let read_len = usize::min(buf.len(), len);

        let s = &data[..read_len];
        buf[..read_len].copy_from_slice(s);

        Ok(read_len)
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.data.write().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Write for &File {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.data.write().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl FileExt for File {
    fn read_at(&self, buf: &mut [u8], offset: u64) -> Result<usize> {
        let offset = offset as usize;

        let data = self.data.read();

        if offset >= data.len() {
            return Ok(0);
        }

        let available_len = data.len() - offset as usize;
        let to_read = buf.len().min(available_len);
        buf[..to_read].copy_from_slice(&data[offset..offset + to_read]);

        Ok(to_read)
    }

    fn write_at(&self, buf: &[u8], offset: u64) -> Result<usize> {
        let mut data = self.data.write();
        let end_offset = offset as usize + buf.len();
        if end_offset > data.len() {
            data.resize(end_offset, 0);
        }
        data[offset as usize..end_offset].copy_from_slice(buf);
        Ok(buf.len())
    }
}
