import { useEffect, useState } from "react";
import type { Option } from "./types";

function BoolInput(onUpdate: (value: 0n | 1n) => void) {
  const [value, setValue] = useState<number>(1); // True

  useEffect(() => {
    if (value === 0) {
      onUpdate(0n);
    } else if (value === 1) {
      onUpdate(1n);
    }
  }, [value]);

  return (
    <select
      value={value}
      onChange={(event) => setValue(Number(event.target.value))}
    >
      <option value={1}>True</option>
      <option value={0}>False</option>
    </select>
  );
}

export function createBoolOptions(prefix: string): Option[] {
  return [
    {
      name: "",
      component: ({ onUpdate }) =>
        BoolInput((value) => onUpdate({ id: `${prefix}_eq`, args: [value] })),
    },
  ];
}
