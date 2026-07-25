import * as backend from "../../pkg/gge_oracle_viewer_wasm";

export interface SortingCriterion {
  field: string;
  desc?: boolean;
}

export function toComparators(
  criteria: SortingCriterion[],
): backend.Comparator[] | undefined {
  const comparators: backend.Comparator[] = [];
  for (const criterion of criteria) {
    try {
      const name = `${criterion.field}_${criterion.desc ? "desc" : "asc"}`;
      // @ts-expect-error Indexing using the function name
      const comparator = backend.Comparator[name]();
      comparators.push(comparator);
    } catch {
      return undefined;
    }
  }
  return comparators;
}
