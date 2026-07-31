import styles from "./init-page.module.css";

export default function InitSuccess() {
  return (
    <div className={styles.container}>
      <span style={{ fontSize: 32, color: "rgb(0, 255, 0)" }}>
        Initialization completed
      </span>
      <span>Everything is ready</span>
    </div>
  );
}
