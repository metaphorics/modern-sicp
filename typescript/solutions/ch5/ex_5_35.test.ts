// SPDX-License-Identifier: GPL-3.0-only
import { describe, expect, it } from "vitest";
import { ex_5_35 } from "./ex_5_35.js";

describe("exercise 5.35", () => {
  it("reproduces Figure 5.18 with the counter seeded at 14", () => {
    const answers = ex_5_35();
    expect(answers[0]).toBe("expression: (define (f x) (+ x (g (+ x 2))))");
    expect(answers[1]).toContain("Figure 5.18 reproduced");
    expect(answers[2]).toContain(
      "(assign val (op make-compiled-procedure) (label entry16) (reg env))",
    );
    expect(answers[2]).toContain("after-call23");
  });
});
