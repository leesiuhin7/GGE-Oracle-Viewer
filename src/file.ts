export class SyncFile {
  handle: FileSystemSyncAccessHandle;

  constructor(handle: FileSystemSyncAccessHandle) {
    this.handle = handle;
  }

  close() {
    this.handle.close();
  }

  flush() {
    this.handle.flush();
  }

  read(size: bigint, offset: bigint): Uint8Array {
    const buffer = new Uint8Array(Number(size));
    const read_size = this.handle.read(buffer, { at: Number(offset) });
    return buffer.slice(0, read_size);
  }

  truncate(size: bigint) {
    this.handle.truncate(Number(size));
  }

  write(buffer: Uint8Array, offset: bigint): number {
    return this.handle.write(buffer, { at: Number(offset) });
  }
}
