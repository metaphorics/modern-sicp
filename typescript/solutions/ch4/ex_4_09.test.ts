// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { lookupVariableValue, Session } from "../../packages/ch4/src/01-metacircular.js";
import { format } from "../../packages/ch4/src/read.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import {
  assign,
  bin,
  bool,
  call,
  exprStmt,
  ident,
  lam,
  num,
  param,
  returnStmt,
  varDecl,
} from "../../packages/ch4/src/syntax/ast.js";
import { admitSource } from "../../packages/ch4/src/syntax/check.js";
import { noSpan } from "../../packages/ch4/src/syntax/diagnostics.js";
import { evalIteration, forRange, whileExpr, whileToWhile } from "./ex_4_09.js";

/** A session running one admitted setup unit in its global frame. */
const envWith = (source: string): { session: Session; env: Env } => {
  const admission = admitSource(source);
  if (!admission.ok) {
    throw new Error(`setup source must admit: ${admission.diagnostics[0]?.construct ?? "reject"}`);
  }
  const session = new Session("core");
  const env = session.globalEnv();
  session.execSequence(admission.program, env);
  return { session, env };
};

/** The observable result: the rendered value, or the fault category. */
const shown = (outcome: Outcome): string =>
  outcome.tag === "ok" ? format(outcome.value) : `error:${outcome.error.tag}`;

const numberIn = (env: Env, name: string): number => {
  const found = lookupVariableValue(name, env);
  return found.tag === "ok" && typeof found.value === "number" ? found.value : -1;
};

describe("exercise 4.9: iteration as derived forms", () => {
  it("a range loop over 1..5 accumulating into total answers 15, like the manual recursion", () => {
    const { session, env } = envWith(`
let total = 0;
const sumTo = (n: number): number => (n === 0 ? 0 : n + sumTo(n - 1));
`);
    const loop = forRange(
      "n",
      num(1),
      num(5),
      [exprStmt(assign(ident("total"), bin("+", ident("total"), ident("n"))))],
      noSpan,
    );
    expect(shown(evalIteration(loop, env, session))).toBe("undefined");
    expect(numberIn(env, "total")).toBe(15);
    expect(shown(evalIteration(call(ident("sumTo"), [num(5)]), env, session))).toBe("15");
  });

  it("a while summing 1 through 4 answers 10 at top level", () => {
    const { session, env } = envWith("let total = 0; let i = 1;");
    const loop = whileExpr(
      bin("<=", ident("i"), num(4)),
      [
        exprStmt(assign(ident("total"), bin("+", ident("total"), ident("i")))),
        exprStmt(assign(ident("i"), bin("+", ident("i"), num(1)))),
      ],
      noSpan,
    );
    expect(shown(evalIteration(loop, env, session))).toBe("undefined");
    expect(numberIn(env, "total")).toBe(10);
  });

  it("the same while nested in a procedure body answers 10", () => {
    const { session, env } = envWith("let total = 0;");
    const loop = whileExpr(
      bin("<=", ident("i"), num(4)),
      [
        exprStmt(assign(ident("total"), bin("+", ident("total"), ident("i")))),
        exprStmt(assign(ident("i"), bin("+", ident("i"), num(1)))),
      ],
      noSpan,
    );
    const tally = lam(
      [],
      [varDecl("let", "i", num(1)), exprStmt(whileToWhile(loop)), returnStmt(ident("total"))],
    );
    expect(shown(evalIteration(call(tally, []), env, session))).toBe("10");
    expect(numberIn(env, "total")).toBe(10);
  });

  it("a while with a false predicate never runs and answers undefined", () => {
    const { session, env } = envWith("let ran = 0;");
    const loop = whileExpr(bool(false), [exprStmt(assign(ident("ran"), num(1)))], noSpan);
    expect(shown(evalIteration(loop, env, session))).toBe("undefined");
    expect(numberIn(env, "ran")).toBe(0);
  });

  it("a range loop over 3..3 runs its body once and answers 3", () => {
    const { session, env } = envWith("let once = 0; let runs = 0;");
    const loop = forRange(
      "n",
      num(3),
      num(3),
      [
        exprStmt(assign(ident("once"), ident("n"))),
        exprStmt(assign(ident("runs"), bin("+", ident("runs"), num(1)))),
      ],
      noSpan,
    );
    expect(shown(evalIteration(loop, env, session))).toBe("undefined");
    expect(numberIn(env, "once")).toBe(3);
    expect(numberIn(env, "runs")).toBe(1);
  });
});
