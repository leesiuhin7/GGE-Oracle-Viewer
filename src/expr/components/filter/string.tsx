import { useEffect, useState } from "react";
import styles from "./filter.module.css";
import type { Option } from "./types";

function StringOnlyInput(onUpdate: (value: string) => void) {
  const [value, setValue] = useState("");

  useEffect(() => {
    onUpdate(value);
  }, [value]);

  return (
    <label className={styles.input}>
      Value
      <input
        value={value}
        onChange={(event) => setValue(event.target.value)}
        className={"validated-input--valid"}
      ></input>
    </label>
  );
}

export function createStringOptions(prefix: string): Option[] {
  return [
    {
      name: "Equal to",
      component: ({ onUpdate }) =>
        StringOnlyInput((value) =>
          onUpdate({ id: `${prefix}_eq`, args: [value] }),
        ),
    },
    {
      name: "Not equal to",
      component: ({ onUpdate }) =>
        StringOnlyInput((value) =>
          onUpdate({ id: `${prefix}_ne`, args: [value] }),
        ),
    },
    {
      name: "Starts with",
      component: ({ onUpdate }) =>
        StringOnlyInput((value) =>
          onUpdate({ id: `${prefix}_prefix`, args: [value] }),
        ),
    },
    {
      name: "Ends with",
      component: ({ onUpdate }) =>
        StringOnlyInput((value) =>
          onUpdate({ id: `${prefix}_suffix`, args: [value] }),
        ),
    },
    {
      name: "Contains",
      component: ({ onUpdate }) =>
        StringOnlyInput((value) =>
          onUpdate({ id: `${prefix}_substring`, args: [value] }),
        ),
    },
  ];
}
