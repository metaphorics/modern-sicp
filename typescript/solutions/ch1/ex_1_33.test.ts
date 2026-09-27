// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  filteredAccumulate,
  gcd,
  isPrime,
  productOfRelativelyPrimes,
  sumOfSquaresOfPrimes,
} from "./ex_1_33.js";

describe("exercise 1.33", () => {
  it("sums the squares of the primes in the range", () => {
    expect(sumOfSquaresOfPrimes(2, 10)).toBe(87);
    expect(sumOfSquaresOfPrimes(2, 20)).toBe(1027);
    expect(sumOfSquaresOfPrimes(14, 16)).toBe(0);
  });

  it("multiplies the integers below n that are relatively prime to n", () => {
    expect(productOfRelativelyPrimes(10)).toBe(189);
    expect(productOfRelativelyPrimes(12)).toBe(385);
    expect(productOfRelativelyPrimes(2)).toBe(1);
  });

  it("the filter leaves a plain accumulate untouched when everything passes", () => {
    const everything = (): boolean => true;
    expect(
      filteredAccumulate(
        (x, y) => x + y,
        0,
        (x) => x,
        1,
        (x) => x + 1,
        10,
        everything,
      ),
    ).toBe(55);
  });

  it("the restated section tools behave as in 1.2", () => {
    expect(isPrime(2)).toBe(true);
    expect(isPrime(997)).toBe(true);
    expect(isPrime(1)).toBe(false);
    expect(gcd(206, 40)).toBe(2);
  });
});
