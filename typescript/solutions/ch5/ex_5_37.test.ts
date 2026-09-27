// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_37 } from "./ex_5_37.js";

describe("exercise 5.37", () => {
  it("runs the blind code and shows the wasted stack", () => {
    const answers = ex_5_37();
    expect(Number(answers[1]?.split(": ")[1]?.split(" ")[0])).toBeGreaterThan(
      Number(answers[0]?.split(": ")[1]?.split(" ")[0]),
    );
    expect(answers[3]).toContain("both answer 120");
  });
});
