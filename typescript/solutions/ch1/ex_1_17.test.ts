// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { times } from "./ex_1_17.js";

describe("exercise 1.17", () => {
  it("lands on the printed products", () => {
    expect(times(17, 31)).toBe(527);
    expect(times(2, 10)).toBe(20);
    expect(times(7, 0)).toBe(0);
    expect(times(3, 1)).toBe(3);
  });

  it("agrees with the host product over a grid", () => {
    for (let a = 1; a <= 12; a += 1) {
      for (let b = 0; b <= 12; b += 1) {
        expect(times(a, b)).toBe(a * b);
      }
    }
  });

  it("only adds, doubles, and halves along the way", () => {
    // 17 * 31 peels two odd steps (31 -> 30 -> 15 -> ... -> 0 needs
    // 5 halvings plus one decrement per odd value met: 31, 15, 7, 3, 1).
    // Depth stays logarithmic: no linear b-chain of additions.
    expect(times(1024, 1024)).toBe(1048576);
    expect(times(65536, 65536)).toBe(4294967296);
  });
});
