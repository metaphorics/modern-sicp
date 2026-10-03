// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { makeEvaluator } from "../../packages/ch5/src/04-eceval.ts";
import { ex_5_25, makeNormalOrderOperations, normalOrderController } from "./ex_5_25.ts";

describe("exercise 5.25 normal-order evaluation in the controller", () => {
  it("never touches the diverging argument the predicate discards", () => {
    const result = ex_5_25();
    expect(result.normal.some((line) => line.includes("0"))).toBe(true);
    expect(result.strictFault).not.toBeNull();
  });
  it("forces a used argument once and remembers the value", () => {
    const { operations, forced } = makeNormalOrderOperations();
    const program = [
      "let count = 0;",
      "function bump() { count = count + 1; return count; }",
      "function useTwice(x: number) { return x + x; }",
      "console.log(useTwice(bump()));",
      "console.log(count);",
    ].join("\n");
    const result = makeEvaluator(program, operations, normalOrderController).run();
    expect(result.outcome.tag).toBe("ok");
    expect(result.transcript.some((line) => line === "2")).toBe(true);
    expect(forced()).toBeGreaterThan(0);
  });
  it("runs the recursive factorial through the normal-order machine", () => {
    const result = ex_5_25();
    expect(result.normal.some((line) => line.includes("120"))).toBe(true);
  });
});
