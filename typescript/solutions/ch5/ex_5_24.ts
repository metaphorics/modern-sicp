// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { makeEvaluator } from "../../packages/ch5/src/04-eceval.ts";

/** Exercise 5.24: switch as a basic form of the controller. The
 * controller evaluates the discriminant once, scans clause tests in order,
 * and runs the selected body plus the default path when no test matches.
 * The typed host subset rejects implicit fallthrough, so the example
 * spells out the later-case effect inside the matching case body. */
export const ex_5_24 = (): readonly string[] => {
  const program = [
    "function classify(n: number) {",
    "  switch (n) {",
    '    case 0: return "zero";',
    '    case 1: return "one";',
    '    default: return "many";',
    "  }",
    "}",
    "console.log(classify(0));",
    "console.log(classify(1));",
    "console.log(classify(7));",
    "function withFall(x: number) {",
    "  let total = 0;",
    "  switch (x) {",
    "    case 1: total = total + 1; total = total + 10; break;",
    "    case 2: total = total + 10;",
    "      break;",
    "    default: total = 100;",
    "  }",
    "  return total;",
    "}",
    "console.log(withFall(1));",
    "console.log(withFall(2));",
    "console.log(withFall(9));",
    "function once(x: number) {",
    "  let seen = 0;",
    "  switch (x) {",
    '    case seen: return "matched";',
    '    default: return "missed";',
    "  }",
    "}",
    "console.log(once(0));",
  ].join("\n");
  const result = makeEvaluator(program).run();
  return result.transcript;
};
