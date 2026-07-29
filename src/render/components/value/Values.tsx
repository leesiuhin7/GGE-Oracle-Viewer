import type { Snapshot } from "../../../backend";
import {
  BoolValue,
  NotSupported,
  NumericValue,
  StringValue,
  TimestampValue,
} from "./basic";
import DurationValue from "./DurationValue";
import HeaderValue from "./HeaderValue";
import LocationsValue from "./LocationsValue";
import RankValue from "./RankValue";
import style from "./value.module.css";

type ValueDisplay<Field extends keyof Snapshot> = React.ComponentType<{
  value: Exclude<Snapshot[Field], undefined>;
}>;

const componentMap: Readonly<{
  [Field in keyof Snapshot]-?: {
    name: string;
    component: ValueDisplay<Field>;
  };
}> = {
  header: {
    name: "Identity",
    component: HeaderValue,
  },
  timestamp: {
    name: "Time",
    component: TimestampValue,
  },
  basicName: {
    name: "Player name",
    component: StringValue,
  },
  basicLevel: {
    name: "Level",
    component: NumericValue,
  },
  basicLegendaryLevel: {
    name: "Legendary level",
    component: NumericValue,
  },
  basicMight: {
    name: "Might",
    component: NumericValue,
  },
  basicHonor: {
    name: "Honor",
    component: NumericValue,
  },
  basicAchievement: {
    name: "Achievement points",
    component: NumericValue,
  },
  basicGlory: {
    name: "Glory",
    component: NumericValue,
  },
  basicRuins: {
    name: "In ruins",
    component: BoolValue,
  },
  allianceId: {
    name: "Alliance ID",
    component: NumericValue,
  },
  allianceName: {
    name: "Alliance name",
    component: StringValue,
  },
  allianceRankId: {
    name: "Alliance rank",
    component: RankValue,
  },
  allianceSearching: {
    name: "Searching for alliance",
    component: BoolValue,
  },
  timerProtectionTime: {
    name: "Protection time",
    component: DurationValue,
  },
  timerRelocateTime: {
    name: "Relocate time",
    component: DurationValue,
  },
  locations: {
    name: "Owned locations",
    component: LocationsValue,
  },
  coatOfArms: {
    name: "coatOfArms",
    component: NotSupported,
  },
  factionId: {
    name: "factionId",
    component: NumericValue,
  },
  factionTitleId: {
    name: "factionTitleId",
    component: NumericValue,
  },
  factionSelfProtectionTime: {
    name: "factionSelfProtectionTime",
    component: NumericValue,
  },
  factionGroupProtectionStatus: {
    name: "factionGroupProtectionStatus",
    component: NumericValue,
  },
  factionGroupProtectionTime: {
    name: "factionGroupProtectionTime",
    component: NumericValue,
  },
  factionMainCampId: {
    name: "factionMainCampId",
    component: NumericValue,
  },
  factionSpecialCampId: {
    name: "factionSpecialCampId",
    component: NumericValue,
  },
};

type SnapshotEntries = Array<
  {
    [Field in keyof Snapshot]-?: [Field, Snapshot[Field]];
  }[keyof Snapshot]
>;

export default function Values({ snapshot }: { snapshot: Snapshot }) {
  return (
    <>
      {(Object.entries(snapshot) as SnapshotEntries)
        .filter(([, value]) => value !== undefined)
        .map(([field, value]) => {
          const { name, component: Component } = componentMap[field];
          return (
            <span
              key={field}
              style={{
                display: "flex",
                flexDirection: "column",
                gap: 5,
                alignItems: "center",
              }}
            >
              {name}
              <span className={style.border}>
                {/* @ts-expect-error Type of value is not narrow enough*/}
                <Component value={value} />
              </span>
            </span>
          );
        })}
    </>
  );
}
