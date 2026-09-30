// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.18: the alternative scan-out strategy. Two scans run over
 * one body: the text's scan runs the initializing writes in order, so a
 * write whose value expression reads an earlier sibling sees the
 * sibling's value; the alternative computes every value expression
 * first — in one inner scope, before any write runs — and only then
 * assigns the results into the pre-bound names, so a value expression
 * that reads a sibling lands while that sibling is still uninitialized
 * and fails on the TDZ guard. A sibling read deferred through an
 * earlier-defined procedure and only called later never reads during
 * the writes, so both scans agree.
 */
import { Session } from "../../packages/ch4/src/01-metacircular.js";
import { child, type Env, makeCell } from "../../packages/ch4/src/runtime/env.js";
import type { Completion, Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { failed, normal, outcomeOf } from "../../packages/ch4/src/runtime/errors.js";
import type { Value } from "../../packages/ch4/src/runtime/value.js";
import type { Decl, Expr, Stmt } from "../../packages/ch4/src/syntax/ast.js";
import { addUninitialized, scanOutDefinitions } from "./ex_4_16.js";

/** One initializing write extracted from the scanned body. */
interface Write {
  readonly name: string;
  readonly value: Expr;
}

const splitWrites = (
  items: ReadonlyArray<Decl | Stmt>,
  names: ReadonlyArray<string>,
): { writes: Write[]; rest: Array<Decl | Stmt> } => {
  const writes: Write[] = [];
  const rest: Array<Decl | Stmt> = [];
  for (const item of items) {
    if (
      item.tag === "expr-stmt" &&
      item.expr.tag === "assign" &&
      item.expr.target.tag === "variable" &&
      names.includes(item.expr.target.name)
    ) {
      writes.push({ name: item.expr.target.name, value: item.expr.value });
      continue;
    }
    rest.push(item);
  }
  return { writes, rest };
};

/** The text's scan: the initializing writes run in order. */
export const execTextScan = (
  items: ReadonlyArray<Decl | Stmt>,
  env: Env,
  session: Session = new Session("core"),
): Completion => {
  const scanned = scanOutDefinitions(items);
  for (const name of scanned.names) {
    addUninitialized(name, env);
  }
  return session.execSequence(scanned.body, env);
};

/** The alternative: every value expression first, then the writes. */
export const execAlternativeScan = (
  items: ReadonlyArray<Decl | Stmt>,
  env: Env,
  session: Session = new Session("core"),
): Completion => {
  const scanned = scanOutDefinitions(items);
  for (const name of scanned.names) {
    addUninitialized(name, env);
  }
  const { writes, rest } = splitWrites(scanned.body, scanned.names);
  const inner = child(env);
  const computed: Array<readonly [string, Value]> = [];
  for (const write of writes) {
    const value = session.evaluate(write.value, inner);
    if (value.tag === "error") {
      return failed(value.error);
    }
    computed.push([write.name, value.value]);
  }
  for (const [name, value] of computed) {
    env.bindings.set(name, makeCell(value, true, true));
  }
  return session.execSequence(rest, env);
};

export function ex_4_18(): string {
  return (
    "The text's scan runs the initializing writes in order; the alternative computes every " +
    "value expression in one inner scope before any write runs, then assigns the results. " +
    "Over `const a = 1; const b = a + 1; return b;` the text's scan answers 2 and the " +
    "alternative fails with tdz-access on a, because b's value expression reads a before " +
    "any write has run. Over a procedure whose body reads its sibling only when called, " +
    "both scans answer 5. The book's solve fails under the alternative exactly as the " +
    "sibling-reader does: dy's value expression reads y during the value-computation " +
    "phase, before y's write."
  );
}
