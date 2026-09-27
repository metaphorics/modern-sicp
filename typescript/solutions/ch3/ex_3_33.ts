// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Connector, Constraint } from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.33: an `averager` over three connectors, the constraint
 * that `c` is the average of `a` and `b`. The book builds it from the
 * primitive multiplier and adder boxes; the edition spells the same
 * three-way arithmetic as one primitive constraint in the module's
 * adder shape, so every value the triangle still determines
 * propagates, and a lost value withdraws exactly the claims the
 * averager made, then rechecks what remains.
 */

/** Constrains `c` to be the average of `a` and `b`: two of the three
 * values determine the third, `c = (a + b) / 2`, `b = 2c - a`, or
 * `a = 2c - b`. When a value is lost, the averager withdraws all
 * three claims (only ones it set are lost) and rechecks, because
 * values it never set may remain. */
export const averager = (a: Connector, b: Connector, c: Connector): Constraint => {
  const processNewValue = (): void => {
    if (a.hasValue() && b.hasValue()) {
      c.setValue((a.getValue() + b.getValue()) / 2, me);
    } else if (c.hasValue() && a.hasValue()) {
      b.setValue(2 * c.getValue() - a.getValue(), me);
    } else if (c.hasValue() && b.hasValue()) {
      a.setValue(2 * c.getValue() - b.getValue(), me);
    }
  };
  const processForgetValue = (): void => {
    a.forgetValue(me);
    b.forgetValue(me);
    c.forgetValue(me);
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
  c.connect(me);
  return me;
};
