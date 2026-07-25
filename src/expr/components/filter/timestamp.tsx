import { useEffect, useState } from "react";
import styles from "./filter.module.css";
import type { Option } from "./types";

function BaseTimestampInput({
  onUpdate,
  onError,
  forceError,
}: {
  onUpdate: (value: bigint) => void;
  onError: () => void;
  forceError?: boolean;
}) {
  const [value, setValue] = useState("");
  const [error, setError] = useState(false);

  useEffect(() => {
    const msTimestamp = new Date(value).valueOf();
    try {
      const timestamp = BigInt(Math.round(msTimestamp / 1000)); // Convert to seconds
      onUpdate(timestamp);
      setError(false);
    } catch {
      onError();
      setError(true);
    }
  }, [value]);

  return (
    <input
      type="datetime-local"
      step={1}
      className={
        styles[`validated-input--${error || forceError ? "error" : "valid"}`]
      }
      value={value}
      onChange={(event) => setValue(event.target.value)}
    ></input>
  );
}

function TimestampInput(
  onUpdate: (value: bigint) => void,
  onError: () => void,
) {
  return <BaseTimestampInput onUpdate={onUpdate} onError={onError} />;
}

function TimestampRangeInput(
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
    <div style={{ display: "flex", gap: 20 }}>
      <BaseTimestampInput
        onUpdate={(value) => {
          setLowerValue(value);
        }}
        onError={onError}
        forceError={error}
      />
      -
      <BaseTimestampInput
        onUpdate={(value) => {
          setUpperValue(value);
        }}
        onError={onError}
        forceError={error}
      />
    </div>
  );
}

export function createTimestampOptions(prefix: string): Option[] {
  return [
    {
      name: "On",
      component: ({ onUpdate, onError }) =>
        TimestampInput(
          (value) => onUpdate({ id: `${prefix}_eq`, args: [value] }),
          onError,
        ),
    },
    {
      name: "Not on",
      component: ({ onUpdate, onError }) =>
        TimestampInput(
          (value) => onUpdate({ id: `${prefix}_ne`, args: [value] }),
          onError,
        ),
    },
    {
      name: "Before",
      component: ({ onUpdate, onError }) =>
        TimestampInput(
          (value) => onUpdate({ id: `${prefix}_lt`, args: [value] }),
          onError,
        ),
    },
    {
      name: "On or before",
      component: ({ onUpdate, onError }) =>
        TimestampInput(
          (value) => onUpdate({ id: `${prefix}_le`, args: [value] }),
          onError,
        ),
    },
    {
      name: "After",
      component: ({ onUpdate, onError }) =>
        TimestampInput(
          (value) => onUpdate({ id: `${prefix}_gt`, args: [value] }),
          onError,
        ),
    },
    {
      name: "On or after",
      component: ({ onUpdate, onError }) =>
        TimestampInput(
          (value) => onUpdate({ id: `${prefix}_ge`, args: [value] }),
          onError,
        ),
    },
    {
      name: "Between",
      component: ({ onUpdate, onError }) =>
        TimestampRangeInput(
          (lower, upper) =>
            onUpdate({ id: `${prefix}_range`, args: [lower, upper] }),
          onError,
        ),
    },
  ];
}
