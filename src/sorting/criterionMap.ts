interface Options {
  ascName: string;
  descName: string;
}

const criterionMap: Record<
  string,
  {
    name: string;
    options: Options;
  }
> = {
  header_id: {
    name: "Player ID",
    options: { ascName: "Lowest first", descName: "Highest first" },
  },
  timestamp: {
    name: "Time",
    options: { ascName: "Oldest first", descName: "Latest first" },
  },
  basic_name: {
    name: "Player Name",
    options: { ascName: "A-Z", descName: "Z-A" },
  },
  basic_level: {
    name: "Level",
    options: { ascName: "Lowest first", descName: "Highest first" },
  },
  basic_legendary_level: {
    name: "Legendary Level",
    options: { ascName: "Lowest first", descName: "Highest first" },
  },
  basic_might: {
    name: "Might",
    options: { ascName: "Lowest first", descName: "Highest first" },
  },
  basic_honor: {
    name: "Honor",
    options: { ascName: "Lowest first", descName: "Highest first" },
  },
  basic_achievement: {
    name: "Achievement Points",
    options: { ascName: "Lowest first", descName: "Highest first" },
  },
  basic_glory: {
    name: "Glory",
    options: { ascName: "Lowest first", descName: "Highest first" },
  },
  alliance_id: {
    name: "Alliance ID",
    options: { ascName: "Lowest first", descName: "Highest first" },
  },
  alliance_name: {
    name: "Alliance Name",
    options: { ascName: "A-Z", descName: "Z-A" },
  },
  alliance_rank_id: {
    name: "Alliance Rank",
    options: { ascName: "Highest first", descName: "Lowest first" },
  },
  timer_protection_time: {
    name: "Protection Time",
    options: { ascName: "Shortest first", descName: "Longest first" },
  },
  timer_relocate_time: {
    name: "Relocate Time",
    options: { ascName: "Shortest first", descName: "Longest first" },
  },
};

export default criterionMap;
export type { Options };
