use std::io::{Read, Seek, Write};

pub(crate) struct Streams<R: Read + Seek, W: Write> {
    pub(crate) reader: R,
    pub(crate) writer: W,
}
