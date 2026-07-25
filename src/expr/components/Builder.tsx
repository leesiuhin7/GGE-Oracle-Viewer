import { createContext, useRef, useState } from "react";
import type { ExprTree } from "../../backend";
import { resolveExpr, type ExprInfo } from "../adapter";
import Expr from "./Expr";
import styles from "./expr.module.css";
import { Table } from "./Table";

export const ExprIdsContext = createContext<number[]>([]);

export default function Builder({
  onUpdate,
}: {
  onUpdate: (expr: ExprTree | undefined) => void;
}) {
  const [exprIds, setExprIds] = useState<number[]>([]);
  const exprInfo = useRef<ExprInfo>(null);
  const exprMap = useRef<Map<number, ExprInfo>>(new Map());

  const [error, setError] = useState(false);

  const updateExpr = () => {
    if (exprInfo.current !== null) {
      const expr = resolveExpr(exprInfo.current, exprMap.current);
      setError(expr === undefined);
      onUpdate(expr);
    }
  };

  return (
    <ExprIdsContext value={exprIds}>
      <div
        className={styles.border}
        style={{
          display: "flex",
          flexDirection: "column",
          gap: 20,
          padding: 10,
        }}
      >
        <div style={{ display: "flex", flexDirection: "column", gap: 5 }}>
          <h2 style={{ color: error ? "rgb(255, 128, 128)" : "rgb(0, 0, 0)" }}>
            Query expression
          </h2>
          <Expr
            onUpdate={(info) => {
              exprInfo.current = info;
              updateExpr();
            }}
          />
        </div>
        <Table
          onUpdate={(map) => {
            exprMap.current = map;
            updateExpr();
          }}
          onNewIds={(ids) => {
            setExprIds(ids);
          }}
        />
      </div>
    </ExprIdsContext>
  );
}
