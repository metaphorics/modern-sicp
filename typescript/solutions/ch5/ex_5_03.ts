// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  assign,
  branch,
  constant,
  gotoLabel,
  type MachineStatement,
  type MachineValue,
  type Operation,
  op,
  register,
  test,
} from "../../packages/ch5/src/01-register-machines.ts";
import { type Machine, makeMachine } from "../../packages/ch5/src/02-simulator.ts";
import { arithmeticOperations, expectOk, mark } from "./ex_5_07.ts";

const TOLERANCE = 0.001;

/** The first design: good-enough? and improve stay primitive operations
 * of the machine. */
export const sqrtPrimitiveController: readonly MachineStatement[] = [
  assign("guess", constant(1)),
  mark("test-guess"),
  test("good-enough?", register("guess"), register("x")),
  branch("sqrt-done"),
  assign("guess", op("improve", register("guess"), register("x"))),
  gotoLabel("test-guess"),
  mark("sqrt-done"),
];

/** The expanded design: only arithmetic operations, the Newton step and
 * the acceptance test spelled out over the scratch register t. */
export const sqrtExpandedController: readonly MachineStatement[] = [
  assign("guess", constant(1)),
  mark("test-guess"),
  assign("t", op("*", register("guess"), register("guess"))),
  assign("t", op("-", register("t"), register("x"))),
  assign("t", op("abs", register("t"))),
  test("<", register("t"), constant(TOLERANCE)),
  branch("sqrt-done"),
  assign("t", op("/", register("x"), register("guess"))),
  assign("t", op("+", register("guess"), register("t"))),
  assign("guess", op("/", register("t"), constant(2))),
  gotoLabel("test-guess"),
  mark("sqrt-done"),
];

/** The two primitive operations of the first design, over machine words. */
const sqrtPrimitives: Readonly<Record<string, Operation>> = {
  "good-enough?": (args) => {
    const guess = args[0];
    const x = args[1];
    if (typeof guess !== "number" || typeof x !== "number")
      throw new Error("good-enough?: expected number operands");
    return Math.abs(guess * guess - x) < TOLERANCE;
  },
  improve: (args) => {
    const guess = args[0];
    const x = args[1];
    if (typeof guess !== "number" || typeof x !== "number")
      throw new Error("improve: expected number operands");
    return (x / guess + guess) / 2;
  },
};

const readGuess = (machine: Machine): number => {
  const guess: MachineValue | undefined = machine.readRegister("guess");
  if (typeof guess !== "number") throw new Error("the sqrt machine left no number in guess");
  return guess;
};

const runSqrt = (
  controller: readonly MachineStatement[],
  registers: readonly string[],
  operations: Readonly<Record<string, Operation>>,
  x: number,
): number => {
  const machine = makeMachine({ registers, operations, controller });
  machine.writeRegister("x", x);
  const run = machine.run();
  expectOk(run);
  return readGuess(machine);
};

/** Compares the primitive and the expanded Newton machines: both answer
 * the square root within the stated tolerance. */
export const ex_5_03 = (x: number): { readonly primitive: number; readonly expanded: number } => ({
  primitive: runSqrt(
    sqrtPrimitiveController,
    ["x", "guess"],
    { ...arithmeticOperations, ...sqrtPrimitives },
    x,
  ),
  expanded: runSqrt(sqrtExpandedController, ["x", "guess", "t"], arithmeticOperations, x),
});
