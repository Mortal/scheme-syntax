use std::io as sio;
use std::io::{Read, Bytes};

pub struct CharsWrap<'a, R> where R: Read {
    c: Bytes<R>,
    e: &'a mut Option<sio::Error>,
}

impl <'a, R> CharsWrap<'a, R> where R: Read {
    pub fn new(c: Bytes<R>, e: &'a mut Option<sio::Error>) -> Self {
        CharsWrap {
            c: c,
            e: e,
        }
    }
}

impl <'a, R> Iterator for CharsWrap<'a, R> where R: Read {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.c.next() {
            Some(Ok(a)) => Some(a as char),
            Some(Err(e)) => {*self.e = Some(e); None},
            None => None,
        }
    }
}
