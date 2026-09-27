// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 1.2

import { describe, expect, it } from "vitest";
import { Random } from "../../../examples/ch1/random.js";

import {
  ackermann,
  countChange,
  expmod,
  factorialIter,
  factorialRecursive,
  fastExpt,
  fastPrime,
  fibIter,
  fibRecursive,
  gcd,
  isPrime,
  plusIterative,
  plusRecursive,
  sine,
} from "./02-processes.js";

describe("section 1.2: processes", () => {
  it("the two factorials land on the figure's 720", () => {
    expect(factorialRecursive(6)).toBe(720);
    expect(factorialIter(6)).toBe(720);
  });

  it("exercise 1.9's additions agree at (4, 5)", () => {
    expect(plusRecursive(4, 5)).toBe(9);
    expect(plusIterative(4, 5)).toBe(9);
  });

  it("exercise 1.10's Ackermann values are the printed ones", () => {
    expect(ackermann(1, 10)).toBe(1024);
    expect(ackermann(2, 4)).toBe(65536);
    expect(ackermann(3, 3)).toBe(65536);
  });

  it("the two Fibonaccis agree with the section's sequence", () => {
    expect(fibRecursive(10)).toBe(55);
    expect(fibIter(10)).toBe(55);
  });

  it("changing a dollar counts 292 ways", () => {
    expect(countChange(100)).toBe(292);
  });

  it("Euclid's Algorithm reduces gcd(206, 40) to 2", () => {
    expect(gcd(206, 40)).toBe(2);
  });

  it("fast exponentiation squares b^8 out of three multiplications", () => {
    expect(fastExpt(2, 8)).toBe(256);
    expect(fastExpt(3, 5)).toBe(243);
  });

  it("the sine reduction lands near the truth in five steps", () => {
    expect(sine(12.15)).toBeCloseTo(Math.sin(12.15), 2);
    expect(sine(12.15)).toBeCloseTo(-0.39980345741334, 12);
  });

  it("primality agrees with the small table", () => {
    expect(isPrime(2)).toBe(true);
    expect(isPrime(199)).toBe(true);
    expect(isPrime(19999)).toBe(false);
  });

  it("expmod stays inside the modulus", () => {
    expect(expmod(2, 1015, 561)).toBe(230);
    expect(expmod(3, 1_000_003, 1_000_003)).toBe(3);
  });

  it("the seeded Fermat test is reproducible: 1009 passes, 100 fails", () => {
    expect(fastPrime(1009, 3, new Random(1n))).toBe(true);
    expect(fastPrime(100, 3, new Random(1n))).toBe(false);
  });
});
