import { useEffect, useState, type ComponentType } from "react";
import { createBoolOptions } from "./bool";
import { createDurationOptions } from "./duration";
import styles from "./filter.module.css";
import { createHeaderIdOptions, createHeaderServerOptions } from "./header";
import { createNumericOptions } from "./numeric";
import { createRankOptions } from "./rank";
import { createStringOptions } from "./string";
import { createTimestampOptions } from "./timestamp";
import type { Option, UpdateProps } from "./types";

interface FilterType {
  name: string;
  options: Option[];
}

const filterTypes: FilterType[] = [
  {
    name: "Player ID",
    options: createHeaderIdOptions("header_id"),
  },
  {
    name: "Server",
    options: createHeaderServerOptions("header_server"),
  },
  {
    name: "Time",
    options: createTimestampOptions("timestamp"),
  },
  {
    name: "Player Name",
    options: createStringOptions("basic_name"),
  },
  {
    name: "Level",
    options: createNumericOptions("basic_level"),
  },
  {
    name: "Legendary Level",
    options: createNumericOptions("basic_legendary_level"),
  },
  {
    name: "Might",
    options: createNumericOptions("basic_might"),
  },
  {
    name: "Honor",
    options: createNumericOptions("basic_honor"),
  },
  {
    name: "Achievement Points",
    options: createNumericOptions("basic_achievement"),
  },
  {
    name: "Glory",
    options: createNumericOptions("basic_glory"),
  },
  {
    name: "In Ruins",
    options: createBoolOptions("basic_ruins"),
  },
  {
    name: "Alliance ID",
    options: createNumericOptions("alliance_id"),
  },
  {
    name: "Alliance Name",
    options: createStringOptions("alliance_name"),
  },
  {
    name: "Alliance Rank",
    options: createRankOptions("alliance_rank_id"),
  },
  {
    name: "Searching For Alliance",
    options: createBoolOptions("alliance_searching"),
  },
  {
    name: "Protection Time",
    options: createDurationOptions("timer_protection_time"),
  },
  {
    name: "Relocate Time",
    options: createDurationOptions("timer_relocate_time"),
  },
];

export default function Filter({ onUpdate, onError }: UpdateProps) {
  const [type, setType] = useState(0);
  const [option, setOption] = useState(0);
  const [options, setOptions] = useState<Option[]>([]);
  const [Component, setComponent] = useState<ComponentType<UpdateProps>>(
    () => () => <></>,
  );

  useEffect(() => {
    // It shouldn't be possible for type to be out of range
    setOptions(filterTypes.at(type)!.options);
  }, [type]);

  useEffect(() => {
    const filterOption = options.at(option);
    setComponent(() => (filterOption ? filterOption.component : () => <></>));
  }, [options, option]);

  return (
    <div
      className={styles.border}
      style={{
        padding: 10,
      }}
    >
      <select
        value={type}
        onChange={(event) => setType(Number(event.target.value))}
      >
        {filterTypes.map(({ name }, index) => (
          <option value={index} key={index}>
            {name}
          </option>
        ))}
      </select>
      <select
        value={option}
        onChange={(event) => setOption(Number(event.target.value))}
      >
        {options.map(({ name }, index) => (
          <option value={index} key={index}>
            {name}
          </option>
        ))}
      </select>
      <div style={{ marginTop: 5 }}>
        <Component onUpdate={onUpdate} onError={() => onError()} />
      </div>
    </div>
  );
}
