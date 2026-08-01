export default function OptionPage({
  onReload,
  reloadDisabled,
}: {
  onReload: () => void;
  reloadDisabled: boolean;
}) {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        gap: 10,
        alignItems: "center",
      }}
    >
      <button
        onClick={onReload}
        disabled={reloadDisabled}
        style={{ fontSize: 24, width: "40%", height: 50 }}
      >
        Synchronize with server
      </button>
      <span style={{ fontSize: 20, color: "rgb(255, 0, 0)" }}>
        Synchronizing downloads a large file and may take a while
      </span>
    </div>
  );
}
