// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.15: the halting diagonal, executed. `halts` is provided by
 * the host as an oracle with an adjustable answer, and the object
 * language gets the book's `runForever` and `tryProgram`. Because a
 * diverging evaluation never returns, a fuel counter is wrapped around
 * the recursion: every application of `runForever` spends one unit, and
 * one spend past the limit fails instead of hanging, which turns "runs
 * forever" into an observable, pinnable outcome. The theorem the runs
 * demonstrate: no `halts` correctly decides halting. If the oracle says
 * true for `tryProgram(tryProgram)`, the try drives into `runForever`
 * and diverges, so the answer was false; if it says false, the try
 * halts with "halted", so the answer was true.
 */
import { Session } from "../../packages/ch4/src/01-metacircular.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { fail, ok } from "../../packages/ch4/src/runtime/errors.js";
import {
  makePrimitive,
  type PrimitiveProcedure,
  type Value,
} from "../../packages/ch4/src/runtime/value.js";
import {
  call,
  type Expr,
  exprStmt,
  ident,
  ifStmt,
  lam,
  num,
  param,
  returnStmt,
  str,
} from "../../packages/ch4/src/syntax/ast.js";
import { noSpan } from "../../packages/ch4/src/syntax/diagnostics.js";
/** The host's `halts` oracle: an adjustable answer, never a real decision. */
export const makeHaltsOracle = (answer: boolean): PrimitiveProcedure =>
  makePrimitive("halts", (_args: ReadonlyArray<Value>): Outcome => ok(answer));

/** A fuel counter: each spend draws one unit; the first spend past the
 * limit fails with `bad-operand`, so divergence becomes observable. */
export const makeFuel = (limit: number): { spend: PrimitiveProcedure; spent: () => number } => {
  let spent = 0;
  return {
    spend: makePrimitive("@@spend", (_args: ReadonlyArray<Value>): Outcome => {
      spent += 1;
      return spent > limit
        ? fail({ tag: "bad-operand", operator: "@@spend", detail: "out of fuel" })
        : ok(spent);
    }),
    spent: () => spent,
  };
};

/** `function runForever(): void { runForever(); }`, spending one unit
 * of fuel per iteration. */
export const runForeverProgram = (): Expr =>
  lam([], [exprStmt(call(ident("@@spend"), [])), exprStmt(call(ident("runForever"), []))]);

/**
 * `function tryProgram(p) { if (halts(p, p)) { runForever(); } return
 * "halted"; }` — the book's try, over the host oracle.
 */
export const tryProgramProgram = (): Expr =>
  lam(
    [param("p")],
    [
      ifStmt(call(ident("halts"), [ident("p"), ident("p")]), {
        tag: "block",
        body: [exprStmt(call(ident("runForever"), []))],
        span: noSpan,
      }),
      returnStmt(str("halted")),
    ],
  );

/** Installs the oracle, the fuel, and both programs in one frame. */
export const haltingEnv = (
  answer: boolean,
  limit: number,
): { session: Session; env: Env; spent: () => number } => {
  const session = new Session("core");
  const env = session.globalEnv();
  const fuel = makeFuel(limit);
  env.bindings.set("@@spend", { value: fuel.spend, initialized: true, mutable: true });
  env.bindings.set("halts", { value: makeHaltsOracle(answer), initialized: true, mutable: true });
  for (const [name, expression] of [
    ["runForever", runForeverProgram()],
    ["tryProgram", tryProgramProgram()],
  ] as const) {
    const outcome = session.evaluate(expression, env);
    if (outcome.tag === "error") {
      throw new Error(`failed to install ${name}: ${outcome.error.tag}`);
    }
    session.defineVariableValue(name, outcome.value, env);
  }
  return { session, env, spent: fuel.spent };
};

export function ex_4_15(): string {
  return (
    "No `halts` correctly decides halting. Under the true oracle, `tryProgram(tryProgram)` " +
    "drives into `runForever` and exhausts its fuel at the 501st spend — the first spend " +
    "past the 500-unit limit is the failure — so the oracle's answer was false. Under the " +
    'false oracle the same call answers "halted" having spent well under 500 units, so ' +
    "the answer was true. Under the true oracle even a procedure that halts on itself " +
    "exhausts the fuel, because the oracle says true and the try drives into " +
    "`runForever`. Divergence is observable because the fuel cap turns it into a typed " +
    "failure instead of a hang."
  );
}
