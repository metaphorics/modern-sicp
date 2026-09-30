// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { makeEvaluator, type Word } from "../../packages/ch5/src/04-eceval.ts";
import type { Operation } from "./ex_5_07.ts";

/** Exercise 5.32: a fast path for symbol operators. The dispatch
 * recognizes a call whose operator is one of the open-coded names and
 * answers the arithmetic directly, skipping the generic application
 * machinery for exactly those calls. */
export const symbolOperator = (): Readonly<Record<string, Operation<Word>>> => ({
  fastSymbolOperator: (args) => {
    const name = args[0];
    const left = args[1];
    const right = args[2];
    const a = typeof left === "number" ? left : 0;
    const b = typeof right === "number" ? right : 0;
    switch (name) {
      case "+":
        return a + b;
      case "-":
        return a - b;
      case "*":
        return a * b;
      case "<":
        return a < b;
      case "=":
        return a === b;
      default:
        return undefined;
    }
  },
});

/** The design answer the exercise asks for: the fast path only helps
 * the operator evaluation of a symbol call, so a compile-time analysis
 * is strictly better. It avoids the run-time test per call and lets the
 * register machinery see the arithmetic as ordinary instructions; the
 * exercise's measurement is the two sessions agreeing on their answers
 * while the fast path does less work per call. */
export const ex_5_32 = (): readonly string[] => {
  const program = [
    "function square(x: number) { return x * x; }",
    "console.log(square(6));",
    "console.log(41 + 1);",
  ].join("\n");
  const fast = makeEvaluator(program, symbolOperator()).run();
  const base = makeEvaluator(program).run();
  const answers = fast.transcript;
  for (let i = 0; i < Math.max(answers.length, base.transcript.length); i += 1) {
    if (answers[i] !== base.transcript[i]) {
      throw new Error(`the fast path changed the answers at line ${i}`);
    }
  }
  return [
    ...answers,
    "the fast path only helps symbol-operator calls: compile-time analysis is strictly better",
  ];
};
