import type * as backend from "../../pkg/gge_oracle_viewer_wasm";

type _Field = {
  header: 0;
  timestamp: 1;
  basicName: 2;
  basicLevel: 3;
  basicLegendaryLevel: 4;
  basicMight: 5;
  basicHonor: 6;
  basicAchievement: 7;
  basicGlory: 8;
  basicRuins: 9;
  allianceId: 10;
  allianceName: 11;
  allianceRankId: 12;
  allianceSearching: 13;
  timerProtectionTime: 14;
  timerRelocateTime: 15;
  locations: 16;
  coatOfArms: 17;
  factionId: 18;
  factionTitleId: 19;
  factionSelfProtectionTime: 20;
  factionGroupProtectionStatus: 21;
  factionGroupProtectionTime: 22;
  factionMainCampId: 23;
  factionSpecialCampId: 24;
};

export type Field = backend.WrapperField;

export const Field: Readonly<Record<keyof _Field, Field>> = {
  header: 0,
  timestamp: 1,
  basicName: 2,
  basicLevel: 3,
  basicLegendaryLevel: 4,
  basicMight: 5,
  basicHonor: 6,
  basicAchievement: 7,
  basicGlory: 8,
  basicRuins: 9,
  allianceId: 10,
  allianceName: 11,
  allianceRankId: 12,
  allianceSearching: 13,
  timerProtectionTime: 14,
  timerRelocateTime: 15,
  locations: 16,
  coatOfArms: 17,
  factionId: 18,
  factionTitleId: 19,
  factionSelfProtectionTime: 20,
  factionGroupProtectionStatus: 21,
  factionGroupProtectionTime: 22,
  factionMainCampId: 23,
  factionSpecialCampId: 24,
};

export function toFields(fields: Field[]): backend.WrapperField[] {
  return fields;
}
