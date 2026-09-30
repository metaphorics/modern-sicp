// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  assign,
  branch,
  constant,
  gotoLabel,
  type MachineStatement,
  op,
  register,
  test,
} from "../../packages/ch5/src/01-register-machines.ts";
import { makeMachine } from "../../packages/ch5/src/02-simulator.ts";
import { arithmeticOperations, expectOk, mark } from "./ex_5_07.ts";

/** The designed iterative factorial machine of 5.1/5.2: product and
 * counter, one pass around the loop per factor, no stack. */
export const factorialIterativeController: readonly MachineStatement[] = [
  assign("product", constant(1)),
  assign("counter", constant(1)),
  mark("test-counter"),
  test(">", register("counter"), register("n")),
  branch("factorial-done"),
  assign("product", op("*", register("counter"), register("product"))),
  assign("counter", op("+", register("counter"), constant(1))),
  gotoLabel("test-counter"),
  mark("factorial-done"),
];

/** Runs the designed iterative factorial machine: the answer is the
 * product register. */
export const ex_5_01 = (n: number): number => {
  const machine = makeMachine({
    registers: ["n", "counter", "product"],
    operations: arithmeticOperations,
    controller: factorialIterativeController,
  });
  machine.writeRegister("n", n);
  const run = machine.run();
  expectOk(run);
  const answer = machine.readRegister("product");
  if (typeof answer !== "number")
    throw new Error("the factorial machine left no number in product");
  return answer;
};
