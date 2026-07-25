import { useEffect, useState } from "react";
import styles from "./filter.module.css";
import type { Option } from "./types";

function ServerInput(onUpdate: (value: string) => void) {
  const [value, setValue] = useState("EmpireEx");
  useEffect(() => {
    onUpdate(value);
  }, [value]);

  return (
    <label style={{ display: "flex", gap: 5 }}>
      Server
      <select value={value} onChange={(event) => setValue(event.target.value)}>
        <option value="EmpireEx">International 1</option>
        <option value="EmpireEx_2">Germany 1</option>
        <option value="EmpireEx_3">France 1</option>
        <option value="EmpireEx_4">Czech Republic 1</option>
        <option value="EmpireEx_5">Poland 1</option>
        <option value="EmpireEx_6">Portuguese 1</option>
        <option value="EmpireEx_7">International 2</option>
        <option value="EmpireEx_8">Spain 1</option>
        <option value="EmpireEx_9">Italy 1</option>
        <option value="EmpireEx_10">Turkey 1</option>
        <option value="EmpireEx_11">Netherlands 1</option>
        <option value="EmpireEx_12">Hungary 1</option>
        <option value="EmpireEx_13">Nordic 1</option>
        <option value="EmpireEx_14">Russia 1</option>
        <option value="EmpireEx_15">Romania 1</option>
        <option value="EmpireEx_16">Bulgaria 1</option>
        <option value="EmpireEx_17">Hungary 2</option>
        <option value="EmpireEx_18">Slovakia 1</option>
        <option value="EmpireEx_19">United Kingdom 1</option>
        <option value="EmpireEx_20">Brazil 1</option>
        <option value="EmpireEx_21">United States 1</option>
        <option value="EmpireEx_22">Australia 1</option>
        <option value="EmpireEx_24">Japan 1</option>
        <option value="EmpireEx_25">Hispanic America 1</option>
        <option value="EmpireEx_26">India 1</option>
        <option value="EmpireEx_27">China 1</option>
        <option value="EmpireEx_28">Greece 1</option>
        <option value="EmpireEx_29">Lithuania 1</option>
        <option value="EmpireEx_32">Saudi Arabia 1</option>
        <option value="EmpireEx_33">United Arab Emirates 1</option>
        <option value="EmpireEx_34">Egypt 1</option>
        <option value="EmpireEx_35">Arab League 1</option>
        <option value="EmpireEx_36">Asia 1</option>
        <option value="EmpireEx_37">Chinese (traditional) 1</option>
        <option value="EmpireEx_38">Spain 2</option>
        <option value="EmpireEx_43">International 3</option>
        <option value="EmpireEx_46">World 1</option>
        <option value="EmpireEx_49">World 2</option>
      </select>
    </label>
  );
}

export function createHeaderServerOptions(prefix: string): Option[] {
  return [
    {
      name: "Equal to",
      component: ({ onUpdate }) =>
        ServerInput((value) => onUpdate({ id: `${prefix}_eq`, args: [value] })),
    },
    {
      name: "Not equal to",
      component: ({ onUpdate }) =>
        ServerInput((value) => onUpdate({ id: `${prefix}_ne`, args: [value] })),
    },
  ];
}

function IdInput(onUpdate: (value: number) => void, onError: () => void) {
  const [value, setValue] = useState("");
  const [error, setError] = useState(false);

  useEffect(() => {
    const num = Number(value);
    if (Number.isInteger(num)) {
      onUpdate(num);
      setError(false);
    } else {
      onError();
      setError(true);
    }
  }, [value]);

  return (
    <label style={{ display: "flex", gap: 5 }}>
      Player ID
      <input
        value={value}
        onChange={(event) => setValue(event.target.value)}
        placeholder="0"
        className={styles[`validated-input--${error ? "error" : "valid"}`]}
      ></input>
    </label>
  );
}

export function createHeaderIdOptions(prefix: string): Option[] {
  return [
    {
      name: "Equal to",
      component: ({ onUpdate, onError }) =>
        IdInput(
          (value) => onUpdate({ id: `${prefix}_eq`, args: [value] }),
          onError,
        ),
    },
    {
      name: "Not equal to",
      component: ({ onUpdate, onError }) =>
        IdInput(
          (value) => onUpdate({ id: `${prefix}_ne`, args: [value] }),
          onError,
        ),
    },
    {
      name: "Less than",
      component: ({ onUpdate, onError }) =>
        IdInput(
          (value) => onUpdate({ id: `${prefix}_lt`, args: [value] }),
          onError,
        ),
    },
    {
      name: "Less than or equal to",
      component: ({ onUpdate, onError }) =>
        IdInput(
          (value) => onUpdate({ id: `${prefix}_le`, args: [value] }),
          onError,
        ),
    },
    {
      name: "More than",
      component: ({ onUpdate, onError }) =>
        IdInput(
          (value) => onUpdate({ id: `${prefix}_gt`, args: [value] }),
          onError,
        ),
    },
    {
      name: "More than or equal to",
      component: ({ onUpdate, onError }) =>
        IdInput(
          (value) => onUpdate({ id: `${prefix}_ge`, args: [value] }),
          onError,
        ),
    },
  ];
}
