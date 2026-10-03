// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.17: the extra frame of scanned-out definitions. The same
 * body runs under three application strategies: sequential (the body's
 * declarations run in order in the call frame), scanned (the 4.16
 * scan's own frame: the pre-bound names live in a child of the call
 * frame, so the scanned application adds exactly one frame per call),
 * and the same-frame design answer (parameters plus the pre-bound names
 * in one frame, so no second frame is built). Frame counts are observed
 * on the environment chain: a program returns a closure and the
 * closure's environment depth is the frame count.
 */
import { extendEnvironment, Session } from "../../packages/ch4/src/01-metacircular.js";
import { child, type Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { fail, outcomeOf } from "../../packages/ch4/src/runtime/errors.js";
import type { Closure, Value } from "../../packages/ch4/src/runtime/value.js";
import type { Decl, Stmt } from "../../packages/ch4/src/syntax/ast.js";
import { addUninitialized, scanOutDefinitions } from "./ex_4_16.js";

/** The number of frames from this environment to the root. */
export const environmentDepth = (env: Env): number => {
  let depth = 0;
  let frame: Env | null = env;
  while (frame !== null) {
    depth += 1;
    frame = frame.parent;
  }
  return depth;
};

/** Sequential application: the body's declarations run in the call frame. */
export const applySequential = (
  procedure: Closure,
  args: ReadonlyArray<Value>,
  session: Session = new Session("core"),
): Outcome => {
  const extended = extendEnvironment(procedure.params, args, procedure.env);
  if (extended.tag === "error") {
    return fail(extended.error);
  }
  return outcomeOf(session.execSequence(procedure.body.body, extended.env));
};

/** Scanned application: the scan's own frame holds the pre-bound names. */
export const applyScannedExtraFrame = (
  procedure: Closure,
  args: ReadonlyArray<Value>,
  session: Session = new Session("core"),
): Outcome => {
  const extended = extendEnvironment(procedure.params, args, procedure.env);
  if (extended.tag === "error") {
    return fail(extended.error);
  }
  const frame = child(extended.env);
  const scanned = scanOutDefinitions(procedure.body.body);
  for (const name of scanned.names) {
    addUninitialized(name, frame);
  }
  return outcomeOf(session.execSequence(scanned.body, frame));
};

/** The design answer: parameters and pre-bound names share one frame. */
export const applySameFrame = (
  procedure: Closure,
  args: ReadonlyArray<Value>,
  session: Session = new Session("core"),
): Outcome => {
  const extended = extendEnvironment(procedure.params, args, procedure.env);
  if (extended.tag === "error") {
    return fail(extended.error);
  }
  const scanned = scanOutDefinitions(procedure.body.body);
  for (const name of scanned.names) {
    addUninitialized(name, extended.env);
  }
  return outcomeOf(session.execSequence(scanned.body, extended.env));
};

export function ex_4_17(): string {
  return (
    "The extra frame is structural: the scanned application runs the pre-bound names in a " +
    "child of the call frame, one frame per call, while the same-frame design extends the " +
    "call frame once with parameters plus the pre-bound names. Over `const a = 1; const b = " +
    "a + 1; return a + b;` all three strategies answer 3; over a body returning a closure " +
    "over its local `a`, the closure's environment depth is 2 sequential, 3 scanned, and " +
    "2 same-frame — the two scanned variants differ by exactly one frame — and applying " +
    "the same-frame closure answers 1."
  );
}
