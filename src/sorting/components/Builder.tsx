import { useState } from "react";
import type { SortingCriterion } from "../../backend";
import Criteria from "../criteria";
import criterionMap from "../criterionMap";
import Criterion from "./Criterion";
import CriterionAdder from "./CriterionAdder";
import styles from "./sorting.module.css";

export default function Builder({
  onUpdate,
}: {
  onUpdate: (criteria: SortingCriterion[]) => void;
}) {
  const [criteria, setCriteria] = useState<Criteria>(new Criteria());
  const updateCriteria = () => {
    const newCriteria = new Criteria(criteria.value());
    setCriteria(newCriteria);
    onUpdate(newCriteria.value());
  };

  return (
    <div
      className={styles.border}
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        gap: 10,
        padding: 10,
      }}
    >
      <span style={{ fontSize: 32 }}>Sorting priority</span>
      <div
        style={{
          display: "flex",
          gap: 50,
          justifyContent: "space-between",
          width: "stretch",
        }}
      >
        <table
          style={{
            borderCollapse: "collapse",
            height: "min-content",
            flex: "1",
          }}
        >
          <tbody>
            {criteria.value().map(({ field, desc }, index) => {
              const { name, options } = criterionMap[field];
              return (
                <tr key={field}>
                  <td className={styles.border} style={{ padding: 5 }}>
                    <Criterion
                      name={name}
                      options={options}
                      asc={!desc}
                      movable={index !== 0}
                      onDirectionUpdate={(asc) => {
                        if (criteria.updateDirection(index, asc)) {
                          updateCriteria();
                        }
                      }}
                      onPriorityUpdate={() => {
                        if (criteria.moveCriterion(index)) {
                          updateCriteria();
                        }
                      }}
                      onRemove={() => {
                        if (criteria.removeCriterion(index) !== undefined) {
                          updateCriteria();
                        }
                      }}
                    />
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
        <table style={{ borderCollapse: "collapse", height: "min-content" }}>
          <tbody>
            {Object.entries(criterionMap).map(([field, { name }]) => (
              <tr key={field}>
                <td className={styles.border} style={{ padding: 5 }}>
                  <CriterionAdder
                    name={name}
                    disabled={criteria.isUsed(field)}
                    onAdd={() => {
                      if (criteria.addCriterion({ field }) !== undefined) {
                        updateCriteria();
                      }
                    }}
                  />
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
