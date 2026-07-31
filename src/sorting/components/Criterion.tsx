import type { Options } from "../criterionMap";

export default function Criterion({
  name,
  options,
  asc,
  movable,
  onDirectionUpdate,
  onPriorityUpdate,
  onRemove,
}: {
  name: string;
  options: Options;
  asc: boolean;
  movable: boolean;
  onDirectionUpdate: (asc: boolean) => void;
  onPriorityUpdate: () => void;
  onRemove: () => void;
}) {
  const { ascName, descName } = options;
  return (
    <span
      style={{
        display: "flex",
        gap: 20,
        justifyContent: "space-between",
      }}
    >
      <span style={{ display: "flex", gap: 10 }}>
        {name}
        <select
          value={asc ? "asc" : "desc"}
          onChange={(event) => onDirectionUpdate(event.target.value === "asc")}
        >
          <option value="asc">{ascName}</option>
          <option value="desc">{descName}</option>
        </select>
      </span>
      <span style={{ display: "flex", gap: 5 }}>
        <button
          onClick={onPriorityUpdate}
          style={{ visibility: movable ? "visible" : "hidden" }}
        >
          ↑
        </button>
        <button onClick={onRemove}>x</button>
      </span>
    </span>
  );
}
