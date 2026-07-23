use std::io::Write;

use wasm_bindgen::prelude::*;
use zstd::stream::write::Decoder;

use crate::wasm::file_wrapper::{FileWrapper, FileWriter, SyncFile};

#[wasm_bindgen]
pub struct Decompressor {
    decoder: Decoder<'static, FileWriter>,
}

#[wasm_bindgen]
impl Decompressor {
    pub fn from_file(file: SyncFile) -> Option<Self> {
        let writer = FileWriter::new(FileWrapper::new(file));
        Decoder::new(writer).ok().map(|decoder| Self { decoder })
    }

    pub fn push(&mut self, buffer: &[u8]) -> bool {
        self.decoder.write_all(buffer).is_ok()
    }

    pub fn finish(mut self) -> bool {
        self.decoder.flush().is_ok()
    }
}
