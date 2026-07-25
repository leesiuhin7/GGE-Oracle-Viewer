import * as backend from "../../pkg/gge_oracle_viewer_wasm";
import type SyncFile from "./file";

export default async function downloadData(file: SyncFile): Promise<boolean> {
  const url = import.meta.env.VITE_DATA_URL;
  const response = await fetch(url);

  const stream = response.body;
  if (!stream) {
    return false;
  }

  const decompressor = backend.Decompressor.from_file(file);
  if (!decompressor) {
    return false;
  }
  for await (const chunk of stream) {
    const success = decompressor.push(chunk);
    if (!success) {
      return false;
    }
  }
  return decompressor.finish();
}
