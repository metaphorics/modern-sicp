// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.16: scan out internal defines. Three parts. (a) Lookup
 * already answers `tdz-access` for an uninitialized cell — reading a
 * name before its initializing write is an error rather than a silent
 * wrong value, and `undefined` cannot stand in for "not yet assigned".
 * (b) `scanOutDefinitions` rewrites a body with no declarations
 * remaining: every internal `const`/`let`/function contributes its name
 * to a pre-bound set and its initializing write at the declaration's
 * position, and the remaining statements follow. (c) The scan is
 * installed in the procedure constructor, not in the body reader: the
 * rewrite then runs once when a closure is created instead of on every
 * application that reads the body.
 */
import { extendEnvironment, Session } from "../../packages/ch4/src/01-metacircular.js";
import { type Env, makeCell } from "../../packages/ch4/src/runtime/env.js";
import type { Completion, Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { fail, outcomeOf } from "../../packages/ch4/src/runtime/errors.js";
import {
  type Closure,
  isClosure,
  makeClosure,
  type Value,
} from "../../packages/ch4/src/runtime/value.js";
import {
  assign,
  type Decl,
  type Expr,
  exprStmt,
  ident,
  type Stmt,
} from "../../packages/ch4/src/syntax/ast.js";

/** A scanned body: the names pre-bound at entry, the body's writes and statements. */
export interface ScannedBody {
  readonly names: ReadonlyArray<string>;
  readonly body: ReadonlyArray<Decl | Stmt>;
}

/** Creates the uninitialized cell the book's scan pre-binds. */
export const addUninitialized = (name: string, env: Env): void => {
  env.bindings.set(name, makeCell(undefined, false, true));
};

/**
 * The 4.1.6 transformation: each internal declaration contributes its
 * name to the pre-bound set and an initializing write at its position;
 * the remaining statements follow unchanged.
 */
export const scanOutDefinitions = (items: ReadonlyArray<Decl | Stmt>): ScannedBody => {
  const names: string[] = [];
  const body: Array<Decl | Stmt> = [];
  for (const item of items) {
    if (item.tag === "function-decl") {
      names.push(item.name);
      const procedure: Expr = {
        tag: "arrow",
        params: item.params,
        body: item.body,
        span: item.span,
      };
      body.push(exprStmt(assign(ident(item.name, item.span), procedure, item.span), item.span));
      continue;
    }
    if (item.tag === "var-decl") {
      names.push(item.name);
      body.push(exprStmt(assign(ident(item.name, item.span), item.init, item.span), item.span));
      continue;
    }
    body.push(item);
  }
  return { names, body };
};

/** Runs one scanned body: pre-bind the names, then the initializing writes. */
export const execScanned = (
  scanned: ScannedBody,
  env: Env,
  session: Session = new Session("core"),
): Completion => {
  for (const name of scanned.names) {
    addUninitialized(name, env);
  }
  return session.execSequence(scanned.body, env);
};

/** The scanned bodies installed at procedure creation (part c). */
const installedScans = new WeakMap<object, ScannedBody>();

/** Applies a procedure whose scan was installed at creation. */
export const applyScanned = (
  procedure: Value,
  args: ReadonlyArray<Value>,
  session: Session = new Session("core"),
): Outcome => {
  if (!isClosure(procedure)) {
    return fail({ tag: "not-callable", detail: "applyScanned: value is not a procedure" });
  }
  const scanned = installedScans.get(procedure);
  if (scanned === undefined) {
    return fail({ tag: "not-callable", detail: "applyScanned: no installed scan" });
  }
  const extended = extendEnvironment(procedure.params, args, procedure.env);
  if (extended.tag === "error") {
    return fail(extended.error);
  }
  return outcomeOf(execScanned(scanned, extended.env, session));
};

/**
 * The make-procedure answer to part (c): the rewrite runs once here,
 * when the closure is created, and the scan is installed on the closure
 * for its applications.
 */
export const scannedProcedure = (
  params: ReadonlyArray<string>,
  items: ReadonlyArray<Decl | Stmt>,
  env: Env,
  onScan?: () => void,
): Closure => {
  const scanned = scanOutDefinitions(items);
  onScan?.();
  const procedure = makeClosure(
    params,
    params.length,
    null,
    { body: items, span: items[0]?.span ?? { start: 0, end: 0, line: 1, column: 1 } },
    env,
  );
  installedScans.set(procedure, scanned);
  return procedure;
};

export function ex_4_16(): string {
  return (
    "Lookup answers tdz-access for an uninitialized cell, so reading a name before its " +
    "write is an error rather than a silent wrong value. `scanOutDefinitions` rewrites a " +
    "body to a pre-bound name set plus the initializing writes in place, and the scan is " +
    "installed at procedure creation so the rewrite runs once per closure rather than per " +
    "body read. Over `const a = 1; const b = 2;` then `a + b`, the scanned body is the two " +
    "writes followed by the sum; a define-free body is unchanged; the mutually recursive " +
    "even?/odd? body answers false for 7 and true for 8; and `const a = b; const b = 1` " +
    "fails with tdz-access on b."
  );
}
