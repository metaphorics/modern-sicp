// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  assign,
  branch,
  constant,
  formatMachineStatement,
  gotoLabel,
  gotoRegister,
  labelRef,
  type MachineStatement,
  op,
  register,
  restore,
  save,
  test,
} from "../../packages/ch5/src/01-register-machines.ts";
import { type Machine, makeMachine } from "../../packages/ch5/src/02-simulator.ts";
import { arithmeticOperations, expectOk, mark } from "./ex_5_07.ts";

/** One save or restore the hand simulation annotates: its executed step
 * and, for a restore, the step of the save it restores. */
export type StackEvent = {
  readonly tag: "save" | "restore";
  readonly step: number;
  readonly reg: string;
  readonly matchedSave: number | null;
};

/** One machine's observable run: the answer in val, the executed
 * instruction count, the maximum stack depth, and the annotated stack
 * discipline. */
export type HandTrace = {
  readonly value: number;
  readonly instructionCount: number;
  readonly maxDepth: number;
  readonly events: readonly StackEvent[];
};

/** Figure 5.11's recursive factorial, transcribed line for line: each
 * level saves continue and n for the recursive call and restores the
 * pair at after-fact before the multiplication. */
export const factorialRecursiveController: readonly MachineStatement[] = [
  assign("continue", labelRef("fact-done")),
  mark("fact-loop"),
  test("=", register("n"), constant(1)),
  branch("base-case"),
  save("continue"),
  save("n"),
  assign("n", op("-", register("n"), constant(1))),
  assign("continue", labelRef("after-fact")),
  gotoLabel("fact-loop"),
  mark("after-fact"),
  restore("n"),
  restore("continue"),
  assign("val", op("*", register("n"), register("val"))),
  gotoRegister("continue"),
  mark("base-case"),
  assign("val", constant(1)),
  gotoRegister("continue"),
  mark("fact-done"),
];

/** Figure 5.12's recursive Fibonacci, transcribed line for line: each
 * level saves continue before the first call, saves n beside it, and
 * restores the pair at afterfib-n-1, so the second call can be made
 * with n - 2 and val kept across it. */
export const fibonacciRecursiveController: readonly MachineStatement[] = [
  assign("continue", labelRef("fib-done")),
  mark("fib-loop"),
  test("<", register("n"), constant(2)),
  branch("immediate-answer"),
  save("continue"),
  assign("continue", labelRef("afterfib-n-1")),
  save("n"),
  assign("n", op("-", register("n"), constant(1))),
  gotoLabel("fib-loop"),
  mark("afterfib-n-1"),
  restore("n"),
  restore("continue"),
  assign("n", op("-", register("n"), constant(2))),
  save("continue"),
  assign("continue", labelRef("afterfib-n-2")),
  save("val"),
  gotoLabel("fib-loop"),
  mark("afterfib-n-2"),
  assign("n", register("val")),
  restore("val"),
  restore("continue"),
  assign("val", op("+", register("val"), register("n"))),
  gotoRegister("continue"),
  mark("immediate-answer"),
  assign("val", register("n")),
  gotoRegister("continue"),
  mark("fib-done"),
];

/** Annotates the executed trace with the stack discipline: every restore
 * is paired with the save whose value it restores, the most recent
 * unmatched save, so the pairing exposes any broken nesting. */
const labelRenderings = (controller: readonly MachineStatement[]): ReadonlySet<string> =>
  new Set(
    controller
      .filter((line): line is Extract<MachineStatement, { tag: "label" }> => line.tag === "label")
      .map((line) => formatMachineStatement(line)),
  );

/** The executed instructions of one trace: label markers are statements
 * the machine walks, never instructions the hand simulation counts. */
export const instructionLines = (
  controller: readonly MachineStatement[],
  trace: readonly string[],
): readonly string[] => {
  const labels = labelRenderings(controller);
  return trace.filter((line) => !labels.has(line));
};

export const annotateTrace = (
  controller: readonly MachineStatement[],
  trace: readonly string[],
  registers: readonly string[],
): readonly StackEvent[] => {
  const events: StackEvent[] = [];
  const stack: { reg: string; step: number }[] = [];
  const instructions = instructionLines(controller, trace);
  for (let step = 0; step < instructions.length; step += 1) {
    const line = instructions[step];
    if (line === undefined) continue;
    for (const reg of registers) {
      if (line === formatMachineStatement(save(reg))) {
        events.push({ tag: "save", step, reg, matchedSave: null });
        stack.push({ reg, step });
        break;
      }
      if (line === formatMachineStatement(restore(reg))) {
        const top = stack.pop();
        events.push({
          tag: "restore",
          step,
          reg,
          matchedSave: top === undefined ? null : top.step,
        });
        break;
      }
    }
  }
  return events;
};

/** Runs one machine for the hand simulation and gathers its observables. */
export const runHandTrace = (
  controller: readonly MachineStatement[],
  registers: readonly string[],
  n: number,
): HandTrace => {
  const machine: Machine = makeMachine({ registers, operations: arithmeticOperations, controller });
  machine.writeRegister("n", n);
  const run = machine.run();
  expectOk(run);
  const value = machine.readRegister("val");
  if (typeof value !== "number") throw new Error("the machine left no number in val");
  return {
    value,
    instructionCount: instructionLines(controller, run.trace).length,
    maxDepth: run.stackStats.maxDepth,
    events: annotateTrace(controller, run.trace, registers),
  };
};

/** Produces the factorial and Fibonacci hand-simulation traces: the
 * stack events annotate every restore with its matching save. */
export const ex_5_05 = (
  factorialN: number,
  fibonacciN: number,
): { factorial: HandTrace; fibonacci: HandTrace } => ({
  factorial: runHandTrace(factorialRecursiveController, ["n", "val", "continue"], factorialN),
  fibonacci: runHandTrace(fibonacciRecursiveController, ["n", "val", "continue"], fibonacciN),
});
