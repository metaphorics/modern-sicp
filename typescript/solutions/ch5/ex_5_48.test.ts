// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { makeEvaluator } from "../../packages/ch5/src/04-eceval.ts";
import { compileRunController, ex_5_48, makeCompileRunOperations } from "./ex_5_48.ts";

describe("exercise 5.48 compile-and-run inside the evaluator", () => {
  it("compiles and runs new code at run time", () => {
    const lines = ex_5_48();
    expect(lines.some((line) => line.includes("7"))).toBe(true);
    expect(lines.some((line) => line.includes("6"))).toBe(true);
  });
  it("the splice runs before the generic application path", () => {
    expect(
      compileRunController.some(
        (line) => line.tag === "test" && line.operation === "isCompileRunCall",
      ),
    ).toBe(true);
    const result = makeEvaluator(
      [
        'function compileAndRun(source: string): number { throw new Error("compileAndRun must be intercepted by the controller"); }',
        'console.log(compileAndRun("2 + 3;"));',
      ].join("\n"),
      makeCompileRunOperations(),
      compileRunController,
    ).run();
    expect(result.outcome.tag).toBe("ok");
  });
});
