// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Decl, Expr, Stmt } from "../../packages/ch4/src/syntax/ast.ts";
import type { Span } from "../../packages/ch4/src/syntax/diagnostics.ts";

const span: Span = { start: 0, end: 0, line: 1, column: 1 };

const isFunctionDecl = (
  item: Decl | Stmt,
): item is Extract<Decl, { readonly tag: "function-decl" }> => item.tag === "function-decl";

/** Exercise 5.43: scan out the internal definitions of a body, turning
 * them into one `const` declaration block whose names start
 * uninitialized and whose definitions become assignments, exactly the
 * transformation the compiler needs before it compiles a lambda body. */
export const scanOutDefines = (body: ReadonlyArray<Decl | Stmt>): readonly (Decl | Stmt)[] => {
  const names: string[] = [];
  const assignments: Stmt[] = [];
  const rest: (Decl | Stmt)[] = [];
  for (const item of body) {
    if (isFunctionDecl(item)) {
      names.push(item.name);
      assignments.push({
        tag: "expr-stmt",
        expr: {
          tag: "assign",
          target: { tag: "variable", name: item.name, span: item.span },
          value: { tag: "arrow", params: item.params, body: item.body, span: item.span },
          span: item.span,
        },
        span: item.span,
      });
    } else {
      rest.push(item);
    }
  }
  if (names.length === 0) return [...body];
  const declarations: Decl[] = names.map((name) => ({
    tag: "var-decl",
    kind: "let",
    name,
    declaredType: null,
    init: { tag: "undefined", span },
    exported: false,
    span,
  }));
  return [...declarations, ...assignments, ...rest];
};

/** The scanned-out shape of the book's example body. */
export const ex_5_43 = (): readonly string[] => {
  const body: (Decl | Stmt)[] = [
    {
      tag: "function-decl",
      name: "even",
      typeParams: [],
      params: [{ kind: "param", name: "n", optional: false, type: null, span }],
      returnType: null,
      body: { body: [{ tag: "return", argument: null, span }], span },
      exported: false,
      span,
    },
    {
      tag: "function-decl",
      name: "odd",
      typeParams: [],
      params: [{ kind: "param", name: "n", optional: false, type: null, span }],
      returnType: null,
      body: { body: [{ tag: "return", argument: null, span }], span },
      exported: false,
      span,
    },
  ];
  return scanOutDefines(body).map((item) => item.tag);
};
