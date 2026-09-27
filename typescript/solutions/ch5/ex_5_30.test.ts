// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { checkingFailureRows, runCheckingEvaluator } from "./ex_5_30.js";

describe("exercise 5.30 error signaling inside the evaluator", () => {
  it("prints each failure's detail through signal-error and returns to the driver loop", () => {
    expect(checkingFailureRows()).toEqual([
      { program: "(/ 1 0)", printed: "division by zero" },
      { program: "(car 5)", printed: "type error: car" },
      { program: "no-such-variable", printed: "unbound variable: no-such-variable" },
      { program: "(cons 1)", printed: "arity mismatch" },
      { program: "(5 6)", printed: "unknown-procedure-type-error" },
    ]);
  });
  it("the checks leave correct programs untouched", () => {
    const transcript = runCheckingEvaluator(
      "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n))) (factorial 5)",
    );
    expect(transcript.filter((line) => !line.startsWith(";;;"))).toEqual(["ok", "120"]);
  });
});
