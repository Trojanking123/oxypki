use bytes::{BufMut, BytesMut};

use crate::error::PkiResult;

use super::FormatType;

pub trait Formatter {
    fn format(&self, tp: FormatType) -> PkiResult<BytesMut>;
}

pub trait PutChar {
    fn put_tab(&mut self, n: usize);
    fn put_newline(&mut self, n: usize);
    fn put_str(&mut self, s: &str);
}

impl PutChar for BytesMut {
    fn put_tab(&mut self, n: usize) {
        self.put(&b"    ".repeat(n)[..]);
    }
    fn put_newline(&mut self, n: usize) {
        self.put(&b"\n".repeat(n)[..]);
    }
    fn put_str(&mut self, s: &str) {
        self.put(s.as_bytes());
    }
}
