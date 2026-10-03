// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MachineStatement, Operation } from "../../packages/ch5/src/01-register-machines.ts";
import type { Machine, MachineRun } from "../../packages/ch5/src/02-simulator.ts";
import { makeMachine } from "../../packages/ch5/src/02-simulator.ts";
import { fibonacciRecursiveController } from "./ex_5_05.ts";
import { fibonacciReducedController } from "./ex_5_06.ts";
import { arithmeticOperations, expectOk, gcdController } from "./ex_5_07.ts";
import { instructionTrace, recursiveFactorialSimController } from "./ex_5_14.ts";

/** The counting machine of exercise 5.15: every executed instruction
 * advances the counter, transfers included. The count comes from the
 * machine's ordered trace reduced to instructions, and print-and-reset
 * returns the count, appends it to the transcript, and moves the
 * baseline so a later count measures only later runs. */
export const makeCountingMachine = (
  registers: readonly string[],
  operations: Readonly<Record<string, Operation>>,
  controller: readonly MachineStatement[],
): {
  machine: Machine;
  count: () => number;
  printInstructionCount: (transcript: string[]) => number;
} => {
  const machine = makeMachine({ registers, operations, controller });
  let baseline = 0;
  let printed = 0;
  return {
    machine,
    count: () => {
      const run: MachineRun = machine.result();
      return instructionTrace(controller, run.trace).length - baseline;
    },
    printInstructionCount: (transcript: string[]) => {
      const run = machine.result();
      const value = instructionTrace(controller, run.trace).length - baseline;
      baseline = instructionTrace(controller, run.trace).length;
      printed = value;
      transcript.push(String(printed));
      return printed;
    },
  };
};

const countRun = (
  registers: readonly string[],
  controller: readonly MachineStatement[],
  inputs: Readonly<Record<string, number>>,
): number => {
  const counter = makeCountingMachine(registers, arithmeticOperations, controller);
  for (const [name, value] of Object.entries(inputs)) {
    counter.machine.writeRegister(name, value);
  }
  const run = counter.machine.run();
  expectOk(run);
  return counter.count();
};

/** The pinned counts: gcd(206, 40) at 26, figure 5.11 factorial at 49
 * for n = 5, and the Fibonacci boundaries of the book's table. */
export const gcdInstructionCounts = (a: number, b: number): number =>
  countRun(["a", "b", "t"], gcdController, { a, b });

export const factorialInstructionCount = (n: number): number =>
  countRun(["n", "val", "continue"], recursiveFactorialSimController, { n });

export const fibInstructionCount = (n: number): number =>
  countRun(["n", "val", "continue"], fibonacciRecursiveController, { n });

export const reducedFibInstructionCount = (n: number): number =>
  countRun(["n", "val", "continue"], fibonacciReducedController, { n });

/** The book's print-and-reset message on one factorial(3) run. */
export const printAndReset = (): {
  message: number;
  transcript: readonly string[];
  after: number;
} => {
  const counter = makeCountingMachine(
    ["n", "val", "continue"],
    arithmeticOperations,
    recursiveFactorialSimController,
  );
  counter.machine.writeRegister("n", 3);
  const run = counter.machine.run();
  expectOk(run);
  const transcript: string[] = [];
  const message = counter.printInstructionCount(transcript);
  return { message, transcript, after: counter.count() };
};

/** Exercise 5.15 answers. */
export const ex_5_15 = (): readonly number[] => [
  gcdInstructionCounts(206, 40),
  factorialInstructionCount(5),
  fibInstructionCount(0),
  fibInstructionCount(1),
  fibInstructionCount(2),
  fibInstructionCount(3),
  fibInstructionCount(6),
];
