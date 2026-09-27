// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Connector } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.37: the constraint arithmetic as expression-style
 * combinators that answer their output connector. Pending scaffold;
 * the solution and its rationale live in solutions/ch3/ex_3_37.ts
 * and .md.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 3.37 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** Constrains a fresh connector to `x + y` and answers it, the
 * book's `c+`. */
export function cadd(_x: Connector, _y: Connector): Connector {
  throw new PendingSolution();
}

/** Constrains a fresh connector to `x * y` and answers it, the
 * book's `c*`. */
export function cmul(_x: Connector, _y: Connector): Connector {
  throw new PendingSolution();
}

/** Constrains a fresh connector `z` so that `x = z * y` and answers
 * it, the book's `c/`. */
export function cdiv(_x: Connector, _y: Connector): Connector {
  throw new PendingSolution();
}

/** Constrains a fresh connector to the fixed `value` and answers it,
 * the book's `cv`. */
export function cconst(_value: number): Connector {
  throw new PendingSolution();
}

/** The book's expression-style converter: `F = (9 / 5) * C + 32`. */
export function celsiusFahrenheitConverterExpr(_c: Connector): Connector {
  throw new PendingSolution();
}
