// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { MachineStatement } from "../../packages/ch5/src/01-register-machines.ts";
import { type Machine, makeMachine } from "../../packages/ch5/src/02-simulator.ts";
import { arithmeticOperations, expectOk, gcdController } from "./ex_5_07.ts";
import { instructionTrace } from "./ex_5_14.ts";

/** The label-aware tracing machine of exercise 5.17: the label
 * currently in effect, the most recently passed label definition, is
 * printed ahead of each traced instruction, so the trace reads like the
 * controller listing. The label information comes from the machine's
 * own trace, so instruction counting is untouched. */
export const makeLabelTracingMachine = (
  controller: readonly MachineStatement[],
): {
  machine: Machine;
  labelTrace: () => readonly string[];
} => {
  const machine = makeMachine({
    registers: ["a", "b", "t"],
    operations: arithmeticOperations,
    controller,
  });
  return {
    machine,
    labelTrace: () =>
      instructionTrace(controller, machine.result().trace).map((entry) =>
        entry.label === null ? entry.text : `${entry.label}: ${entry.text}`,
      ),
  };
};

/** Exercise 5.17 on the gcd machine. */
export const ex_5_17 = (): { readonly lines: readonly string[]; readonly answer: number } => {
  const tracer = makeLabelTracingMachine(gcdController);
  tracer.machine.writeRegister("a", 206);
  tracer.machine.writeRegister("b", 40);
  const run = tracer.machine.run();
  expectOk(run);
  const answer = tracer.machine.readRegister("a");
  return { lines: tracer.labelTrace(), answer: typeof answer === "number" ? answer : 0 };
};
