import { SnapshotPreview, type Snapshot } from "./display";
import styles from "./render.module.css";

export default function SnapshotPicker({
  snapshots,
  onSelect,
}: {
  snapshots: Snapshot[];
  onSelect: (id: number) => void;
}) {
  return (
    <div
      className={`${styles.border} ${styles["scroll-container"]}`}
      style={{
        gap: 10,
        padding: 10,
        width: "stretch",
      }}
    >
      {snapshots.map((snapshot) => (
        <div
          key={snapshot.metadata.id}
          onClick={() => onSelect(snapshot.metadata.id)}
        >
          <SnapshotPreview snapshot={snapshot} />
        </div>
      ))}
    </div>
  );
}
