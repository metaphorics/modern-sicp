// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { Session } from "../../packages/ch4/src/01-metacircular.js";
import { format } from "../../packages/ch4/src/read.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import {
  bin,
  bool,
  call,
  cond,
  type Decl,
  exprStmt,
  functionDecl,
  ident,
  num,
  param,
  returnStmt,
  type Stmt,
  varDecl,
} from "../../packages/ch4/src/syntax/ast.js";
import { noSpan } from "../../packages/ch4/src/syntax/diagnostics.js";
import { applyScanned, execScanned, scannedProcedure, scanOutDefinitions } from "./ex_4_16.js";

const session = (): { session: Session; env: Env } => {
  const engine = new Session("core");
  return { session: engine, env: engine.globalEnv() };
};

/** The observable result: the rendered value, or the fault category and name. */
const shown = (outcome: Outcome): string => {
  if (outcome.tag === "ok") {
    return format(outcome.value);
  }
  return outcome.error.tag === "tdz-access" || outcome.error.tag === "unbound-name"
    ? `error:${outcome.error.tag}:${outcome.error.name}`
    : `error:${outcome.error.tag}`;
};

/** Erases spans so two constructed trees can be compared structurally. */
const normalize = (value: object): unknown =>
  JSON.parse(JSON.stringify(value, (key, item) => (key === "span" ? null : item)));

describe("exercise 4.16: scan out internal defines", () => {
  it("the scan keeps one pre-bound name per declaration and the writes in place", () => {
    const items: Array<Decl> = [
      varDecl("const", "a", num(1), null, noSpan),
      varDecl("const", "b", num(2), null, noSpan),
    ];
    const scanned = scanOutDefinitions(items);
    expect(scanned.names).toEqual(["a", "b"]);
    const { session: engine, env } = session();
    const run = execScanned(scanned, env, engine);
    expect(run.tag).toBe("normal");
  });

  it("a define-free body is unchanged", () => {
    const items: Array<Decl | Stmt> = [exprStmt(bin("+", num(1), num(2)), noSpan)];
    const scanned = scanOutDefinitions(items);
    expect(scanned.names).toEqual([]);
    expect(normalize(scanned.body)).toEqual(normalize(items));
  });

  it("mutually recursive internal definitions answer false for 7 and true for 8", () => {
    const evenOdd = (callName: string): Array<Decl | Stmt> => [
      functionDecl(
        "even?",
        [param("n")],
        [
          exprStmt(
            cond(
              bin("===", ident("n"), num(0)),
              bool(true),
              call(ident("odd?"), [bin("-", ident("n"), num(1))]),
            ),
          ),
        ],
      ),
      functionDecl(
        "odd?",
        [param("n")],
        [
          exprStmt(
            cond(
              bin("===", ident("n"), num(0)),
              bool(false),
              call(ident("even?"), [bin("-", ident("n"), num(1))]),
            ),
          ),
        ],
      ),
      returnStmt(call(ident(callName), [num(7)])),
    ];
    const first = session();
    const procedure7 = scannedProcedure([], evenOdd("even?"), first.env);
    expect(shown(applyScanned(procedure7, [], first.session))).toBe("false");
    const second = session();
    const items = evenOdd("even?").slice(0, 2);
    items.push(returnStmt(call(ident("even?"), [num(8)])));
    const procedure8 = scannedProcedure([], items, second.env);
    expect(shown(applyScanned(procedure8, [], second.session))).toBe("true");
  });

  it("a write that reads a sibling before its write fails with tdz-access on that name", () => {
    const { session: engine, env } = session();
    const scanned = scanOutDefinitions([
      varDecl("const", "a", ident("b"), null, noSpan),
      varDecl("const", "b", num(1), null, noSpan),
      returnStmt(ident("a"), noSpan),
    ]);
    const run = execScanned(scanned, env, engine);
    expect(run.tag).toBe("error");
    if (run.tag === "error") {
      expect(shown({ tag: "error", error: run.error })).toBe("error:tdz-access:b");
    }
  });

  it("the scan runs at procedure creation, not per body read", () => {
    let scans = 0;
    const { env } = session();
    const procedure = scannedProcedure([], [returnStmt(num(1))], env, () => {
      scans += 1;
    });
    const again = session();
    applyScanned(procedure, [], again.session);
    applyScanned(procedure, [], again.session);
    applyScanned(procedure, [], again.session);
    expect(scans).toBe(1);
  });
});
