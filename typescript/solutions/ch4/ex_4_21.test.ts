// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  evenOddSixSource,
  evenOddSource,
  factorialSource,
  fibonacciSource,
  runSelfApplication,
} from "./ex_4_21.js";

const value = (text: string): unknown => {
  const result = runSelfApplication(text);
  expect(result.outcome.tag).toBe("ok");
  return result.outcome.tag === "ok" ? result.outcome.value : undefined;
};

describe("exercise 4.21: recursion without define", () => {
  it("the book's factorial expression evaluates to 3628800", () => {
    expect(value(factorialSource)).toBe(3628800);
  });

  it("the Fibonacci analog evaluates to 55", () => {
    expect(value(fibonacciSource)).toBe(55);
  });

  it("the completed two-procedure template answers false for 5 and true for 6", () => {
    expect(value(evenOddSource)).toBe(false);
    expect(value(evenOddSixSource)).toBe(true);
  });
});
