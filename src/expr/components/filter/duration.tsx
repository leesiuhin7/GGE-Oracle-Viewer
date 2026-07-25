import { createContext, useContext, useEffect, useState } from "react";
import styles from "./filter.module.css";
import type { Option } from "./types";

const ErrorContext = createContext(false);

function DurationUnitInput({
  max,
  width,
  name,
  onUpdate,
}: {
  max?: number;
  width: number;
  name: string;
  onUpdate: (value: number) => void;
}) {
  const [value, setValue] = useState<number>(0);
  const error = useContext(ErrorContext);

  useEffect(() => {
    onUpdate(value);
  }, [value]);

  return (
    <label style={{ display: "flex", gap: 5 }}>
      <input
        type="number"
        min={0}
        max={max}
        step={1}
        inputMode="numeric"
        style={{ width: `${width}em` }}
        className={styles[`validated-input--${error ? "error" : "valid"}`]}
        value={value}
        onChange={(event) => {
          setValue(Number(event.target.value));
        }}
      ></input>
      {name}
    </label>
  );
}

function BaseDurationInput({
  onUpdate,
}: {
  onUpdate: (value: bigint) => void;
}) {
  const [durationDays, setDurationDays] = useState<number>(0);
  const [durationHours, setDurationHours] = useState<number>(0);
  const [durationMinutes, setDurationMinutes] = useState<number>(0);
  const [durationSeconds, setDurationSeconds] = useState<number>(0);

  useEffect(() => {
    const duration =
      durationDays * 86400 +
      durationHours * 3600 +
      durationMinutes * 60 +
      durationSeconds;
    onUpdate(BigInt(duration));
  }, [durationDays, durationHours, durationMinutes, durationSeconds]);

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 10 }}>
      <DurationUnitInput
        width={4}
        name="Days"
        onUpdate={(value) => {
          setDurationDays(value);
        }}
      />
      <DurationUnitInput
        max={23}
        width={3}
        name="Hours"
        onUpdate={(value) => {
          setDurationHours(value);
        }}
      />
      <DurationUnitInput
        max={59}
        width={3}
        name="Minutes"
        onUpdate={(value) => {
          setDurationMinutes(value);
        }}
      />
      <DurationUnitInput
        max={59}
        width={3}
        name="Seconds"
        onUpdate={(value) => {
          setDurationSeconds(value);
        }}
      />
    </div>
  );
}

function DurationInput(onUpdate: (value: bigint) => void) {
  return (
    <div style={{ display: "flex", gap: 20 }}>
      Duration
      <BaseDurationInput onUpdate={onUpdate} />
    </div>
  );
}

function DurationRangeInput(
  onUpdate: (lower: bigint, upper: bigint) => void,
  onError: () => void,
) {
  const [lowerValue, setLowerValue] = useState(0n);
  const [upperValue, setUpperValue] = useState(0n);
  const [error, setError] = useState(false);

  useEffect(() => {
    const upper = upperValue + 1n; // Add 1 to make range inclusive
    if (lowerValue < upper) {
      setError(false);
      onUpdate(lowerValue, upper);
    } else {
      setError(true);
      onError();
    }
  }, [lowerValue, upperValue]);

  return (
    <ErrorContext value={error}>
      <div style={{ display: "flex", gap: 20, alignItems: "center" }}>
        Duration range
        <BaseDurationInput
          onUpdate={(value) => {
            setLowerValue(value);
          }}
        />
        -
        <BaseDurationInput
          onUpdate={(value) => {
            setUpperValue(value);
          }}
        />
      </div>
    </ErrorContext>
  );
}

export function createDurationOptions(prefix: string): Option[] {
  return [
    {
      name: "Equal to",
      component: ({ onUpdate }) =>
        DurationInput((value) =>
          onUpdate({ id: `${prefix}_eq`, args: [value] }),
        ),
    },
    {
      name: "Not equal to",
      component: ({ onUpdate }) =>
        DurationInput((value) =>
          onUpdate({ id: `${prefix}_ne`, args: [value] }),
        ),
    },
    {
      name: "Less than",
      component: ({ onUpdate }) =>
        DurationInput((value) =>
          onUpdate({ id: `${prefix}_lt`, args: [value] }),
        ),
    },
    {
      name: "Less than or equal to",
      component: ({ onUpdate }) =>
        DurationInput((value) =>
          onUpdate({ id: `${prefix}_le`, args: [value] }),
        ),
    },
    {
      name: "More than",
      component: ({ onUpdate }) =>
        DurationInput((value) =>
          onUpdate({ id: `${prefix}_gt`, args: [value] }),
        ),
    },
    {
      name: "More than or equal to",
      component: ({ onUpdate }) =>
        DurationInput((value) =>
          onUpdate({ id: `${prefix}_ge`, args: [value] }),
        ),
    },
    {
      name: "Within range",
      component: ({ onUpdate, onError }) =>
        DurationRangeInput(
          (lower, upper) =>
            onUpdate({ id: `${prefix}_range`, args: [lower, upper] }),
          onError,
        ),
    },
  ];
}
