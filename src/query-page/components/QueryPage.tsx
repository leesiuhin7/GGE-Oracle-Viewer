import { useState } from "react";
import type { ExprTree, SortingCriterion } from "../../backend";
import ExprBuilder from "../../expr/";
import SortBuilder from "../../sorting";
import ActionBar from "./ActionBar";
import styles from "./query-page.module.css";

export default function QueryPage({
  actionsDisabled,
  onMatch,
  onSort,
}: {
  actionsDisabled: boolean;
  onMatch: (expr: ExprTree | undefined) => void;
  onSort: (criteria: SortingCriterion[]) => void;
}) {
  const [expr, setExpr] = useState<ExprTree | undefined>(undefined);
  const [sortingCriteria, setSortingCriteria] = useState<SortingCriterion[]>(
    [],
  );

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        gap: 10,
        padding: 20,
        height: "stretch",
      }}
    >
      <div
        style={{
          display: "flex",
          gap: 20,
          flex: "1",
          minHeight: 0,
        }}
      >
        <div className={styles["scroll-container"]}>
          <ExprBuilder onUpdate={setExpr} />
        </div>
        <div className={styles["scroll-container"]}>
          <SortBuilder onUpdate={setSortingCriteria} />
        </div>
      </div>
      <ActionBar
        disabled={actionsDisabled}
        onMatch={() => {
          onMatch(expr);
        }}
        onSort={() => {
          onSort(sortingCriteria);
        }}
      />
    </div>
  );
}
