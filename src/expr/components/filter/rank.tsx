import { useEffect, useState } from "react";
import styles from "./filter.module.css";
import type { Option } from "./types";

function RankInput(onUpdate: (value: bigint | null) => void) {
  const [value, setValue] = useState("");

  useEffect(() => {
    if (value === "null") {
      onUpdate(null);
    } else {
      onUpdate(BigInt(value));
    }
  }, [value]);

  return (
    <label className={styles.input}>
      Rank
      <select
        value={value}
        onChange={(event) => {
          setValue(event.target.value);
        }}
      >
        <option value={0}>Leader</option>
        <option value={1}>Deputy</option>
        <option value={2}>War marshal</option>
        <option value={3}>Treasurer</option>
        <option value={4}>Diplomat</option>
        <option value={5}>Recruiter</option>
        <option value={6}>General</option>
        <option value={7}>Sergeant</option>
        <option value={8}>Member</option>
        <option value={"null"}>No rank</option>
      </select>
    </label>
  );
}

export function createRankOptions(prefix: string): Option[] {
  return [
    {
      name: "Has",
      component: ({ onUpdate }) =>
        RankInput((value) => onUpdate({ id: `${prefix}_eq`, args: [value] })),
    },
    {
      name: "Does not have",
      component: ({ onUpdate }) =>
        RankInput((value) => onUpdate({ id: `${prefix}_ne`, args: [value] })),
    },
    {
      name: "Above",
      component: ({ onUpdate }) =>
        RankInput((value) => onUpdate({ id: `${prefix}_gt`, args: [value] })),
    },
    {
      name: "Below",
      component: ({ onUpdate }) =>
        RankInput((value) => onUpdate({ id: `${prefix}_lt`, args: [value] })),
    },
    {
      name: "At or above",
      component: ({ onUpdate }) =>
        RankInput((value) => onUpdate({ id: `${prefix}_ge`, args: [value] })),
    },
    {
      name: "At or below",
      component: ({ onUpdate }) =>
        RankInput((value) => onUpdate({ id: `${prefix}_le`, args: [value] })),
    },
  ];
}
