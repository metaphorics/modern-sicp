// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  arithmeticOperations,
  assign,
  branch,
  type ControllerLine,
  c,
  getRegisterContents,
  jump,
  jumpReg,
  lbl,
  type Machine,
  makeMachine,
  mark,
  op,
  perform,
  reg,
  restore,
  save,
  setRegisterContents,
  test,
  type Value,
} from "../../packages/ch5/src/02-simulator.js";
import { expectOk } from "./ex_5_07.js";

/** The recursive factorial controller of figure 5.11, transcribed line
 * for line: each level saves continue and n for the recursive call and
 * restores the pair at after-fact. */
export const recursiveFactorialSimController: ControllerLine[] = [
  assign("continue", lbl("fact-done")),
  mark("fact-loop"),
  test("=", reg("n"), c(1)),
  branch("base-case"),
  save("continue"),
  save("n"),
  assign("n", op("-", reg("n"), c(1))),
  assign("continue", lbl("after-fact")),
  jump("fact-loop"),
  mark("after-fact"),
  restore("n"),
  restore("continue"),
  assign("val", op("*", reg("n"), reg("val"))),
  jumpReg("continue"),
  mark("base-case"),
  assign("val", c(1)),
  jumpReg("continue"),
  mark("fact-done"),
];

/** The controller with the book's measuring perform before fact-done. */
const measuredController: ControllerLine[] = [
  ...recursiveFactorialSimController.slice(0, -1),
  perform("print-stack-statistics"),
  ...recursiveFactorialSimController.slice(-1),
];

const runStatistics = (controller: ReadonlyArray<ControllerLine>, n: number): string => {
  const machine: Machine = expectOk(
    makeMachine(["n", "continue", "val"], arithmeticOperations, controller),
  );
  expectOk(setRegisterContents(machine, "n", n));
  expectOk(machine.start());
  return machine.stack.statisticsLine();
};

/** One run of a controller with input n, answering the stack statistics
 * line the machine's stack reports. */
export const factorialStackStatistics = (): string[] => {
  const table = [1, 2, 3, 4, 5, 6].map(
    (n) => `n = ${n}: ${runStatistics(recursiveFactorialSimController, n)}`,
  );
  const printed = `the measured machine for n = 5 prints: ${runStatistics(measuredController, 5)}`;
  return [...table, printed];
};

/** The factorial value itself, for the oracle pairing in the test. */
export const factorialMachineFactorial = (n: number): Value => {
  const machine = expectOk(
    makeMachine(["n", "continue", "val"], arithmeticOperations, recursiveFactorialSimController),
  );
  expectOk(setRegisterContents(machine, "n", n));
  expectOk(machine.start());
  return expectOk(getRegisterContents(machine, "val"));
};
