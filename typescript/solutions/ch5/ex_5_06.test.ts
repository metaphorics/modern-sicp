// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_06 } from "./ex_5_06.ts";

describe("exercise 5.6 redundant Fibonacci stack operations", () => {
  it("preserves the answer and removes one save/restore pair", () => {
    const result = ex_5_06(6);
    expect(result.original.value).toBe(8);
    expect(result.reduced.value).toBe(8);
    expect(result.original.instructionCount).toBe(281);
    expect(result.reduced.instructionCount).toBe(257);
    expect(result.original.maxDepth).toBe(10);
    expect(result.reduced.maxDepth).toBe(10);
    expect(result.original.events.filter((event) => event.tag === "save")).toHaveLength(48);
    expect(result.original.events.filter((event) => event.tag === "restore")).toHaveLength(48);
    expect(result.reduced.events.filter((event) => event.tag === "save")).toHaveLength(36);
    expect(result.reduced.events.filter((event) => event.tag === "restore")).toHaveLength(36);
  });
  it("the reduced controller keeps every restore paired", () => {
    const result = ex_5_06(6);
    for (const event of result.reduced.events) {
      if (event.tag === "restore") expect(event.matchedSave).not.toBeNull();
    }
  });
});
