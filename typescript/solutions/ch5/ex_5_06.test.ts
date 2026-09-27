// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_06 } from "./ex_5_06.js";

describe("exercise 5.6 redundant Fibonacci stack operations", () => {
  it("preserves the answer and removes one save/restore pair", () => {
    const result = ex_5_06(6);
    expect(result.original.ok).toBe(true);
    expect(result.reduced.ok).toBe(true);
    if (!result.original.ok || !result.reduced.ok) return;
    expect(result.original.value.registers["val"]).toBe(8);
    expect(result.reduced.value.registers["val"]).toBe(8);
    expect(result.original.value.instructions).toBe(281);
    expect(result.reduced.value.instructions).toBe(257);
    expect(result.original.value.maxDepth).toBe(10);
    expect(result.reduced.value.maxDepth).toBe(10);
    expect(result.original.value.events.filter((event) => event.tag === "save")).toHaveLength(48);
    expect(result.original.value.events.filter((event) => event.tag === "restore")).toHaveLength(
      48,
    );
    expect(result.reduced.value.events.filter((event) => event.tag === "save")).toHaveLength(36);
    expect(result.reduced.value.events.filter((event) => event.tag === "restore")).toHaveLength(36);
  });
});
