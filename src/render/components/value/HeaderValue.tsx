import type { Header } from "../../../backend";
import { UnknownValue } from "./base";
import style from "./value.module.css";

const serverNameMap: Readonly<Record<string, string>> = {
  EmpireEx: "International 1",
  EmpireEx_2: "Germany 1",
  EmpireEx_3: "France 1",
  EmpireEx_4: "Czech Republic 1",
  EmpireEx_5: "Poland 1",
  EmpireEx_6: "Portuguese 1",
  EmpireEx_7: "International 2",
  EmpireEx_8: "Spain 1",
  EmpireEx_9: "Italy 1",
  EmpireEx_10: "Turkey 1",
  EmpireEx_11: "Netherlands 1",
  EmpireEx_12: "Hungary 1",
  EmpireEx_13: "Nordic 1",
  EmpireEx_14: "Russia 1",
  EmpireEx_15: "Romania 1",
  EmpireEx_16: "Bulgaria 1",
  EmpireEx_17: "Hungary 2",
  EmpireEx_18: "Slovakia 1",
  EmpireEx_19: "United Kingdom 1",
  EmpireEx_20: "Brazil 1",
  EmpireEx_21: "United States 1",
  EmpireEx_22: "Australia 1",
  EmpireEx_24: "Japan 1",
  EmpireEx_25: "Hispanic America 1",
  EmpireEx_26: "India 1",
  EmpireEx_27: "China 1",
  EmpireEx_28: "Greece 1",
  EmpireEx_29: "Lithuania 1",
  EmpireEx_32: "Saudi Arabia 1",
  EmpireEx_33: "United Arab Emirates 1",
  EmpireEx_34: "Egypt 1",
  EmpireEx_35: "Arab League 1",
  EmpireEx_36: "Asia 1",
  EmpireEx_37: "Chinese (traditional) 1",
  EmpireEx_38: "Spain 2",
  EmpireEx_43: "International 3",
  EmpireEx_46: "World 1",
  EmpireEx_49: "World 2",
};

export default function HeaderValue({ value }: { value: Header | null }) {
  if (value === null) {
    return <UnknownValue />;
  }
  const { id: playerId, server: serverId } = value;
  const serverName = serverNameMap[serverId];

  return (
    <span style={{ display: "flex", gap: 10 }}>
      <label className={style.label}>
        Player ID
        <span className={style.border}>{playerId.toString()}</span>
      </label>
      <label className={style.label}>
        Server
        <span className={style.border}>
          {serverName === undefined ?
            <UnknownValue />
          : serverName}
        </span>
      </label>
    </span>
  );
}
