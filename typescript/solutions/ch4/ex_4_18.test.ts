// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { Session } from "../../packages/ch4/src/01-metacircular.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Completion } from "../../packages/ch4/src/runtime/errors.js";
import {
  bin,
  call,
  type Decl,
  functionDecl,
  ident,
  num,
  returnStmt,
  type Stmt,
  varDecl,
} from "../../packages/ch4/src/syntax/ast.js";
import { noSpan } from "../../packages/ch4/src/syntax/diagnostics.js";
import { execAlternativeScan, execTextScan } from "./ex_4_18.js";

const session = (): { session: Session; env: Env } => {
  const engine = new Session("core");
  return { session: engine, env: engine.globalEnv() };
};

const shown = (completion: Completion): string => {
  if (completion.tag === "normal" || completion.tag === "return") {
    return String(completion.value);
  }
  return completion.tag === "error"
    ? completion.error.tag === "tdz-access" ||
      (completion.tag === "error" && completion.error.tag === "unbound-name")
      ? `error:${completion.error.tag}`
      : `error:${completion.error.tag}`
    : `completion:${completion.tag}`;
};

describe("exercise 4.18: the alternative scan-out strategy", () => {
  const g: Array<Decl | Stmt> = [
    varDecl("const", "a", num(1), null, noSpan),
    varDecl("const", "b", bin("+", ident("a"), num(1)), null, noSpan),
    returnStmt(ident("b"), noSpan),
  ];

  it("the text's scan answers 2; the alternative fails on the unassigned sibling", () => {
    const first = session();
    expect(shown(execTextScan(g, first.env, first.session))).toBe("2");
    const second = session();
    expect(shown(execAlternativeScan(g, second.env, second.session))).toBe("error:tdz-access");
  });

  it("a deferred sibling read agrees under both scans: 5", () => {
    const h: Array<Decl | Stmt> = [
      functionDecl("get", [], [returnStmt(ident("b"), noSpan)]),
      varDecl("const", "b", num(5), null, noSpan),
      returnStmt(call(ident("get"), []), noSpan),
    ];
    const first = session();
    expect(shown(execTextScan(h, first.env, first.session))).toBe("5");
    const second = session();
    expect(shown(execAlternativeScan(h, second.env, second.session))).toBe("5");
  });
});
