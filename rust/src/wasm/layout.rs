use wasm_bindgen::prelude::*;

use crate::{
    data::Layout,
    wasm::file_wrapper::{FileReader, FileWrapper, FileWriter, SyncFile},
};

#[wasm_bindgen]
pub struct LayoutWrapper {
    layout: Layout,
}

#[wasm_bindgen]
impl LayoutWrapper {
    pub fn from_data(sync_file: SyncFile) -> Option<Self> {
        let wrapper = FileWrapper::new(sync_file);
        let mut reader = FileReader::new(wrapper);
        match Layout::from_data(&mut reader) {
            Ok(layout) => Some(LayoutWrapper { layout }),
            Err(_) => None,
        }
    }

    pub fn from_reader(sync_file: SyncFile) -> Option<Self> {
        let wrapper = FileWrapper::new(sync_file);
        let mut reader = FileReader::new(wrapper);
        match Layout::from_reader(&mut reader) {
            Ok(layout) => Some(LayoutWrapper { layout }),
            Err(_) => None,
        }
    }

    pub fn to_writer(&self, sync_file: SyncFile) -> bool {
        let wrapper = FileWrapper::new(sync_file);
        let mut writer = FileWriter::new(wrapper);
        self.layout.to_writer(&mut writer).is_ok()
    }

    pub(super) fn into_layout(self) -> Layout {
        self.layout
    }
}
