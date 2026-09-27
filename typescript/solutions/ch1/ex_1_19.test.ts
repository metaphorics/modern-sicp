// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { fibLog } from "./ex_1_19.js";

describe("exercise 1.19", () => {
  it("agrees with the linear iteration while number is still exact", () => {
    const fibIterNumber = (n: number): number => {
      let a = 1;
      let b = 0;
      for (let k = 0; k < n; k += 1) {
        const next = a + b;
        a = b;
        b = next;
      }
      return b;
    };
    for (let n = 0; n <= 78; n += 1) {
      expect(fibLog(n)).toBe(BigInt(fibIterNumber(n)));
    }
    expect(fibLog(10)).toBe(55n);
    expect(fibLog(50)).toBe(12586269025n);
  });

  it("stays exact past 2^53, where number goes wrong", () => {
    expect(fibLog(79)).toBe(14472334024676221n);
    expect(fibLog(92)).toBe(7540113804746346429n);
    expect(fibLog(93)).toBe(12200160415121876738n);
    expect(fibLog(100)).toBe(354224848179261915075n);
    const fibIterNumber = (n: number): number => {
      let a = 1;
      let b = 0;
      for (let k = 0; k < n; k += 1) {
        const next = a + b;
        a = b;
        b = next;
      }
      return b;
    };
    expect(fibIterNumber(78)).toBe(8944394323791464);
    expect(fibIterNumber(79)).toBe(14472334024676220);
    expect(fibLog(79)).toBe(14472334024676221n);
  });
});
