// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  assign,
  constant,
  labelRef,
  type MachineError,
  type MachineStatement,
  op,
  register,
  type Source,
} from "../../packages/ch5/src/01-register-machines.ts";
import { mark } from "./ex_5_07.ts";

/** A machine operation whose operand is written (label b): the book's
 * offending snippet. Labels denote instruction addresses, never operand
 * values, so the exercise's rule rejects them inside operation calls
 * while plain label sources (the continue register's addresses) stay
 * legal. */
export const labelAsOperandController: readonly MachineStatement[] = [
  assign("t", op("+", register("a"), labelRef("b"))),
  mark("b"),
];

/** The offending label of one source tree, or null when the tree holds
 * only values and nested operation calls. The rule is the exercise's
 * added assembly check, stated beside the book's reason: an operation
 * receives values, and an instruction address is not one. */
const labelOperand = (source: Source): string | null => {
  if (source.tag === "label") return source.name;
  if (source.tag === "op") {
    for (const arg of source.args) {
      const offender = labelOperand(arg);
      if (offender !== null) return offender;
    }
  }
  return null;
};

/** The added rule over one controller: every operation call in every
 * assign source and every test/perform argument list is checked, and the
 * first offending label is reported as a `bad-target` fault. */
export const rejectLabelOperands = (
  controller: readonly MachineStatement[],
): MachineError | null => {
  for (const statement of controller) {
    const sources: ReadonlyArray<Source> =
      statement.tag === "assign"
        ? [statement.source]
        : statement.tag === "test" || statement.tag === "perform"
          ? statement.args
          : [];
    for (const source of sources) {
      const offender = labelOperand(source);
      if (offender !== null) {
        return {
          tag: "bad-target",
          detail: `label ${offender} used as an operation operand`,
        };
      }
    }
  }
  return null;
};

/** Exercise 5.9: the offending controller is rejected by the added rule;
 * the same controller with a constant operand passes. */
export const ex_5_09 = (): {
  readonly offending: MachineError | null;
  readonly fixed: MachineError | null;
} => ({
  offending: rejectLabelOperands(labelAsOperandController),
  fixed: rejectLabelOperands([assign("t", op("+", register("a"), constant(2)))]),
});
