import * as backend from "../../pkg/gge_oracle_viewer_wasm";

export type ExprTree =
  | {
      type: "filter";
      id: string;
      args: unknown[];
    }
  | {
      type: "and" | "or";
      exprs: ExprTree[];
    };

function createFilter(id: string, args: unknown[]): backend.Filter | undefined {
  // @ts-expect-error Using id directly as it is identical to function names
  const func = backend.Filter[id];
  if (typeof func !== "function") {
    return undefined;
  }
  try {
    return func(...args);
  } catch {
    return undefined;
  }
}

export function resolveExpr(tree: ExprTree): backend.Expr | undefined {
  if (tree.type === "filter") {
    const filter = createFilter(tree.id, tree.args);
    return filter ? backend.Expr.filter(filter) : undefined;
  }

  const exprs: backend.Expr[] = [];
  for (const subTree of tree.exprs) {
    const expr = resolveExpr(subTree);
    if (!expr) {
      return undefined;
    }
    exprs.push(expr);
  }

  if (tree.type === "and") {
    return backend.Expr.and(exprs);
  }
  if (tree.type === "or") {
    return backend.Expr.or(exprs);
  }
}
