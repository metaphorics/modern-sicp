// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { formatValue } from "../../packages/ch5/src/04-eceval.ts";
import { compileAndRun } from "../../packages/ch5/src/05-compilation.ts";

/** Exercise 5.49: the read-compile-execute-print loop. Each input form
 * is read by the shared reader, compiled by the compiler, executed on
 * the machine, and printed; the loop is the machine's driver over the
 * compiler's own entry point. */
export const readCompileExecutePrint = (inputs: readonly string[]): readonly string[] => {
  const transcript: string[] = [];
  for (const input of inputs) {
    const run = compileAndRun(input);
    if (run.outcome.tag !== "ok") {
      transcript.push(`Error: ${JSON.stringify(run.outcome.error)}`);
      continue;
    }
    transcript.push(formatValue(run.outcome.value));
  }
  return transcript;
};
/** The loop's own session: two inputs, each compiled and run before the
 * next is read. */
export const ex_5_49 = (): readonly string[] =>
  readCompileExecutePrint(["1 + 1;", "function square(x: number) { return x * x; }\nsquare(6);"]);
