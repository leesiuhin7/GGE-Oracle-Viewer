import type { Location } from "../../../backend";
import { UnknownValue } from "./base";
import style from "./value.module.css";

const kingdomIdMap: Readonly<Record<number, React.ComponentType>> = {
  0: () => <span style={{ color: "rgb(87, 142, 0)" }}>The Great Empire</span>,
  1: () => (
    <span style={{ color: "rgb(231, 159, 32)" }}>The Burning Sands</span>
  ),
  2: () => (
    <span style={{ color: "rgb(97, 157, 190)" }}>The Everwinter Glacier</span>
  ),
  3: () => <span style={{ color: "rgb(75, 47, 39)" }}>The Fire Peaks</span>,
  4: () => <span style={{ color: "rgb(0, 233, 162)" }}>The Storm Islands</span>,
};

const locationTypeMap: Readonly<Record<number, string>> = {
  1: "Main castle",
  3: "Capital",
  4: "Outpost",
  12: "Kingdom castle",
  22: "Trading metropolis",
  23: "Royal tower",
  24: "Resource island",
  26: "Monument",
  28: "Laboratory",
};

function LocationDisplay({ location }: { location: Location }) {
  const { id, kingdomId, type, x, y } = location;
  const Kingdom = kingdomIdMap[Number(kingdomId)] ?? UnknownValue;

  return (
    <span style={{ display: "flex", gap: 10 }}>
      <label className={style.label}>
        ID
        <span className={style.border}>{id}</span>
      </label>
      <label className={style.label}>
        Description
        <span className={style.border}>
          {locationTypeMap[Number(type)] ?? <UnknownValue />} at {x}:{y} (
          <Kingdom />)
        </span>
      </label>
    </span>
  );
}

export default function LocationsValue({
  value,
}: {
  value: Location[] | null;
}) {
  if (value === null) {
    return <UnknownValue />;
  }
  return (
    <span
      style={{ display: "flex", flexDirection: "column", gap: 5, margin: 10 }}
    >
      {value.map((location) => (
        <LocationDisplay key={location.id} location={location} />
      ))}
    </span>
  );
}
