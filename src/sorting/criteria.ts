import type { SortingCriterion } from "../backend";

export default class Criteria {
  private criteria: SortingCriterion[];
  private usedFields: Set<string>;

  constructor(criteria?: SortingCriterion[]) {
    this.criteria = criteria ?? [];
    this.usedFields = new Set(this.criteria.map(({ field }) => field));
  }

  public value(): SortingCriterion[] {
    return this.criteria;
  }

  public addCriterion(criterion: SortingCriterion): number | undefined {
    const { field } = criterion;
    if (this.usedFields.has(field)) {
      return undefined;
    }
    return this.criteria.push(criterion) - 1;
  }

  public removeCriterion(index: number): SortingCriterion | undefined {
    const criterion = this.criteria.splice(index, 1).at(0);
    if (criterion === undefined) {
      return undefined;
    }
    this.usedFields.delete(criterion.field);
    return criterion;
  }

  public updateDirection(index: number, asc: boolean): boolean {
    const criterion = this.criteria.at(index);
    if (criterion === undefined) {
      return false;
    }
    criterion.desc = !asc;
    return true;
  }

  public moveCriterion(index: number): boolean {
    if (
      this.criteria.length >= 2 &&
      0 < index &&
      index < this.criteria.length
    ) {
      [this.criteria[index - 1], this.criteria[index]] = [
        this.criteria[index],
        this.criteria[index - 1],
      ];
      return true;
    }
    return false;
  }

  public isUsed(field: string): boolean {
    return this.usedFields.has(field);
  }
}
