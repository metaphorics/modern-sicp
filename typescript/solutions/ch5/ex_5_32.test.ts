// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_32 } from "./ex_5_32.js";

describe("exercise 5.32", () => {
  it("runs symbol and compound operators and beats the base push count", () => {
    const answers = ex_5_32();
    expect(answers[0]).toContain("36");
    expect(answers[0]).toContain("42");
    const base = Number(answers[1]?.split(": ")[1]?.split("/")[0]);
    const fast = Number(answers[2]?.split(": ")[1]?.split("/")[0]);
    expect(fast).toBeLessThan(base);
    expect(answers[3]).toContain("compile time");
  });
});
