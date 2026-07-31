import styles from "./init-page.module.css";

export default function InitFailed() {
  return (
    <div className={styles.container}>
      <span style={{ fontSize: 32, color: "rgb(255, 0, 0)" }}>
        Initialization failed
      </span>
      <span>Refresh page and try again</span>
    </div>
  );
}
