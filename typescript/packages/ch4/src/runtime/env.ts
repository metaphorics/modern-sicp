// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: sections 4.1 to 5.5

/**
 * Environments and cells (host-subsets grammar section 5). A binding maps a
 * name to one mutable `Cell`; closures capture cells by reference, so a
 * captured `let` write is visible through every alias and recursive
 * environment frames never copy captured mutable cells. A cell with
 * `initialized: false` is the temporal dead zone: reading it is a `tdz-access`
 * error, exactly the ECMAScript behavior the subset follows.
 */
import type { Value } from "./value.ts";

/** One mutable binding slot. `initialized: false` is the TDZ state;
 * `mutable: false` marks a `const` binding whose reassignment the runtime
 * rejects even for hand-built syntax. */
export interface Cell {
  value: Value | undefined;
  initialized: boolean;
  mutable: boolean;
}

/** One name-to-cell binding in a frame. */
export interface Binding {
  readonly name: string;
  readonly cell: Cell;
}

/** A lexical frame: name-to-cell bindings plus the enclosing frame. */
export interface Env {
  readonly bindings: Map<string, Cell>;
  readonly parent: Env | null;
}

/** A fresh empty frame extending `parent`, as a call or block does. */
export const child = (parent: Env | null): Env => ({ bindings: new Map(), parent });

/** A fresh uninitialized or initialized cell. */
export const makeCell = (value: Value | undefined, initialized: boolean, mutable = true): Cell => ({
  value,
  initialized,
  mutable,
});

/** The cell bound to `name` along the frame chain, or nothing when unbound. */
export const findCell = (env: Env | null, name: string): Cell | undefined => {
  let current = env;
  while (current !== null) {
    const own = current.bindings.get(name);
    if (own !== undefined) {
      return own;
    }
    current = current.parent;
  }
  return undefined;
};
