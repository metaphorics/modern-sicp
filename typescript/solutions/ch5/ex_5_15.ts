// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  arithmeticOperations,
  type ControllerLine,
  getRegisterContents,
  type Machine,
  makeMachine,
  type Operation,
  setRegisterContents,
  type Value,
} from "../../packages/ch5/src/02-simulator.js";
import { expectOk, gcdController } from "./ex_5_07.js";
import { recursiveFactorialSimController } from "./ex_5_14.js";

/** The counting machine: every executed instruction advances the
 * counter, transfers included. The count is read after the run, and the
 * machine answers the book's print-and-reset message, which appends the
 * count to the transcript and zeroes the counter. */
export const makeCountingMachine = (
  registerNames: string[],
  controller: ReadonlyArray<ControllerLine>,
  operations: Record<string, Operation> = arithmeticOperations,
): { machine: Machine; instructionCount: () => number; printInstructionCount: () => number } => {
  let count = 0;
  const machine = expectOk(
    makeMachine(registerNames, operations, controller, {
      onInstruction: () => {
        count += 1;
      },
    }),
  );
  return {
    machine,
    instructionCount: () => count,
    printInstructionCount: () => {
      machine.transcript.push(String(count));
      const printed = count;
      count = 0;
      return printed;
    },
  };
};

const countedRun = (
  machine: Machine,
  registers: Record<string, number>,
  answerReg: string,
): Value => {
  for (const [name, value] of Object.entries(registers)) {
    expectOk(setRegisterContents(machine, name, value));
  }
  expectOk(machine.start());
  return expectOk(getRegisterContents(machine, answerReg));
};

/** The gcd and factorial machines with their instruction counts for one
 * run each. */
export const gcdInstructionCounts = (): string[] => {
  const gcd = makeCountingMachine(["a", "b", "t"], gcdController);
  countedRun(gcd.machine, { a: 206, b: 40 }, "a");
  const fact = makeCountingMachine(["n", "continue", "val"], recursiveFactorialSimController);
  countedRun(fact.machine, { n: 5 }, "val");
  return [
    `gcd(206, 40): ${gcd.instructionCount()} instructions`,
    `factorial(5): ${fact.instructionCount()} instructions`,
  ];
};

/** The print-and-reset message: the count is returned and printed, and
 * the counter is back to zero afterwards. */
export const printAndReset = (): { printed: number; after: number; transcript: string[] } => {
  const fib = makeCountingMachine(["n", "continue", "val"], recursiveFactorialSimController);
  countedRun(fib.machine, { n: 3 }, "val");
  const printed = fib.printInstructionCount();
  return {
    printed,
    after: fib.instructionCount(),
    transcript: [...fib.machine.transcript],
  };
};
