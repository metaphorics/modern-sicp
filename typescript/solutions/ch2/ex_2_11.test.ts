// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { mulIntervalFast, signOf } from "./ex_2_11.js";

/** The naive four-product multiplication, for the cross-check. */
const mulIntervalNaive = (
  x: { readonly lo: number; readonly hi: number },
  y: {
    readonly lo: number;
    readonly hi: number;
  },
): { readonly lo: number; readonly hi: number } => {
  const p1 = x.lo * y.lo;
  const p2 = x.lo * y.hi;
  const p3 = x.hi * y.lo;
  const p4 = x.hi * y.hi;
  return { lo: Math.min(p1, p2, p3, p4), hi: Math.max(p1, p2, p3, p4) };
};

describe("exercise 2.11", () => {
  it("the sign classifier splits the three classes", () => {
    expect(signOf({ lo: 2, hi: 6 })).toBe("positive");
    expect(signOf({ lo: -6, hi: -2 })).toBe("negative");
    expect(signOf({ lo: -2, hi: 6 })).toBe("span");
  });

  it("every case matches the naive multiplication across the sign grid", () => {
    const endpoints = [-5, -2, -0.5, 0, 0.5, 2, 5];
    for (const xlo of endpoints) {
      for (const xhi of endpoints) {
        for (const ylo of endpoints) {
          for (const yhi of endpoints) {
            if (xlo > xhi || ylo > yhi) {
              continue;
            }
            const x = { lo: xlo, hi: xhi };
            const y = { lo: ylo, hi: yhi };
            expect(mulIntervalFast(x, y)).toStrictEqual(mulIntervalNaive(x, y));
          }
        }
      }
    }
  });
});
