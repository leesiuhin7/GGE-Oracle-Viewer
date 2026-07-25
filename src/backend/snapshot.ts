import * as backend from "../../pkg/gge_oracle_viewer_wasm";
import { Field } from "./field";

export interface Header {
  id: number;
  server: string;
}

export interface CoatOfArms {
  bgColor1: bigint;
  bgColor2: bigint;
  bgType: bigint;
  symbolColor1: bigint;
  symbolColor2: bigint;
  symbolPosType: bigint;
  symbolType1: bigint;
  symbolType2: bigint;
}

export interface Location {
  id: bigint;
  kingdomId: bigint;
  type: bigint;
  x: bigint;
  y: bigint;
}

export interface Snapshot {
  header?: Header | null;
  timestamp?: bigint | null;
  basicName?: string | null;
  basicLevel?: bigint | null;
  basicLegendaryLevel?: bigint | null;
  basicMight?: bigint | null;
  basicHonor?: bigint | null;
  basicAchievement?: bigint | null;
  basicGlory?: bigint | null;
  basicRuins?: bigint | null;
  allianceId?: bigint | null;
  allianceName?: string | null;
  allianceRankId?: bigint | null;
  allianceSearching?: bigint | null;
  timerProtectionTime?: bigint | null;
  timerRelocateTime?: bigint | null;
  locations?: Location[] | null;
  coatOfArms?: CoatOfArms | null;
  factionId?: bigint | null;
  factionTitleId?: bigint | null;
  factionSelfProtectionTime?: bigint | null;
  factionGroupProtectionStatus?: bigint | null;
  factionGroupProtectionTime?: bigint | null;
  factionMainCampId?: bigint | null;
  factionSpecialCampId?: bigint | null;
}

export default function extractFields(
  snapshot: backend.Snapshot,
  fields: Field[],
): Snapshot {
  const result: Snapshot = {};
  for (const field of fields) {
    switch (field) {
      case Field.header:
        if (snapshot.header.present) {
          if (snapshot.header.value) {
            const { id, server } = snapshot.header.value;
            result.header = { id, server };
          } else {
            result.header = null;
          }
        }
        break;
      case Field.timestamp: {
        const { present, value } = snapshot.timestamp;
        if (present) {
          result.timestamp = value ?? null;
        }
        break;
      }
      case Field.basicName: {
        const { present, value } = snapshot.basic_name;
        if (present) {
          result.basicName = value ?? null;
        }
        break;
      }
      case Field.basicLevel: {
        const { present, value } = snapshot.basic_level;
        if (present) {
          result.basicLevel = value ?? null;
        }
        break;
      }
      case Field.basicLegendaryLevel: {
        const { present, value } = snapshot.basic_legendary_level;
        if (present) {
          result.basicLegendaryLevel = value ?? null;
        }
        break;
      }
      case Field.basicMight: {
        const { present, value } = snapshot.basic_might;
        if (present) {
          result.basicMight = value ?? null;
        }
        break;
      }
      case Field.basicHonor: {
        const { present, value } = snapshot.basic_honor;
        if (present) {
          result.basicHonor = value ?? null;
        }
        break;
      }
      case Field.basicAchievement: {
        const { present, value } = snapshot.basic_achievement;
        if (present) {
          result.basicAchievement = value ?? null;
        }
        break;
      }
      case Field.basicGlory: {
        const { present, value } = snapshot.basic_glory;
        if (present) {
          result.basicGlory = value ?? null;
        }
        break;
      }
      case Field.basicRuins: {
        const { present, value } = snapshot.basic_ruins;
        if (present) {
          result.basicRuins = value ?? null;
        }
        break;
      }
      case Field.allianceId: {
        const { present, value } = snapshot.alliance_id;
        if (present) {
          result.allianceId = value ?? null;
        }
        break;
      }
      case Field.allianceName: {
        const { present, value } = snapshot.alliance_name;
        if (present) {
          result.allianceName = value ?? null;
        }
        break;
      }
      case Field.allianceRankId: {
        const { present, value } = snapshot.alliance_rank_id;
        if (present) {
          result.allianceRankId = value ?? null;
        }
        break;
      }
      case Field.allianceSearching: {
        const { present, value } = snapshot.alliance_searching;
        if (present) {
          result.allianceSearching = value ?? null;
        }
        break;
      }
      case Field.timerProtectionTime: {
        const { present, value } = snapshot.timer_protection_time;
        if (present) {
          result.timerProtectionTime = value ?? null;
        }
        break;
      }
      case Field.timerRelocateTime: {
        const { present, value } = snapshot.timer_relocate_time;
        if (present) {
          result.timerRelocateTime = value ?? null;
        }
        break;
      }
      case Field.locations: {
        const { present, value } = snapshot.locations;
        if (present) {
          result.locations =
            value?.map(
              ({ id, kingdom_id: kingdomId, location_type: type, x, y }) => ({
                id,
                kingdomId,
                type,
                x,
                y,
              }),
            ) ?? null;
        }
        break;
      }
      case Field.coatOfArms: {
        const { present, value } = snapshot.coat_of_arms;
        if (present) {
          if (value) {
            const {
              bg_color1: bgColor1,
              bg_color2: bgColor2,
              bg_type: bgType,
              symbol_color1: symbolColor1,
              symbol_color2: symbolColor2,
              symbol_pos_type: symbolPosType,
              symbol_type1: symbolType1,
              symbol_type2: symbolType2,
            } = value;
            result.coatOfArms = {
              bgColor1,
              bgColor2,
              bgType,
              symbolColor1,
              symbolColor2,
              symbolPosType,
              symbolType1,
              symbolType2,
            };
          } else {
            result.coatOfArms = null;
          }
        }
        break;
      }
      case Field.factionId: {
        const { present, value } = snapshot.faction_id;
        if (present) {
          result.factionId = value ?? null;
        }
        break;
      }
      case Field.factionTitleId: {
        const { present, value } = snapshot.faction_title_id;
        if (present) {
          result.factionTitleId = value ?? null;
        }
        break;
      }
      case Field.factionSelfProtectionTime: {
        const { present, value } = snapshot.faction_self_protection_time;
        if (present) {
          result.factionSelfProtectionTime = value ?? null;
        }
        break;
      }
      case Field.factionGroupProtectionStatus: {
        const { present, value } = snapshot.faction_group_protection_status;
        if (present) {
          result.factionGroupProtectionStatus = value ?? null;
        }
        break;
      }
      case Field.factionGroupProtectionTime: {
        const { present, value } = snapshot.faction_group_protection_time;
        if (present) {
          result.factionGroupProtectionTime = value ?? null;
        }
        break;
      }
      case Field.factionMainCampId: {
        const { present, value } = snapshot.faction_main_camp_id;
        if (present) {
          result.factionMainCampId = value ?? null;
        }
        break;
      }
      case Field.factionSpecialCampId: {
        const { present, value } = snapshot.faction_special_camp_id;
        if (present) {
          result.factionSpecialCampId = value ?? null;
        }
        break;
      }
    }
  }
  return result;
}
