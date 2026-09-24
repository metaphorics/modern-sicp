// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import fc from "fast-check";
import { describe, expect, it } from "vitest";

import { compose, type Mapper, repeated } from "./ex_0_02.js";

const inc: Mapper = (x) => x + 1;
const dbl: Mapper = (x) => 2 * x;

describe("exercise 0.2", () => {
  it("compose applies g and then f, in both orders", () => {
    expect(compose(inc, dbl)(3)).toBe(7);
    expect(compose(dbl, inc)(3)).toBe(8);
  });

  it("repeated applies f exactly n times, with n = 0 the identity", () => {
    expect(repeated(inc, 0)(41)).toBe(41);
    expect(repeated(inc, 5)(41)).toBe(46);
  });

  it("repeated composes f with itself n times, for generated inputs", () => {
    fc.assert(
      fc.property(
        fc.integer({ min: -1000, max: 1000 }),
        fc.integer({ min: 0, max: 50 }),
        (x, n) => {
          expect(repeated(inc, n)(x)).toBe(x + n);
          expect(repeated(dbl, n)(0)).toBe(0);
        },
      ),
    );
  });

  it("repeated of a twice-incrementer adds twice per step", () => {
    fc.assert(
      fc.property(
        fc.integer({ min: -1000, max: 1000 }),
        fc.integer({ min: 0, max: 20 }),
        (x, n) => {
          expect(repeated(compose(inc, inc), n)(x)).toBe(x + 2 * n);
        },
      ),
    );
  });
});
