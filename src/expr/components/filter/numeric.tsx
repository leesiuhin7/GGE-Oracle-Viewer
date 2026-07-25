import { useEffect, useState } from "react";
import styles from "./filter.module.css";
import type { Option } from "./types";

function NumericOnlyInput(
  onUpdate: (value: bigint) => void,
  onError: () => void,
) {
  const [value, setValue] = useState("");
  const [error, setError] = useState(false);

  useEffect(() => {
    try {
      const num = BigInt(value);
      setError(false);
      onUpdate(num);
    } catch {
      setError(true);
      onError();
    }
  }, [value]);

  return (
    <label className={styles.input}>
      Value
      <input
        value={value}
        onChange={(event) => setValue(event.target.value)}
        placeholder="0"
        className={styles[`validated-input--${error ? "error" : "valid"}`]}
      ></input>
    </label>
  );
}

function NumericRangeInput(
  onUpdate: (lower: bigint, upper: bigint) => void,
  onError: () => void,
) {
  const [lowerValue, setLowerValue] = useState("");
  const [upperValue, setUpperValue] = useState("");

  const [error, setError] = useState(false);

  useEffect(() => {
    try {
      const lower = BigInt(lowerValue);
      const upper = BigInt(upperValue) + 1n; // Add 1 to make range inclusive
      if (lower >= upper) {
        throw new Error();
      }
      setError(false);
      onUpdate(lower, upper);
    } catch {
      setError(true);
      onError();
    }
  }, [lowerValue, upperValue]);

  return (
    <label className={styles.input}>
      Range
      <input
        value={lowerValue}
        onChange={(event) => setLowerValue(event.target.value)}
        placeholder="0"
        className={styles[`validated-input--${error ? "error" : "valid"}`]}
      ></input>
      -
      <input
        value={upperValue}
        onChange={(event) => setUpperValue(event.target.value)}
        placeholder="0"
        className={styles[`validated-input--${error ? "error" : "valid"}`]}
      ></input>
    </label>
  );
}

function NumericEqualityInput(
  onUpdate: (value: bigint | null) => void,
  onError: () => void,
) {
  const [value, setValue] = useState("");
  const [error, setError] = useState(false);

  useEffect(() => {
    try {
      const num = value === "" ? null : BigInt(value);
      setError(false);
      onUpdate(num);
    } catch {
      setError(true);
      onError();
    }
  }, [value]);

  return (
    <label className={styles.input}>
      Value
      <input
        value={value}
        onChange={(event) => setValue(event.target.value)}
        placeholder="null"
        className={styles[`validated-input--${error ? "error" : "valid"}`]}
      ></input>
    </label>
  );
}

export function createNumericOptions(prefix: string): Option[] {
  return [
    {
      name: "Equal to",
      component: ({ onUpdate, onError }) =>
        NumericEqualityInput(
          (value) => onUpdate({ id: `${prefix}_eq`, args: [value] }),
          onError,
        ),
    },
    {
      name: "Not equal to",
      component: ({ onUpdate, onError }) =>
        NumericEqualityInput(
          (value) => onUpdate({ id: `${prefix}_ne`, args: [value] }),
          onError,
        ),
    },
    {
      name: "Less than",
      component: ({ onUpdate, onError }) =>
        NumericOnlyInput(
          (value) => onUpdate({ id: `${prefix}_lt`, args: [value] }),
          onError,
        ),
    },
    {
      name: "Less than or equal to",
      component: ({ onUpdate, onError }) =>
        NumericOnlyInput(
          (value) => onUpdate({ id: `${prefix}_le`, args: [value] }),
          onError,
        ),
    },
    {
      name: "More than",
      component: ({ onUpdate, onError }) =>
        NumericOnlyInput(
          (value) => onUpdate({ id: `${prefix}_gt`, args: [value] }),
          onError,
        ),
    },
    {
      name: "More than or equal to",
      component: ({ onUpdate, onError }) =>
        NumericOnlyInput(
          (value) => onUpdate({ id: `${prefix}_ge`, args: [value] }),
          onError,
        ),
    },
    {
      name: "Within range",
      component: ({ onUpdate, onError }) =>
        NumericRangeInput(
          (lower, upper) =>
            onUpdate({ id: `${prefix}_range`, args: [lower, upper] }),
          onError,
        ),
    },
  ];
}
