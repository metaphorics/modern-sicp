// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.1

/**
 * The primitive table (decision D19): a real `Map` keyed by the operation
 * name. `get` on a missing key returns the absent `Option`, never `false`;
 * `put` overwrites. Installing the book's actual primitives is section 4.1's
 * lesson; the spine only owns the table.
 */
import { Option } from "effect";

import type { Primitive } from "./core.js";

export class OpTable {
  readonly #ops: Map<string, Primitive>;

  constructor() {
    this.#ops = new Map<string, Primitive>();
  }

  /** Installs or replaces the handler for `name`. */
  put(name: string, fn: Primitive): void {
    this.#ops.set(name, fn);
  }

  /** The handler for `name`, or nothing when absent. */
  get(name: string): Option.Option<Primitive> {
    return Option.fromNullishOr(this.#ops.get(name));
  }
}
