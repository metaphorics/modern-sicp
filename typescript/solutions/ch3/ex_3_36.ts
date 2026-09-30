// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  type Connector,
  type Constraint,
  type ConstraintMessage,
  ContradictionError,
  type Informant,
} from "../../packages/ch3/src/03-mutable-data.js";

/**
 * Exercise 3.36: the environment structure the book draws for
 *
 *   const a = makeConnector();
 *   const b = makeConnector();
 *   setValue(a, 10, "user");
 *
 * with the connector's local `forEachExcept` call live in it. The
 * edition's connectors are closures over exactly that local state,
 * so the drawing becomes an execution trace: one line per message
 * in the wiring and the `setValue` exchange, every arrow the book drew
 * visible as a from/to event. The traced connector is the module's
 * connector plus the log; the traced constraint boxes are the
 * module's adder, multiplier, and constant with their labels
 * registered, so the connector can name who it is talking to.
 */

/** One line of the trace: a message `what` sent `from` a connector
 * or constraint label `to` one. `setValue` is a setter claiming a
 * value; the other two are the connector's `forEachExcept`
 * notifications the book's diagram centers on. */
export interface TraceEvent {
  readonly from: string;
  readonly to: string;
  readonly what: "informAboutValue" | "informAboutNoValue" | "setValue";
}

/** Names constraint dispatches for the connectors that notify them:
 * the traced builders register their dispatch under `label` before
 * connecting, so events name constraint boxes the way the book's
 * diagram does. */
const constraintLabels = new WeakMap<Constraint, string>();

const nameOf = (who: Informant): string =>
  typeof who === "string" ? who : (constraintLabels.get(who) ?? "constraint");

/** Builds the module's connector plus a trace: a claim logs the
 * `setValue` line and then the `forEachExcept` value notices, a
 * lost value logs the `forEachExcept` lost notices, and a late
 * `connect` logs the catch-up notice the module's connector sends. */
export const makeTracedConnector = (label: string, log: TraceEvent[]): Connector => {
  let value: number | undefined;
  let informant: Informant | undefined;
  let constraints: Constraint[] = [];
  const forEachExcept = (exception: Informant, message: ConstraintMessage): void => {
    for (const constraint of constraints) {
      if (constraint !== exception) {
        log.push({ from: label, to: nameOf(constraint), what: message });
        constraint(message);
      }
    }
  };
  return {
    hasValue: () => value !== undefined,
    getValue: () => {
      if (value === undefined) {
        throw new Error("connector has no value");
      }
      return value;
    },
    setValue: (newValue, setter) => {
      if (value === undefined) {
        log.push({ from: nameOf(setter), to: label, what: "setValue" });
        value = newValue;
        informant = setter;
        forEachExcept(setter, "informAboutValue");
      } else if (value !== newValue) {
        throw new ContradictionError(value, newValue);
      }
    },
    forgetValue: (retractor) => {
      if (retractor === informant) {
        informant = undefined;
        value = undefined;
        forEachExcept(retractor, "informAboutNoValue");
      }
    },
    connect: (newConstraint) => {
      if (!constraints.includes(newConstraint)) {
        constraints = [newConstraint, ...constraints];
      }
      if (value !== undefined) {
        log.push({ from: label, to: nameOf(newConstraint), what: "informAboutValue" });
        newConstraint("informAboutValue");
      }
    },
  };
};

/** The module's adder wearing a label, so the traced connectors name
 * it in the events its claims trigger. */
export const tracedAdder = (
  label: string,
  a1: Connector,
  a2: Connector,
  sum: Connector,
  _log: TraceEvent[],
): Constraint => {
  const processNewValue = (): void => {
    if (a1.hasValue() && a2.hasValue()) {
      sum.setValue(a1.getValue() + a2.getValue(), me);
    } else if (a1.hasValue() && sum.hasValue()) {
      a2.setValue(sum.getValue() - a1.getValue(), me);
    } else if (a2.hasValue() && sum.hasValue()) {
      a1.setValue(sum.getValue() - a2.getValue(), me);
    }
  };
  const processForgetValue = (): void => {
    sum.forgetValue(me);
    a1.forgetValue(me);
    a2.forgetValue(me);
    processNewValue();
  };
  const me: Constraint = (message) => {
    if (message === "informAboutValue") {
      processNewValue();
    } else {
      processForgetValue();
    }
  };
  constraintLabels.set(me, label);
  a1.connect(me);
  a2.connect(me);
  sum.connect(me);
  return me;
};

/** The module's multiplier wearing a label, so the traced connectors
 * name it in the events its claims trigger. */
export const tracedMultiplier = (
  label: string,
  m1: Connector,
  m2: Connector,
  product: Connector,
  _log: TraceEvent[],
): Constraint => {
  const processNewValue = (): void => {
    if ((m1.hasValue() && m1.getValue() === 0) || (m2.hasValue() && m2.getValue() === 0)) {
      product.setValue(0, me);
    } else if (m1.hasValue() && m2.hasValue()) {
      product.setValue(m1.getValue() * m2.getValue(), me);
    } else if (product.hasValue() && m1.hasValue()) {
      m2.setValue(product.getValue() / m1.getValue(), me);
    } else if (product.hasValue() && m2.hasValue()) {
      m1.setValue(product.getValue() / m2.getValue(), me);
    }
  };
  const processForgetValue = (): void => {
    product.forgetValue(me);
    m1.forgetValue(me);
    m2.forgetValue(me);
    processNewValue();
  };
  const me: Constraint = (message) => {
    if (message === "informAboutValue") {
      processNewValue();
    } else {
      processForgetValue();
    }
  };
  constraintLabels.set(me, label);
  m1.connect(me);
  m2.connect(me);
  product.connect(me);
  return me;
};

/** The module's constant wearing a label. It registers before it
 * claims its connector, so the claim's event names the constant box;
 * every message sent to the constant box is an error, as in the
 * book. */
export const tracedConstant = (
  label: string,
  value: number,
  connector: Connector,
  _log: TraceEvent[],
): Constraint => {
  const me: Constraint = () => {
    throw new Error("Unknown request: CONSTANT");
  };
  constraintLabels.set(me, label);
  connector.connect(me);
  connector.setValue(value, me);
  return me;
};
