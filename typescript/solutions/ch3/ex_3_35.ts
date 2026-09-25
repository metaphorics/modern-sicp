// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Connector, Constraint } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.35: Ben Bitdiddle's squarer as a new primitive
 * constraint, the book's outline filled in. A value on either
 * terminal determines the other (`b = a * a`, `a = sqrt(b)`); a
 * negative `b` is a square less than zero, the book's `error`; and a
 * lost value withdraws both claims then rechecks, the module's adder
 * discipline, so the network can turn around and repropagate from
 * the other terminal.
 */

/** Constrains `b` to be the square of `a`, both directions: a value
 * on `a` sets `b` to its square, a value on `b` sets `a` to its
 * square root, and a negative `b` is refused, the book's "square
 * less than 0" error. */
export const squarer = (a: Connector, b: Connector): Constraint => {
  const processNewValue = (): void => {
    if (a.hasValue()) {
      b.setValue(a.getValue() * a.getValue(), me);
    } else if (b.hasValue()) {
      const bv = b.getValue();
      if (bv < 0) {
        throw new Error("squarer: negative");
      }
      a.setValue(Math.sqrt(bv), me);
    }
  };
  const processForgetValue = (): void => {
    b.forgetValue(me);
    a.forgetValue(me);
    processNewValue();
  };
  const me: Constraint = (message) => {
    if (message === "informAboutValue") {
      processNewValue();
    } else {
      processForgetValue();
    }
  };
  a.connect(me);
  b.connect(me);
  return me;
};
