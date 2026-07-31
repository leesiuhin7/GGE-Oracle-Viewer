import { useState } from "react";
import styles from "./init-page.module.css";

export default function InitPending({
  onInit,
}: {
  onInit: () => Promise<void>;
}) {
  const [disabled, setDisabled] = useState(false);

  return (
    <div className={styles.container} style={{ gap: 10 }}>
      <button
        onClick={() => {
          setDisabled(true);
          onInit().then(() => setDisabled(false));
        }}
        disabled={disabled}
        style={{ fontSize: 24, width: "40%", height: 50 }}
      >
        Initialize
      </button>
      <span style={{ fontSize: 20, color: "rgb(255, 0, 0)" }}>
        Initialization may download a large file and take a while
      </span>
    </div>
  );
}
