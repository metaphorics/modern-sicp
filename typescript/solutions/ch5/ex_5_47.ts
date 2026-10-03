// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { makeEvaluator } from "../../packages/ch5/src/04-eceval.ts";
import { compileAndRun } from "../../packages/ch5/src/05-compilation.ts";

/** Exercise 5.47: compiled code calls interpreted procedures and the
 * other way round, and the two paths answer the same values. The
 * session is the book's mixed one; the pins are the agreement of the
 * two engines on every line. */
export const ex_5_47 = (): readonly string[] => {
  const program = [
    "function add(a: number, b: number) { return a + b; }",
    "function twice(f: (a: number, b: number) => number, x: number) { return f(f(x, x), f(x, x)); }",
    "twice(add, 3);",
  ].join("\n");
  const compiled = compileAndRun(program);
  const interpreted = makeEvaluator(program).run();
  for (let i = 0; i < Math.max(compiled.transcript.length, interpreted.transcript.length); i += 1) {
    if (compiled.transcript[i] !== interpreted.transcript[i]) {
      throw new Error(`the mixed session diverged at line ${i}`);
    }
  }
  return [...compiled.transcript, "compiled and interpreted agree on the mixed calls"];
};
