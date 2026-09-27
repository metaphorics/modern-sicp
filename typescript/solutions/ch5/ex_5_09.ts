// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  arithmeticOperations,
  assemble,
  assign,
  type ControllerLine,
  getRegisterContents,
  lbl,
  makeMachine,
  makeNewMachine,
  mark,
  op,
  reg,
  renderMachineError,
  setRegisterContents,
  type Value,
} from "../../packages/ch5/src/02-simulator.js";
import { expectError, expectOk } from "./ex_5_07.js";

/** A machine operation whose operand is written (label b). */
const labelAsOperandController: ControllerLine[] = [assign("t", op("+", reg("a"), lbl("b")))];

/** The strict assembler's answer: the typed label-operand error, raised
 * while the machine is being built. */
export const labelOperandOutcome = (): string => {
  const machine = makeNewMachine(["a", "t"], arithmeticOperations);
  const program = assemble(labelAsOperandController, machine, { strictLabels: true });
  return program.ok ? "assembled" : renderMachineError(expectError(program));
};

/** The book's base assembler accepts the same controller and computes
 * the label's address (the index of the first instruction), the behavior
 * the strict variant refuses: a + address(addr) with a = 5 is 5. */
export const labelOperandUnderBaseAssembler = (): Value => {
  const controller: ControllerLine[] = [mark("addr"), assign("t", op("+", reg("a"), lbl("addr")))];
  const machine = expectOk(makeMachine(["a", "t"], arithmeticOperations, controller));
  expectOk(setRegisterContents(machine, "a", 5));
  expectOk(machine.start());
  return expectOk(getRegisterContents(machine, "t"));
};
