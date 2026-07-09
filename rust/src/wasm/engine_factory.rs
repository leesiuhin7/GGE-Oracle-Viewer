use std::io::Seek;

use crate::{
    data::{BlockReader, block},
    query,
    sorting::{self, Streams},
    wasm::{
        file_wrapper::{FileReader, FileWrapper, FileWriter, SyncFile},
        layout::LayoutWrapper,
    },
};

pub(super) type QueryEngine = query::Engine<FileReader>;
pub(super) type SortingEngine = sorting::Engine<
    FileReader,
    FileWriter,
    Box<dyn FnMut(Streams<FileReader, FileWriter>) -> Result<Streams<FileReader, FileWriter>, ()>>,
>;

fn swap_streams(
    streams: Streams<FileReader, FileWriter>,
) -> Result<Streams<FileReader, FileWriter>, ()> {
    let Streams { reader, writer } = streams;

    let mut writer_wrapper = reader.into_wrapper();
    writer_wrapper.truncate(0); // Clear the file to reset pointer to the start

    let mut reader_wrapper = writer.into_wrapper().map_err(|_| ())?;
    reader_wrapper
        .seek(std::io::SeekFrom::Start(0))
        .map_err(|_| ())?;

    Ok(Streams {
        reader: FileReader::new(reader_wrapper),
        writer: FileWriter::new(writer_wrapper),
    })
}

pub(super) fn query_engine(data_file: SyncFile, layout_wrapper: LayoutWrapper) -> QueryEngine {
    let reader = FileReader::new(FileWrapper::new(data_file));
    query::Engine::new(BlockReader::new(reader, layout_wrapper.into_layout()))
}

pub(super) fn sorting_engine(temp_file1: SyncFile, temp_file2: SyncFile) -> SortingEngine {
    let reader = FileReader::new(FileWrapper::new(temp_file1));
    let writer = FileWriter::new(FileWrapper::new(temp_file2));
    sorting::Engine::new(
        reader,
        writer,
        Box::new(swap_streams),
        sorting::Parameters {
            buffer_length: 65536,
            memory: 536_870_912, // 512 MB
        },
    )
}

pub(super) struct Producer<'a> {
    query_engine: &'a mut QueryEngine,
}

impl<'a> Producer<'a> {
    pub(super) fn new(query_engine: &'a mut QueryEngine) -> Self {
        Producer { query_engine }
    }
}

impl sorting::Producer for Producer<'_> {
    fn produce(
        &mut self,
        snapshot_info: &query::SnapshotInfo,
        fields: &[block::Field],
    ) -> Option<query::Snapshot> {
        self.query_engine.build_snapshot(snapshot_info, fields)
    }
}
