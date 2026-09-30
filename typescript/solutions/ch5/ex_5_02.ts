// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { makeMachine } from "../../packages/ch5/src/02-simulator.ts";
import { factorialIterativeController } from "./ex_5_01.ts";
import { arithmeticOperations, expectOk } from "./ex_5_07.ts";

/** Runs the controller sequence of exercise 5.2: the same iterative
 * factorial controller, assembled as typed machine statements, answers
 * one for zero and the factorial for positive input. */
export const ex_5_02 = (n: number): number => {
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
