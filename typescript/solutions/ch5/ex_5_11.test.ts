// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { restoreDisciplineRuns } from "./ex_5_11.js";

describe("exercise 5.11 the three save and restore disciplines", () => {
  it("runs the disciplines on the same machines and the elimination", () => {
    expect(restoreDisciplineRuns()).toEqual([
      "out-of-order restore under (a) name-blind: y = 2",
      "out-of-order restore under (b) checking: restore y but the stack holds x",
      "out-of-order restore under (c) per-register: y = 1",
      "fib(3) under (a): val = 2",
      "fib(3) under (b): val = 2",
      "fib(3) under (c): val = 2",
      "fib(3) with the eliminated assign, discipline (a): val = 2",
      "fib(5) with the eliminated assign, discipline (a): val = 5",
      "the eliminated controller under (b): restore n but the stack holds val",
    ]);
  });
});
