use std::io::{Error, Read, Write};

use crate::query::SnapshotInfo;

pub(super) fn encode(writer: &mut impl Write, snapshot_info: &SnapshotInfo) -> Result<(), Error> {
    let mut buffer = [0u8; 12];
    // Using unwrap because conversion from usize to u64 shouldn't fail
    let block_id = u64::try_from(snapshot_info.block_id).unwrap();
    let snapshot_id = snapshot_info.snapshot_id;
    buffer[..8].copy_from_slice(&block_id.to_be_bytes());
    buffer[8..12].copy_from_slice(&snapshot_id.to_be_bytes());

    writer.write_all(&buffer)?;
    Ok(())
}

pub(super) fn decode(reader: &mut impl Read) -> Result<SnapshotInfo, Error> {
    let mut buffer = [0u8; 12];
    reader.read_exact(&mut buffer)?;

    let (block_id_bytes, snapshot_id_bytes) = buffer.split_at(8);
    // Using unwrap since the slice size is guaranteed to be correct
    let block_id = u64::from_be_bytes(block_id_bytes.try_into().unwrap());
    let snapshot_id = u32::from_be_bytes(snapshot_id_bytes.try_into().unwrap());

    Ok(SnapshotInfo {
        block_id: block_id.try_into().unwrap_or(usize::MAX),
        snapshot_id,
    })
}
