import { useRef } from "react";
import List, { type ListHandle } from "../../components/list";
import type { ExprInfo } from "../adapter";
import Expr from "./Expr";
import styles from "./expr.module.css";

interface ItemProps {
  onUpdate: (info: ExprInfo) => void;
  onRemove: () => void;
  remove: () => void;
  id: number;
}

function Item({ onUpdate, onRemove, remove, id }: ItemProps) {
  return (
    <div
      className={styles.border}
      style={{ display: "flex", flexDirection: "column", gap: 10, padding: 5 }}
    >
      <div className={styles["title-bar"]}>
        Expr {id + 1}
        <button
          onClick={() => {
            remove();
            onRemove();
          }}
        >
          x
        </button>
      </div>
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          gap: 5,
        }}
      >
        <Expr
          onUpdate={(info) => {
            onUpdate(info);
          }}
        />
      </div>
    </div>
  );
}

export function Table({
  onUpdate,
  onNewIds,
}: {
  onUpdate: (exprMap: Map<number, ExprInfo>) => void;
  onNewIds: (ids: number[]) => void;
}) {
  const ref = useRef<ListHandle>(null);

  const exprMap = useRef<Map<number, ExprInfo>>(new Map());
  const updateMap = (callback: (map: Map<number, ExprInfo>) => void) => {
    callback(exprMap.current);
    onUpdate(exprMap.current);
  };

  return (
    <div>
      <div className={styles["title-bar"]}>
        Expressions
        <button
          onClick={() => {
            ref.current?.append(({ remove, id }) =>
              Item({
                onUpdate: (info) => updateMap((map) => map.set(id, info)),
                onRemove: () => updateMap((map) => map.delete(id)),
                remove,
                id,
              }),
            );
          }}
        >
          +
        </button>
      </div>
      <div className={styles.list}>
        <List
          ref={ref}
          onChange={(ids) => {
            onNewIds(ids);
          }}
        />
      </div>
    </div>
  );
}
