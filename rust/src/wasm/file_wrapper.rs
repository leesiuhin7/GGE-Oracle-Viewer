use std::io::{BufReader, BufWriter, IntoInnerError, Read, Seek, SeekFrom, Write};

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/../src/backend/file.ts")]
extern "C" {
    pub type SyncFile;

    #[wasm_bindgen(method)]
    fn close(this: &SyncFile);

    #[wasm_bindgen(method)]
    fn flush(this: &SyncFile);

    #[wasm_bindgen(method)]
    fn read(this: &SyncFile, size: u64, offset: u64) -> Vec<u8>;

    #[wasm_bindgen(method)]
    fn truncate(this: &SyncFile, size: u64);

    #[wasm_bindgen(method)]
    fn write(this: &SyncFile, buffer: &[u8], offset: u64) -> u64;
}

pub(super) struct FileWrapper {
    sync_file: SyncFile,
    pointer: u64,
}

impl FileWrapper {
    pub(super) fn new(sync_file: SyncFile) -> Self {
        FileWrapper {
            sync_file,
            pointer: 0,
        }
    }

    #[allow(dead_code)] // Keeping this to mirror interface
    pub(super) fn close(self) {
        self.sync_file.close();
    }

    pub(super) fn truncate(&mut self, size: u64) {
        self.sync_file.truncate(size);
        if self.pointer > size {
            self.pointer = size;
        }
    }
}

impl Read for FileWrapper {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let array = self.sync_file.read(buf.len() as u64, self.pointer);
        buf[..array.len()].copy_from_slice(&array);
        self.pointer += array.len() as u64;
        Ok(array.len())
    }
}

impl Write for FileWrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let size = self.sync_file.write(buf, self.pointer);
        self.pointer += size;
        Ok(usize::try_from(size).unwrap())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.sync_file.flush();
        Ok(())
    }
}

impl Seek for FileWrapper {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Start(n) => self.pointer = n,
            SeekFrom::Current(n) => self.pointer = self.pointer.saturating_add_signed(n),
            SeekFrom::End(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "SeekFrom::End is not supported",
            ))?,
        }
        Ok(self.pointer)
    }
}

pub(super) struct FileReader {
    reader: BufReader<FileWrapper>,
}

impl FileReader {
    pub(super) fn new(file_wrapper: FileWrapper) -> Self {
        // Using a very large buffer size to reduce JS calls
        FileReader {
            reader: BufReader::with_capacity(1_048_576, file_wrapper),
        }
    }

    pub(super) fn into_wrapper(self) -> FileWrapper {
        self.reader.into_inner()
    }
}

impl Read for FileReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl Seek for FileReader {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let offset = match pos {
            SeekFrom::Start(n) => n.cast_signed() - self.reader.stream_position()?.cast_signed(),
            SeekFrom::Current(n) => n,
            SeekFrom::End(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "SeekFrom::End is not supported",
            ))?,
        };
        self.reader.seek_relative(offset)?;
        self.reader.stream_position()
    }
}

pub(super) struct FileWriter {
    writer: BufWriter<FileWrapper>,
}

impl FileWriter {
    pub(super) fn new(file_wrapper: FileWrapper) -> Self {
        FileWriter {
            writer: BufWriter::with_capacity(1_048_576, file_wrapper),
        }
    }

    pub(super) fn into_wrapper(
        self,
    ) -> Result<FileWrapper, IntoInnerError<BufWriter<FileWrapper>>> {
        self.writer.into_inner()
    }
}

impl Write for FileWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl Seek for FileWriter {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.writer.seek(pos)
    }
}
