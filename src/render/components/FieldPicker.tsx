import { useEffect, useState } from "react";
import { Field, type Snapshot } from "../../backend";
import styles from "./render.module.css";

function FieldItem({
  name,
  previewSelected,
  onPreviewUpdate,
  detailSelected,
  onDetailUpdate,
}: {
  name: string;
  previewSelected: boolean;
  onPreviewUpdate: (state: boolean) => void;
  detailSelected: boolean;
  onDetailUpdate: (state: boolean) => void;
}) {
  return (
    <div className={styles.border} style={{ padding: 2 }}>
      {name}
      <div style={{ display: "flex", gap: 10, justifyContent: "space-evenly" }}>
        <label style={{ display: "flex", gap: 5 }}>
          Preview
          <input
            type="checkbox"
            checked={previewSelected}
            onChange={(event) => onPreviewUpdate(event.target.checked)}
          ></input>
        </label>
        <label style={{ display: "flex", gap: 5 }}>
          Detail
          <input
            type="checkbox"
            checked={detailSelected}
            onChange={(event) => onDetailUpdate(event.target.checked)}
          ></input>
        </label>
      </div>
    </div>
  );
}

const fields: Readonly<{
  [K in keyof Snapshot]?: {
    name: string;
    value: Field;
  };
}> = {
  header: {
    name: "Identity",
    value: Field.header,
  },
  timestamp: {
    name: "Time",
    value: Field.timestamp,
  },
  basicName: {
    name: "Player name",
    value: Field.basicName,
  },
  basicLevel: {
    name: "Level",
    value: Field.basicLevel,
  },
  basicLegendaryLevel: {
    name: "Legendary level",
    value: Field.basicLegendaryLevel,
  },
  basicMight: {
    name: "Might",
    value: Field.basicMight,
  },
  basicHonor: {
    name: "Honor",
    value: Field.basicHonor,
  },
  basicAchievement: {
    name: "Achievement points",
    value: Field.basicAchievement,
  },
  basicGlory: {
    name: "Glory",
    value: Field.basicGlory,
  },
  basicRuins: {
    name: "In ruins",
    value: Field.basicRuins,
  },
  allianceId: {
    name: "Alliance ID",
    value: Field.allianceId,
  },
  allianceName: {
    name: "Alliance name",
    value: Field.allianceName,
  },
  allianceRankId: {
    name: "Alliance rank",
    value: Field.allianceRankId,
  },
  allianceSearching: {
    name: "Searching for alliance",
    value: Field.allianceSearching,
  },
  timerProtectionTime: {
    name: "Protection time",
    value: Field.timerProtectionTime,
  },
  timerRelocateTime: {
    name: "Relocate time",
    value: Field.timerRelocateTime,
  },
  locations: {
    name: "Owned locations",
    value: Field.locations,
  },
};

function updateFields(
  fieldSet: Set<Field>,
  field: Field,
  state: boolean,
): Set<Field> {
  const newSet = new Set(fieldSet);
  if (state) {
    newSet.add(field);
  } else {
    newSet.delete(field);
  }
  return newSet;
}

export default function FieldPicker({
  onPreviewUpdate,
  onDetailUpdate,
}: {
  onPreviewUpdate: (fields: Field[]) => void;
  onDetailUpdate: (fields: Field[]) => void;
}) {
  const [previewFields, setPreviewFields] = useState<Set<Field>>(new Set());
  const [detailFields, setdetailFields] = useState<Set<Field>>(new Set());

  // Sort for consistent ordering
  useEffect(() => {
    onPreviewUpdate([...previewFields].sort((a, b) => a - b));
  }, [previewFields, onPreviewUpdate]);
  useEffect(() => {
    onDetailUpdate([...detailFields].sort((a, b) => a - b));
  }, [detailFields, onDetailUpdate]);

  return (
    <div
      className={styles.border}
      style={{
        display: "flex",
        flexDirection: "column",
        gap: 20,
        alignItems: "center",
        padding: 10,
      }}
    >
      <span style={{ fontSize: 32 }}>Fields</span>
      <div
        style={{
          display: "flex",
          gap: 10,
          flexWrap: "wrap",
        }}
      >
        {Object.entries(fields).map(([key, { name, value }]) => (
          <FieldItem
            key={key}
            name={name}
            previewSelected={previewFields.has(value)}
            onPreviewUpdate={(state) =>
              setPreviewFields((fieldSet) =>
                updateFields(fieldSet, value, state),
              )
            }
            detailSelected={detailFields.has(value)}
            onDetailUpdate={(state) =>
              setdetailFields((fieldSet) =>
                updateFields(fieldSet, value, state),
              )
            }
          />
        ))}
      </div>
    </div>
  );
}
