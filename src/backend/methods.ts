import downloadData from "./data";
import { resolveExpr, type ExprTree } from "./expr";
import { toFields, type Field } from "./field";
import type { ResultSnapshots } from "./result";
import { toComparators, type SortingCriterion } from "./sorting";
import storage from "./storage";

export function matchAll(exprTree: ExprTree) {
  const expr = resolveExpr(exprTree);
  if (expr) {
    storage.result.matchAll(expr);
  }
}

export function sort(criteria: SortingCriterion[]): boolean {
  const comparators = toComparators(criteria);
  if (comparators) {
    return storage.result.sort(comparators);
  }
  return false;
}

export function readResult(
  skip: number,
  take: number,
  fields: Field[],
): ResultSnapshots {
  return storage.result.readResult(skip, take, toFields(fields));
}

export async function loadData(): Promise<boolean> {
  const success = await downloadData(storage.files.dataFile);
  if (!success) {
    return false;
  }
  return storage.engine.updateData();
}
