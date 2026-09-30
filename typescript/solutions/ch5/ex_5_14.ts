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
import { makeMachine } from "../../packages/ch5/src/02-simulator.ts";
import { arithmeticOperations, expectOk, mark } from "./ex_5_07.ts";

/** Figure 5.11's recursive factorial, transcribed line for line as the
 * simulator exercises measure it. */
export const recursiveFactorialSimController: readonly MachineStatement[] = [
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

/** The executed instructions of one run, label markers excluded: the
 * trace is the machine's ordered renderings, and the label renderings
 * in it carry the label-in-effect information the tracing exercises
 * print. */
export const instructionTrace = (
  controller: readonly MachineStatement[],
  trace: readonly string[],
): readonly { label: string | null; text: string }[] => {
  const labels = new Set(
    controller
      .filter((line): line is Extract<MachineStatement, { tag: "label" }> => line.tag === "label")
      .map((line) => formatMachineStatement(line)),
  );
  let current: string | null = null;
  const out: { label: string | null; text: string }[] = [];
  for (const line of trace) {
    if (labels.has(line)) {
      current = line.slice(0, -1);
      continue;
    }
    out.push({ label: current, text: line });
  }
  return out;
};

const runFactorial = (n: number): { value: number; pushes: number; maxDepth: number } => {
  const machine = makeMachine({
    registers: ["n", "val", "continue"],
    operations: arithmeticOperations,
    controller: recursiveFactorialSimController,
  });
  machine.writeRegister("n", n);
  const run = machine.run();
  expectOk(run);
  const value = machine.readRegister("val");
  return {
    value: typeof value === "number" ? value : 0,
    pushes: run.stackStats.pushes,
    maxDepth: run.stackStats.maxDepth,
  };
};

/** The monitored table of exercise 5.14: each level saves continue and
 * n once, so the counts are exactly 2(n - 1), linear because the
 * controller is linear-recursive. */
export const factorialStackStatistics = (): readonly {
  n: number;
  pushes: number;
  maxDepth: number;
}[] => [1, 2, 3, 4, 5, 6].map((n) => ({ n, ...runFactorial(n) }));

/** The book's own measuring message: the rendered line and the read
 * counters come from the same run and agree. */
export const measuredFactorial = (
  n: number,
): {
  printedLine: string;
  pushes: number;
  maxDepth: number;
  value: number;
} => {
  const run = runFactorial(n);
  return {
    printedLine: `(total-pushes = ${run.pushes} maximum-depth = ${run.maxDepth})`,
    pushes: run.pushes,
    maxDepth: run.maxDepth,
    value: run.value,
  };
};

/** The machine whose stack is measured is the factorial machine. */
export const factorialMachineFactorial = (n: number): number => runFactorial(n).value;

/** Exercise 5.14 answers. */
export const ex_5_14 = (): readonly string[] => {
  const table = factorialStackStatistics();
  const measured = measuredFactorial(5);
  return [
    ...table.map((row) => `n = ${row.n}: pushes = ${row.pushes}, maximum depth = ${row.maxDepth}`),
    `measured n = 5: ${measured.printedLine}`,
    `factorial machine: ${factorialMachineFactorial(5)}`,
  ];
};
