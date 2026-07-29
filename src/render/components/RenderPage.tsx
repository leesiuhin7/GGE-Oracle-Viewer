import { useEffect, useState } from "react";
import type { Field, ResultSnapshots } from "../../backend";
import { SnapshotDetail, type Snapshot } from "./display";
import FieldPicker from "./FieldPicker";
import PaginationControl from "./PaginationControl";
import SnapshotPicker from "./SnapshotPicker";

export default function RenderPage({
  readResult,
  refreshSignal,
}: {
  readResult: (
    skip: number,
    take: number,
    fields: Field[],
  ) => Promise<ResultSnapshots | undefined>;
  refreshSignal?: Record<string, never>;
}) {
  const [previewFields, setPreviewFields] = useState<Field[]>([]);
  const [detailFields, setDetailFields] = useState<Field[]>([]);

  const [snapshots, setSnapshots] = useState<Snapshot[]>([]);
  const [selectedId, setSelectedId] = useState<number | undefined>(undefined);
  const [selectedSnapshot, setSelectedSnapshot] = useState<
    Snapshot | undefined
  >(undefined);

  const [page, setPage] = useState(0);
  const [size, setSize] = useState(10);

  useEffect(() => {
    (async () => {
      const offset = page * size;
      const result = await readResult(offset, size, previewFields);

      if (result?.result === undefined) {
        return;
      }
      setSnapshots(
        result.result.map((snapshot, index) => ({
          data: snapshot,
          metadata: {
            id: offset + index,
            type: result.type,
          },
        })),
      );
    })();
  }, [page, size, previewFields, readResult, refreshSignal]);

  useEffect(() => {
    (async () => {
      if (selectedId === undefined) {
        return;
      }
      const result = await readResult(selectedId, 1, detailFields);
      if (result === undefined) {
        return;
      }
      const snapshot = result.result?.at(0);
      setSelectedSnapshot({
        data: snapshot,
        metadata: {
          id: selectedId,
          type: result.type,
        },
      });
    })();
  }, [selectedId, detailFields, readResult, refreshSignal]);

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        gap: 20,
        minHeight: 0,
        height: "stretch",
      }}
    >
      <FieldPicker
        onPreviewUpdate={setPreviewFields}
        onDetailUpdate={setDetailFields}
      />
      <div
        style={{
          display: "flex",
          gap: 20,
          width: "stretch",
          minHeight: 0,
          height: "stretch",
        }}
      >
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            gap: 10,
            alignItems: "center",
            flex: "1",
          }}
        >
          <SnapshotPicker snapshots={snapshots} onSelect={setSelectedId} />
          <PaginationControl
            page={page}
            setPage={setPage}
            size={size}
            setSize={setSize}
          />
        </div>
        <div style={{ flex: "1", minHeight: 0 }}>
          {selectedSnapshot && <SnapshotDetail snapshot={selectedSnapshot} />}
        </div>
      </div>
    </div>
  );
}
