import styles from "./query-page.module.css";

export default function ActionBar({
  disabled,
  onMatch,
  onSort,
}: {
  disabled: boolean;
  onMatch: () => void;
  onSort: () => void;
}) {
  return (
    <div style={{ display: "flex", gap: 10 }}>
      <button
        className={styles["action-button"]}
        onClick={onMatch}
        disabled={disabled}
      >
        Run query
      </button>
      <button
        className={styles["action-button"]}
        onClick={onSort}
        disabled={disabled}
      >
        Sort
      </button>
    </div>
  );
}
