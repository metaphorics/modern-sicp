// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { accumulate, accumulateIter, productViaAccumulate, sumViaAccumulate } from "./ex_1_32.js";

const inc = (x: number): number => x + 1;

describe("exercise 1.32", () => {
  it("sum and product are simple calls to accumulate", () => {
    expect(sumViaAccumulate((x) => x, 1, inc, 10)).toBe(55);
    expect(productViaAccumulate((x) => x, 1, inc, 10)).toBe(3628800);
    expect(sumViaAccumulate((x) => x * x * x, 1, inc, 10)).toBe(3025);
  });

  it("the two shapes agree over a grid of ranges", () => {
    // Ranges short enough that every partial sum and product is an exact
    // integer below 2^53, where the two fold orders cannot disagree.
    for (let a = 1; a <= 6; a += 1) {
      for (let b = a; b <= a + 10; b += 1) {
        expect(
          accumulate(
            (x, y) => x + y,
            0,
            (x) => x * x,
            a,
            inc,
            b,
          ),
        ).toBe(
          accumulateIter(
            (x, y) => x + y,
            0,
            (x) => x * x,
            a,
            inc,
            b,
          ),
        );
        expect(
          accumulate(
            (x, y) => x * y,
            1,
            (x) => x + 1,
            a,
            inc,
            b,
          ),
        ).toBe(
          accumulateIter(
            (x, y) => x * y,
            1,
            (x) => x + 1,
            a,
            inc,
            b,
          ),
        );
      }
    }
  });

  it("an empty range yields the null value of the combiner", () => {
    expect(
      accumulate(
        (x, y) => x + y,
        0,
        (x) => x,
        5,
        inc,
        4,
      ),
    ).toBe(0);
    expect(
      accumulate(
        (x, y) => x * y,
        1,
        (x) => x,
        5,
        inc,
        4,
      ),
    ).toBe(1);
  });
});
