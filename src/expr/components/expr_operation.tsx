import { useContext, useEffect, useRef, useState } from "react";
import List, { type ListHandle } from "../../components/list";
import { ExprIdsContext } from "./Builder";
import styles from "./expr.module.css";

interface ItemProps {
  remove: () => void;
  onRemove: () => void;
  onUpdate: (exprId: number) => void;
}

function Item({ remove, onRemove, onUpdate }: ItemProps) {
  const [exprId, setExprId] = useState<number>(0);

  useEffect(() => {
    onUpdate(exprId);
  }, [exprId]);

  const exprIds = useContext(ExprIdsContext);
  return (
    <span className={`${styles.border} ${styles["title-bar"]}`}>
      <span style={{ display: "flex", gap: 5 }}>
        Expr
        <select
          value={exprId}
          onChange={(event) => setExprId(Number(event.target.value))}
        >
          {exprIds.map((exprId) => (
            <option value={exprId} key={exprId}>
              {exprId + 1}
            </option>
          ))}
        </select>
      </span>
      <button
        onClick={() => {
          remove();
          onRemove();
        }}
      >
        x
      </button>
    </span>
  );
}

interface UpdateProps {
  onUpdate(exprIds: Set<number>): void;
}

function ExprIdList({ title, onUpdate }: UpdateProps & { title: string }) {
  const ref = useRef<ListHandle>(null);

  const exprIdMap = useRef<Map<number, number>>(new Map());
  const updateMap = (callback: (map: Map<number, number>) => void) => {
    callback(exprIdMap.current);
    onUpdate(new Set(exprIdMap.current.values()));
  };

  return (
    <div
      className={styles.border}
      style={{
        display: "flex",
        flexDirection: "column",
        padding: 5,
        paddingTop: 10,
      }}
    >
      <div className={styles["title-bar"]}>
        {title}
        <button
          onClick={() =>
            ref.current?.append(({ id, remove }) => {
              return Item({
                remove,
                onRemove() {
                  updateMap((map) => {
                    map.delete(id);
                  });
                },
                onUpdate(exprId) {
                  updateMap((map) => {
                    map.set(id, exprId);
                  });
                },
              });
            })
          }
        >
          +
        </button>
      </div>
      <div className={styles.list}>
        <List ref={ref}></List>
      </div>
    </div>
  );
}

export function MatchAll({ onUpdate }: UpdateProps) {
  return <ExprIdList title="Match all" onUpdate={onUpdate} />;
}

export function MatchAny({ onUpdate }: UpdateProps) {
  return <ExprIdList title="Match any" onUpdate={onUpdate} />;
}
