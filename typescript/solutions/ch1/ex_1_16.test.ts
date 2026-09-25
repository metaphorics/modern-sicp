// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { fastExptIter, fastExptIterStates } from "./ex_1_16.js";

describe("exercise 1.16", () => {
  it("lands on the printed powers", () => {
    expect(fastExptIter(2, 10)).toBe(1024);
    expect(fastExptIter(3, 5)).toBe(243);
    expect(fastExptIter(5, 0)).toBe(1);
    expect(fastExptIter(2, 40)).toBe(1099511627776);
  });

  it("agrees with the section's recursive fast-expt over a grid", () => {
    const isEven = (n: number): boolean => n % 2 === 0;
    const square = (x: number): number => x * x;
    const fastExpt = (b: number, n: number): number =>
      n === 0 ? 1 : isEven(n) ? square(fastExpt(b, n / 2)) : b * fastExpt(b, n - 1);
    for (let b = 2; b <= 5; b += 1) {
      for (let n = 0; n <= 20; n += 1) {
        expect(fastExptIter(b, n)).toBe(fastExpt(b, n));
      }
    }
  });

  it("a * b^n is the same number at every state", () => {
    const states = fastExptIterStates(2, 10);
    const products = states.map((s) => s.a * s.b ** s.n);
    expect(products).toStrictEqual([1024, 1024, 1024, 1024, 1024, 1024]);
    expect(states.length).toBe(6);
  });
});
