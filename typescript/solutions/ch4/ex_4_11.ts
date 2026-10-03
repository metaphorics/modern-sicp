// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type Binding, type Cell, makeCell } from "../../packages/ch4/src/runtime/env.js";
/**
 * Exercise 4.11: frames as association lists. A frame becomes one list
 * of name-cell bindings instead of two parallel maps: `AListEnv` holds
 * an ordered `Binding` list plus its parent. Lookup scans the current
 * frame's entries and walks the parent chain when absent; set finds the
 * first frame binding the name and rewrites that entry's cell in place;
 * define conses a fresh binding onto the current frame, shadowing outer
 * ones; `frameVariables` reports a frame's own names, in binding order.
 * The edition's own frames stay untouched: this file re-derives the same
 * visible semantics over the book's representation.
 */
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { fail, ok } from "../../packages/ch4/src/runtime/errors.js";
import type { Value } from "../../packages/ch4/src/runtime/value.js";

/** One frame: an association list of name-cell bindings, plus its parent. */
export interface AListEnv {
  readonly bindings: Binding[];
  readonly parent: AListEnv | null;
}

/** A fresh empty frame extending `parent`. */
export const makeAListEnv = (parent: AListEnv | null = null): AListEnv => ({
  bindings: [],
  parent,
});

/** The cell bound to `name` in this frame only. */
const ownCell = (env: AListEnv, name: string): Cell | undefined => {
  for (const binding of env.bindings) {
    if (binding.name === name) {
      return binding.cell;
    }
  }
  return undefined;
};

/** The book's `lookup-variable-value` over association-list frames. */
export const lookupInAList = (name: string, env: AListEnv): Outcome => {
  let frame: AListEnv | null = env;
  while (frame !== null) {
    const cell = ownCell(frame, name);
    if (cell !== undefined) {
      return cell.initialized ? ok(cell.value) : fail({ tag: "tdz-access", name });
    }
    frame = frame.parent;
  }
  return fail({ tag: "unbound-name", name });
};

/** The book's `set-variable-value!`: rewrite the first entry that binds the name. */
export const setInAList = (name: string, value: Value, env: AListEnv): Outcome => {
  let frame: AListEnv | null = env;
  while (frame !== null) {
    const cell = ownCell(frame, name);
    if (cell !== undefined) {
      if (!cell.mutable) {
        return fail({ tag: "bad-operand", operator: "=", detail: "assignment to a const binding" });
      }
      cell.value = value;
      cell.initialized = true;
      return ok(value);
    }
    frame = frame.parent;
  }
  return fail({ tag: "unbound-name", name });
};

/** The book's `define-variable!`: cons a fresh binding onto the current frame. */
export const defineInAList = (name: string, value: Value, env: AListEnv, mutable = true): void => {
  env.bindings.unshift({ name, cell: makeCell(value, true, mutable) });
};

/** The frame's own names, in binding order (newest first). */
export const frameVariables = (env: AListEnv): ReadonlyArray<string> =>
  env.bindings.map((binding) => binding.name);

export function ex_4_11(): string {
  return (
    "A frame is one list of name-cell bindings: lookup scans the entries of the current " +
    "frame and walks the parent chain when absent, set rewrites the first entry that binds " +
    "the name, define conses a fresh binding onto the current frame, and frameVariables " +
    "reports the frame's own names. With a = 1 outer and b = 2 inner, lookup answers 2, 1, " +
    "and unbound-name for zz; a write to a lands in the outer entry and reads back 10; " +
    "defining c leaves the inner frame as [c, b] and the outer as [a]."
  );
}
