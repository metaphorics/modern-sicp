// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { carmichaelNumbers, expmod, passesFermatForAllA } from "./ex_1_27.js";

describe("exercise 1.27", () => {
  it("all six Carmichael numbers pass the congruence for every a below n", () => {
    for (const n of carmichaelNumbers) {
      expect(passesFermatForAllA(n)).toBe(true);
    }
  });

  it("they are nonetheless composite, and the primes pass for honest reasons", () => {
    for (const n of carmichaelNumbers) {
      const smallestDivisor = (x: number): number => {
        for (let d = 2; d * d <= x; d += 1) {
          if (x % d === 0) {
            return d;
          }
        }
        return x;
      };
      expect(smallestDivisor(n)).not.toBe(n);
    }
    expect(passesFermatForAllA(13)).toBe(true);
    expect(passesFermatForAllA(1009)).toBe(true);
  });

  it("an ordinary composite fails the very first witness that tries", () => {
    expect(passesFermatForAllA(100)).toBe(false);
    expect(expmod(2, 100, 100)).not.toBe(2);
  });
});
