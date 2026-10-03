// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_05 } from "./ex_5_05.ts";

describe("exercise 5.5 recursive machine hand traces", () => {
  it("pins the executed factorial and Fibonacci stack traces", () => {
    const result = ex_5_05(4, 4);
    const fact = result.factorial;
    expect(fact.value).toBe(24);
    expect(fact.instructionCount).toBe(38);
    expect(
      fact.events.map((event) => [
        event.tag,
        event.step,
        event.reg,
        event.tag === "restore" ? event.matchedSave : null,
      ]),
    ).toEqual([
      ["save", 3, "continue", null],
      ["save", 4, "n", null],
      ["save", 10, "continue", null],
      ["save", 11, "n", null],
      ["save", 17, "continue", null],
      ["save", 18, "n", null],
      ["restore", 26, "n", 18],
      ["restore", 27, "continue", 17],
      ["restore", 30, "n", 11],
      ["restore", 31, "continue", 10],
      ["restore", 34, "n", 4],
      ["restore", 35, "continue", 3],
    ]);
    const fib = result.fibonacci;
    expect(fib.value).toBe(3);
    expect(fib.instructionCount).toBe(97);
    expect(fib.maxDepth).toBe(6);
    expect(fib.events).toHaveLength(32);
    expect(
      fib.events.filter((event) => event.tag === "restore").map((event) => event.matchedSave),
    ).toEqual([19, 17, 31, 29, 12, 10, 47, 45, 5, 3, 69, 67, 81, 79, 63, 61]);
  });
  it("every restore is paired with its save, so the nesting holds", () => {
    const result = ex_5_05(4, 4);
    for (const trace of [result.factorial, result.fibonacci]) {
      for (const event of trace.events) {
        if (event.tag === "restore") expect(event.matchedSave).not.toBeNull();
      }
    }
  });
});
