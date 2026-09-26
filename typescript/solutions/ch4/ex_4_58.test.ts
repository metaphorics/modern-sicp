// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { bigShots, ex_4_58 } from "./ex_4_58.js";

describe("exercise 4.58: big shot", () => {
  it("includes division heads and the top executive, not same-division reports", () => {
    const answers = bigShots();
    expect(answers).toHaveLength(3);
    expect(answers.some((answer) => answer.includes("(Bitdiddle Ben)"))).toBe(true);
    expect(answers.some((answer) => answer.includes("(Scrooge Eben)"))).toBe(true);
    expect(answers.some((answer) => answer.includes("(Warbucks Oliver)"))).toBe(true);
  });

  it("explains the supervisor-in-division criterion", () => {
    expect(ex_4_58()).toContain("supervisor is absent");
  });
});
