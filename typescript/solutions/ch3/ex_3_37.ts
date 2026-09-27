// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  adder,
  type Connector,
  constant,
  makeConnector,
  multiplier,
} from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.37: the constraint arithmetic as expression-style
 * combinators. Each c-procedure builds its output connector, wires
 * the module's primitive constraint between its inputs and the
 * output, and answers the output, so compound constraints compose
 * like arithmetic expressions instead of naming every intermediate
 * connector, the book's footnote point about returning compound
 * objects as values.
 */

/** Constrains a fresh connector to `x + y` and answers it, the
 * book's `c+`. */
export const cadd = (x: Connector, y: Connector): Connector => {
  const z = makeConnector();
  adder(x, y, z);
  return z;
};

/** Constrains a fresh connector to `x * y` and answers it, the
 * book's `c*`. */
export const cmul = (x: Connector, y: Connector): Connector => {
  const z = makeConnector();
  multiplier(x, y, z);
  return z;
};

/** Constrains a fresh connector `z` so that `x = z * y` and answers
 * it, the book's `c/`. */
export const cdiv = (x: Connector, y: Connector): Connector => {
  const z = makeConnector();
  multiplier(y, z, x);
  return z;
};

/** Constrains a fresh connector to the fixed `value` and answers it,
 * the book's `cv`. */
export const cconst = (value: number): Connector => {
  const z = makeConnector();
  constant(value, z);
  return z;
};

/** The book's expression-style converter: `F = (9 / 5) * C + 32`,
 * built from the combinators with no intermediate connector named. */
export const celsiusFahrenheitConverterExpr = (c: Connector): Connector =>
  cadd(cmul(cdiv(cconst(9), cconst(5)), c), cconst(32));
