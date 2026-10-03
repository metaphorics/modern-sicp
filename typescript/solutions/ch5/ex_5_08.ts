// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  assign,
  constant,
  gotoLabel,
  type MachineError,
  type MachineStatement,
  type MachineValue,
} from "../../packages/ch5/src/01-register-machines.ts";
import { type AssemblyResult, assemble, makeMachine } from "../../packages/ch5/src/02-simulator.ts";
import { arithmeticOperations, expectOk, mark } from "./ex_5_07.ts";

/** The book's snippet: here defined twice, there defined only after the
 * second here. The assembler resolves labels by name, so the second
 * definition collides with the first. */
export const duplicateLabelController: readonly MachineStatement[] = [
  mark("start"),
  gotoLabel("here"),
  mark("here"),
  assign("a", constant(1)),
  gotoLabel("there"),
  mark("here"),
  assign("a", constant(2)),
  mark("there"),
];

/** The same control shape with the duplicate removed: there is still
 * defined after its use, and a forward reference resolves to the
 * instruction that follows the label. */
export const forwardLabelController: readonly MachineStatement[] = [
  mark("start"),
  gotoLabel("there"),
  mark("here"),
  assign("a", constant(1)),
  mark("there"),
  assign("a", constant(2)),
];

/** Exercise 5.8: the duplicate is an assembly fault, and a forward
 * reference is not. */
export const ex_5_08 = (): {
  readonly duplicate: MachineError | null;
  readonly forwardAnswer: MachineValue;
} => {
  const duplicate: AssemblyResult = assemble(duplicateLabelController, arithmeticOperations);
  const forward = makeMachine({
    registers: ["a"],
    operations: arithmeticOperations,
    controller: forwardLabelController,
  });
  const run = forward.run();
  expectOk(run);
  const answer = forward.readRegister("a");
  return {
    duplicate: duplicate.ok ? null : duplicate.error,
    forwardAnswer: answer === undefined ? null : answer,
  };
};
