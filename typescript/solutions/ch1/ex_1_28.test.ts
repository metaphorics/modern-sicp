// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { millerRabinWitness, mrExpmod } from "./ex_1_28.js";

const carmichael = [561, 1105, 1729, 2465, 2821, 6601];

describe("exercise 1.28", () => {
  it("known primes pass with the fixed witnesses", () => {
    for (const n of [13, 563, 1009, 1013, 1000003]) {
      for (const a of [2, 3, 5, 7]) {
        expect(millerRabinWitness(n, a)).toBe(true);
      }
    }
  });

  it("the signaling expmod returns 0 exactly on a nontrivial square root", () => {
    expect(mrExpmod(2, 560, 561)).toBe(0);
    expect(mrExpmod(2, 6600, 6601)).toBe(0);
    expect(mrExpmod(2, 1012, 1013)).toBe(1);
  });

  it("the Carmichael numbers are caught, witness after witness", () => {
    for (const n of carmichael) {
      for (const a of [2, 3, 5, 7]) {
        expect(millerRabinWitness(n, a)).toBe(false);
      }
    }
  });

  it("2047's witness 2 lies, witness 3 catches it", () => {
    expect(millerRabinWitness(2047, 2)).toBe(true);
    expect(millerRabinWitness(2047, 3)).toBe(false);
    expect(millerRabinWitness(100, 2)).toBe(false);
  });
});
