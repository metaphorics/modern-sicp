// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type {
  MachineStatement,
  MachineValue,
} from "../../packages/ch5/src/01-register-machines.ts";
import { type Machine, makeMachine } from "../../packages/ch5/src/02-simulator.ts";
import {
  arithmeticOperations,
  expectOk,
  exptIterativeController,
  exptRecursiveController,
} from "./ex_5_07.ts";

const readNumber = (machine: Machine, registerName: string): number => {
  const value: MachineValue | undefined = machine.readRegister(registerName);
  if (typeof value !== "number")
    throw new Error(`the expt machine left no number in ${registerName}`);
  return value;
};

const runExpt = (
  controller: readonly MachineStatement[],
  registers: readonly string[],
  answerRegister: string,
  b: number,
  n: number,
): number => {
  const machine = makeMachine({ registers, operations: arithmeticOperations, controller });
  machine.writeRegister("b", b);
  machine.writeRegister("n", n);
  const run = machine.run();
  expectOk(run);
  return readNumber(machine, answerRegister);
};

/** Runs both exponentiation controllers: the recursive machine answers
 * in val, the iterative one in product. */
export const ex_5_04 = (b: number, n: number): { recursive: number; iterative: number } => ({
  recursive: runExpt(exptRecursiveController, ["b", "n", "val", "continue"], "val", b, n),
  iterative: runExpt(exptIterativeController, ["b", "n", "counter", "product"], "product", b, n),
});
