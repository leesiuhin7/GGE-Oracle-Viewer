import { Activity, useState } from "react";
import type { ExprInfo } from "../adapter";
import { MatchAll, MatchAny } from "./expr_operation";
import Filter from "./filter";

export default function Expr({
  onUpdate,
}: {
  onUpdate: (info: ExprInfo) => void;
}) {
  const [exprType, setExprType] = useState(0);

  return (
    <>
      <select
        value={exprType}
        onChange={(event) => setExprType(Number(event.target.value))}
      >
        <option value={0}>Filter</option>
        <option value={1}>Match all</option>
        <option value={2}>Match any</option>
      </select>

      <Activity mode={exprType === 0 ? "visible" : "hidden"}>
        <Filter
          onUpdate={({ id, args }) => {
            onUpdate({ type: "filter", id, args });
          }}
          onError={() => onUpdate({ type: "error" })}
        />
      </Activity>
      <Activity mode={exprType === 1 ? "visible" : "hidden"}>
        <MatchAll
          onUpdate={(exprIds) => {
            onUpdate({ type: "and", exprIds: [...exprIds] });
          }}
        />
      </Activity>
      <Activity mode={exprType === 2 ? "visible" : "hidden"}>
        <MatchAny
          onUpdate={(exprIds) => {
            onUpdate({ type: "or", exprIds: [...exprIds] });
          }}
        />
      </Activity>
    </>
  );
}
