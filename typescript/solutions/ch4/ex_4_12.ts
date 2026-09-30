// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type Binding, type Env, makeCell } from "../../packages/ch4/src/runtime/env.js";
/**
 * Exercise 4.12: abstract environment traversals. The three environment
 * procedures share one shape — walk the chain looking for a name — so
 * the walk is factored into two abstract operations:
 * `forEachBinding` visits every binding of one frame, and `walkEnvChain`
 * visits frame after frame until a per-frame probe answers, answering
 * undefined when the chain runs out. Lookup is then the probe "this
 * frame's initialized cell for the name", set is the probe "the first
 * frame that binds the name" whose answer is the cell to write, and
 * define needs no traversal at all: it always writes the current frame.
 */
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { fail, ok } from "../../packages/ch4/src/runtime/errors.js";
import type { Value } from "../../packages/ch4/src/runtime/value.js";

/** Visits every binding of one frame, in binding order. */
export const forEachBinding = (frame: Env, visit: (binding: Binding) => void): void => {
  for (const [name, cell] of frame.bindings) {
    visit({ name, cell });
  }
};

/** Visits frame after frame until `probe` answers; undefined when the chain runs out. */
export const walkEnvChain = <A>(env: Env, probe: (frame: Env) => A | undefined): A | undefined => {
  let frame: Env | null = env;
  while (frame !== null) {
    const answer = probe(frame);
    if (answer !== undefined) {
      return answer;
    }
    frame = frame.parent;
  }
  return undefined;
};

/** Lookup as the probe "this frame's cell for the name". */
export const lookupTraversing = (name: string, env: Env): Outcome => {
  const cell = walkEnvChain(env, (frame) => frame.bindings.get(name));
  if (cell === undefined) {
    return fail({ tag: "unbound-name", name });
  }
  return cell.initialized ? ok(cell.value) : fail({ tag: "tdz-access", name });
};

/** Set as the probe "the first frame that binds the name". */
export const setTraversing = (name: string, value: Value, env: Env): Outcome => {
  const cell = walkEnvChain(env, (frame) => frame.bindings.get(name));
  if (cell === undefined) {
    return fail({ tag: "unbound-name", name });
  }
  if (!cell.mutable) {
    return fail({ tag: "bad-operand", operator: "=", detail: "assignment to a const binding" });
  }
  cell.value = value;
  cell.initialized = true;
  return ok(value);
};

/** Define writes the current frame directly — the book's own answer. */
export const defineDirect = (name: string, value: Value, env: Env, mutable = true): void => {
  env.bindings.set(name, makeCell(value, true, mutable));
};

export function ex_4_12(): string {
  return (
    "The three environment procedures share one traversal shape, factored into " +
    "forEachBinding (one frame) and walkEnvChain (frame after frame until a probe answers). " +
    "Lookup and set are the two probes; define writes the current frame and needs no " +
    "traversal. On a three-frame chain a = 1, b = 2, c = 3 the traversing lookup answers " +
    "3, 2, 1 and unbound-name for zz; a write to b lands in the middle frame and is " +
    "visible from both inner and middle; defining d is visible from the inner frame only, " +
    "and forEachBinding over the inner frame visits exactly c before the define."
  );
}
