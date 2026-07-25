import * as backend from "../../pkg/gge_oracle_viewer_wasm";
import SyncFile from "./file";

class Layout {
  private file: SyncFile;
  private _layout: backend.LayoutWrapper;

  constructor(file: SyncFile) {
    this.file = file;
    this._layout = this.readFromData();
  }

  public get layout(): backend.LayoutWrapper {
    return this._layout;
  }

  private readFromData(): backend.LayoutWrapper {
    const layout = backend.LayoutWrapper.from_data(this.file);
    if (!layout) {
      throw new Error("Loading layout failed.");
    }
    return layout;
  }
}

export interface Files {
  dataFile: SyncFile;
  tempFile1: SyncFile;
  tempFile2: SyncFile;
  sortResultFile: SyncFile;
}

export class Engine {
  private layout: Layout;
  private engine: backend.Engine;
  private readonly files: Files;

  constructor(files: Files) {
    this.files = files;

    this.layout = new Layout(files.dataFile);
    this.engine = new backend.Engine(
      new backend.Files(files.dataFile, files.tempFile1, files.tempFile2),
      this.layout.layout,
    );
  }

  public matchAll(expr: backend.Expr): backend.MatchResult {
    return this.engine.match_all(expr);
  }

  public sort(
    result: backend.MatchResult,
    comparators: backend.Comparator[],
  ): backend.SortingResult | undefined {
    return this.engine.sort(result, comparators, this.files.sortResultFile);
  }

  public buildSnapshot(
    snapshotInfo: backend.SnapshotInfo,
    fields: backend.WrapperField[],
  ) {
    return this.engine.build_snapshot(snapshotInfo, fields);
  }

  public updateData(): boolean {
    try {
      this.layout = new Layout(this.files.dataFile);
    } catch {
      return false;
    }
    const { dataFile, tempFile1, tempFile2 } = this.files;
    this.engine = new backend.Engine(
      new backend.Files(dataFile, tempFile1, tempFile2),
      this.layout.layout,
    );
    return true;
  }
}
