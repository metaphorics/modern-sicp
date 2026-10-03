// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Session } from "../../packages/ch4/src/01-metacircular.ts";
import { makeMachine } from "../../packages/ch5/src/02-simulator.ts";
import { readProgram, type Word } from "../../packages/ch5/src/04-eceval.ts";
import {
  compiledOperations,
  compileProgram,
  isCompileError,
} from "../../packages/ch5/src/05-compilation.ts";
import { compiledLines, summary } from "./ex_5_33.ts";

const ITERATIVE =
  "function factorial(n: number): number { function iter(product: number, counter: number): number { return counter > n ? product : iter(counter * product, counter + 1); } return iter(1, 1); }";

const executedStackIsPaired = (source: string): boolean => {
  const compiled = compileProgram(readProgram(source));
  if (isCompileError(compiled)) {
    throw new Error(`compilation failed: ${JSON.stringify(compiled)}`);
  }
  const machine = makeMachine<Word>({
    registers: ["val", "env", "argl", "proc", "continue", "entry", "thrown", "item"],
    operations: compiledOperations(),
    controller: compiled.instructions,
  });
  machine.writeRegister("env", new Session("core").globalEnv());
  machine.writeRegister("thrown", undefined);
  machine.writeRegister("continue", { tag: "symbol", name: "program-end" });
  const run = machine.run(1_000_000);
  if (run.error !== null) return false;
  const stack: string[] = [];
  for (const instruction of run.trace) {
    if (instruction.startsWith("save ")) {
      stack.push(instruction.slice(5));
    } else if (instruction.startsWith("restore ") && stack.pop() !== instruction.slice(8)) {
      return false;
    }
  }
  return stack.length === 0;
};

/** Exercise 5.34: the instruction stream has branch-specific restore
 * instructions, so pairing is checked over the executed factorial run,
 * not by comparing mutually exclusive static totals. */
export const ex_5_34 = (): readonly string[] => {
  const counts = summary(ITERATIVE);
  const lines = compiledLines(ITERATIVE);
  const paired = executedStackIsPaired(`${ITERATIVE}\nconsole.log(factorial(6));`);
  return [
    `statements: ${counts.statements}, saves: ${counts.saves}, restores: ${counts.restores}`,
    `every executed save is paired: ${paired}`,
    lines.some((line) => line.startsWith("goto ")) ? "tail calls compile to gotos" : "no goto",
  ];
};
