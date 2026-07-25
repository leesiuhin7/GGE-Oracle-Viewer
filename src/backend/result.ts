import * as backend from "../../pkg/gge_oracle_viewer_wasm";
import type { Snapshot } from "./snapshot";
import extractFields from "./snapshot";
import type { Engine } from "./wrapper";

export type ResultSnapshots = {
  result: (Snapshot | undefined)[] | undefined;
  type: "match" | "sort";
};

export default class Result {
  private matchResult: backend.MatchResult | null = null;
  private sortingResult: backend.SortingResult | null = null;
  private resultType: "match" | "sort" = "match";
  private readonly engine: Engine;

  constructor(engine: Engine) {
    this.engine = engine;
  }

  public matchAll(expr: backend.Expr) {
    const result = this.engine.matchAll(expr);
    this.matchResult = result;
    this.resultType = "match";
  }

  public sort(comparators: backend.Comparator[]): boolean {
    if (this.matchResult === null) {
      return false;
    }
    const result = this.engine.sort(this.matchResult, comparators);
    if (!result) {
      return false;
    }

    this.sortingResult = result;
    this.resultType = "sort";
    return true;
  }

  public readResult(
    skip: number,
    take: number,
    fields: backend.WrapperField[],
  ): ResultSnapshots {
    if (this.resultType === "match") {
      return {
        result: this.matchResult?.get(skip, take).map((info) => {
          const snapshot = this.engine.buildSnapshot(info, fields);
          if (snapshot) {
            return extractFields(snapshot, fields);
          }
        }),
        type: "match",
      };
    } else {
      return {
        result: this.sortingResult?.get(skip, take)?.map((info) => {
          const snapshot = this.engine.buildSnapshot(info, fields);
          if (snapshot) {
            return extractFields(snapshot, fields);
          }
        }),
        type: "sort",
      };
    }
  }
}
