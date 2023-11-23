use std::any::Any;

use bytes::{BufMut, BytesMut};

use crate::error::PkiResult;

use super::FormatType;

pub trait Formatter {
    type Target;
    fn format(&self, _tp: FormatType) -> PkiResult<Self::Target>;
}

pub trait PutChar: Any {
    fn put_tab(&mut self, n: usize);
    fn put_newline(&mut self, n: usize);
    fn put_str(&mut self, s: &str);
    fn put_byte(&mut self, s: &[u8]);
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
    fn put_byte(&mut self, s: &[u8]) {
        self.put(s)
    }
}

impl PutChar for String {
    fn put_tab(&mut self, n: usize) {
        self.push_str(&"    ".repeat(n))
    }
    fn put_newline(&mut self, n: usize) {
        self.push_str(&"\n".repeat(n)[..]);
    }
    fn put_str(&mut self, s: &str) {
        self.push_str(s);
    }
    fn put_byte(&mut self, _: &[u8]) {
        unimplemented!()
    }
}

impl PutChar for Vec<u8> {
    fn put_tab(&mut self, n: usize) {
        self.put_bytes( b' ', n * 4)
    }
    fn put_newline(&mut self, n: usize) {
        self.put_bytes(b'\n', n);
    }
    fn put_str(&mut self, s: &str) {
        self.put_slice(s.as_bytes());
    }
    fn put_byte(&mut self, s: &[u8]) {
        self.put_slice(s)
    }
}
