import type { ExprTree } from "../backend";

export type ExprInfo =
  | {
      type: "filter";
      id: string;
      args: unknown[];
    }
  | {
      type: "and" | "or";
      exprIds: number[];
    }
  | { type: "error" };

export function resolveExpr(
  expr: ExprInfo,
  exprMap: Map<number, ExprInfo>,
): ExprTree | undefined {
  const usedExpr: Set<number> = new Set();

  const resolve: (exprInfo: ExprInfo) => ExprTree | undefined = (exprInfo) => {
    if (exprInfo.type === "error") {
      return undefined;
    }
    if (exprInfo.type === "filter") {
      return exprInfo;
    }

    const exprs: ExprTree[] = [];
    for (const id of exprInfo.exprIds) {
      if (usedExpr.has(id)) {
        return undefined; // Recursion error
      }
      const exprInfo = exprMap.get(id);
      if (exprInfo === undefined) {
        return undefined;
      }

      usedExpr.add(id); // Push stack
      const expr = resolve(exprInfo);
      usedExpr.delete(id); // Pop stack

      if (expr === undefined) {
        return undefined;
      }
      exprs.push(expr);
    }

    return {
      type: exprInfo.type,
      exprs,
    };
  };

  return resolve(expr);
}
