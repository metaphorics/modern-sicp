// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { makeEvaluator } from "../../packages/ch5/src/04-eceval.ts";
import { derivedSwitchController, ex_5_23, makeSwitchTransformer } from "./ex_5_23.ts";

describe("exercise 5.23 derived expressions via transformer operations", () => {
  it("the transformed session answers the same values the basic form answers", () => {
    const lines = ex_5_23();
    expect(lines.some((line) => line.includes("zero"))).toBe(true);
    expect(lines.some((line) => line.includes("one"))).toBe(true);
    expect(lines.some((line) => line.includes("many"))).toBe(true);
  });
  it("one transformation carries each switch", () => {
    const lines = ex_5_23();
    expect(lines[lines.length - 1]).toBe("transformations: 3");
  });
  it("a non-returning clause is left to the basic form", () => {
    const { operations } = makeSwitchTransformer();
    const program = [
      "function f(x: number) {",
      "  let total = 0;",
      "  switch (x) {",
      "    case 1: total = 1; break;",
      "    default: total = 9;",
      "  }",
      "  return total;",
      "}",
      "f(1);",
    ].join("\n");
    const variant = makeEvaluator(program, operations, derivedSwitchController);
    const result = variant.run();
    expect(result.outcome.tag).toBe("ok");
  });
});
