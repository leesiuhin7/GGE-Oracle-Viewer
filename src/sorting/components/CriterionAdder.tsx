export default function CriterionAdder({
  name,
  disabled,
  onAdd,
}: {
  name: string;
  disabled: boolean;
  onAdd: () => void;
}) {
  return (
    <span style={{ display: "flex", gap: 10, justifyContent: "space-between" }}>
      {name}
      <button onClick={onAdd} disabled={disabled}>
        +
      </button>
    </span>
  );
}
