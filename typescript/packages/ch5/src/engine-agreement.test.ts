// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: sections 4.1.7, 5.4, and 5.5

import { runAnalyzedSource, runSource } from "@sicp-ts/ch4/01-metacircular";
import { describe, expect, it } from "vitest";
import { runEvaluator } from "./04-eceval.ts";
import { compileAndRun } from "./05-compilation.ts";

const engines = {
  direct: runSource,
  analyzed: runAnalyzedSource,
  eceval: runEvaluator,
  compiled: compileAndRun,
} as const;

/** Each program's transcript as the native Node run prints it. */
const programs: ReadonlyArray<readonly [string, string, ReadonlyArray<string>]> = [
  [
    "division after an identifier is not a regex",
    "const x = 9;\nconst n = 3;\nconsole.log(x / n / 3);\n",
    ["1"],
  ],
  [
    "a parenthesized type may be an arrow's return type",
    "type Tree = { readonly v: number };\nconst f = (t: Tree | null): (Tree | null) => t;\nconst g = (h: (x: number) => number): ((x: number) => number) => (x: number): number => h(x) + 1;\nconsole.log(f(null) === null);\nconsole.log(g((x: number): number => x * 2)(5));\n",
    ["true", "11"],
  ],
  [
    "console.log and member calls run in every engine",
    'console.log("abc".padEnd(5, "."));\n',
    ["abc.."],
  ],
  [
    "a template operand keeps the left operand of +",
    'const v = 2.5;\nconsole.log("x" + `${v}` + "y" + `${v * v}`);\n',
    ["x2.5y6.25"],
  ],
  [
    "an object literal operand keeps the left operand of +",
    "console.log(10 + { a: 2 }.a);\n",
    ["12"],
  ],
  [
    "a bound console shadows the log builtin",
    'const call = (console: { log: (x: string) => string }): string =>\n  console.log("inner");\nconsole.log(call({ log: (x: string): string => "S:" + x }));\n',
    ["S:inner"],
  ],
  [
    "nested loops and a switch inside a loop keep the outer loop's state",
    'let total = 0;\nfor (const a of [1, 2]) {\n  for (const b of [10, 20, 30]) {\n    total = total + a * b;\n  }\n}\nconsole.log(total);\nfor (const s of ["x", "y"]) {\n  switch (s) {\n    case "x":\n      console.log("ex");\n      break;\n    default:\n      console.log("other");\n  }\n}\n',
    ["180", "ex", "other"],
  ],
];

describe("the four teaching engines agree with native output", () => {
  for (const [name, source, expected] of programs) {
    for (const [engine, run] of Object.entries(engines)) {
      it(`${name} (${engine})`, () => {
        const result = run(source);
        expect(result.outcome.tag).toBe("ok");
        expect(result.transcript).toEqual(expected);
      });
    }
  }

  it("the explicit-control evaluator finishes 5000-deep non-tail recursion within its step bound", () => {
    const result = runEvaluator(
      "function count(n: number): number {\n  return n === 0 ? 0 : 1 + count(n - 1);\n}\nconsole.log(count(5000));\n",
    );
    expect(result.outcome.tag).toBe("ok");
    expect(result.transcript).toEqual(["5000"]);
  });
});
