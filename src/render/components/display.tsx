import type { Snapshot as SnapshotData } from "../../backend";
import styles from "./render.module.css";
import Values from "./value";

export interface Metadata {
  id: number;
  type: "match" | "sort";
}

export interface Snapshot {
  data: SnapshotData | undefined;
  metadata: Metadata;
}

export function SnapshotDetail({ snapshot }: { snapshot: Snapshot }) {
  const { data, metadata } = snapshot;

  return (
    <div
      className={`${styles.border} ${styles["scroll-container"]}`}
      style={{
        padding: 10,
        backgroundColor: data ? undefined : "rgb(255, 128, 128)",
      }}
    >
      <span style={{ display: "flex", gap: 10, alignItems: "center" }}>
        <span style={{ fontSize: 28 }}>Snapshot {metadata.id + 1}</span>
        <span>({metadata.type === "sort" ? "Ordered" : "Unordered"})</span>
      </span>
      {data && (
        <div className={styles["field-container"]}>
          <Values snapshot={data} />
        </div>
      )}
    </div>
  );
}

export function SnapshotPreview({ snapshot }: { snapshot: Snapshot }) {
  const { data, metadata } = snapshot;

  return (
    <div
      className={styles.border}
      style={{
        display: "flex",
        gap: 20,
        alignItems: "start",
        padding: 5,
        backgroundColor: data ? undefined : "rgb(255, 128, 128)",
      }}
    >
      <span style={{ fontSize: 24 }}>{metadata.id + 1}</span>
      {data && (
        <div
          className={styles["field-container"]}
          style={{
            justifyContent: "space-evenly",
            flex: "auto",
          }}
        >
          <Values snapshot={data} />
        </div>
      )}
    </div>
  );
}
