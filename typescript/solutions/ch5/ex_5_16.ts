// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MachineStatement } from "../../packages/ch5/src/01-register-machines.ts";
import type { Machine } from "../../packages/ch5/src/02-simulator.ts";
import { makeMachine } from "../../packages/ch5/src/02-simulator.ts";
import { arithmeticOperations, expectOk, gcdController } from "./ex_5_07.ts";
import { instructionTrace } from "./ex_5_14.ts";

/** The tracing machine of exercise 5.16: when the switch is on, every
 * executed instruction's text is appended to the transcript before the
 * instruction executes, so the trace is exactly the executed
 * instructions in execution order. Switched off, tracing adds output
 * and nothing else: the same machine answers both halves. */
export const makeTracingMachine = (
  controller: readonly MachineStatement[],
): {
  machine: Machine;
  setTrace: (on: boolean) => void;
  trace: () => readonly string[];
} => {
  const machine = makeMachine({
    registers: ["a", "b", "t"],
    operations: arithmeticOperations,
    controller,
  });
  let on = false;
  return {
    machine,
    setTrace: (next: boolean) => {
      on = next;
    },
    trace: () =>
      on ? instructionTrace(controller, machine.result().trace).map((entry) => entry.text) : [],
  };
};

/** Exercise 5.16 on the gcd machine. */
export const ex_5_16 = (): {
  readonly traced: readonly string[];
  readonly silent: readonly string[];
  readonly answer: number;
} => {
  const tracer = makeTracingMachine(gcdController);
  tracer.setTrace(true);
  tracer.machine.writeRegister("a", 206);
  tracer.machine.writeRegister("b", 40);
  const run = tracer.machine.run();
  expectOk(run);
  const traced = tracer.trace();
  const answer = tracer.machine.readRegister("a");
  tracer.setTrace(false);
  const silent = tracer.trace();
  return { traced, silent, answer: typeof answer === "number" ? answer : 0 };
};
