// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { listDatum, numDatum, symDatum } from "../../packages/ch2/src/03-symbolic-data.js";
import { equalQ } from "./ex_2_54.js";

describe("exercise 2.54", () => {
  it("agrees with the book's two examples", () => {
    const a = listDatum(symDatum("this"), symDatum("is"), symDatum("a"), symDatum("list"));
    expect(
      equalQ(a, listDatum(symDatum("this"), symDatum("is"), symDatum("a"), symDatum("list"))),
    ).toBe(true);
    expect(
      equalQ(
        a,
        listDatum(symDatum("this"), listDatum(symDatum("is"), symDatum("a")), symDatum("list")),
      ),
    ).toBe(false);
  });

  it("compares numbers numerically and recursion depth structurally", () => {
    expect(equalQ(numDatum(2), numDatum(2))).toBe(true);
    expect(equalQ(numDatum(2), numDatum(3))).toBe(false);
    expect(equalQ(numDatum(2), symDatum("2"))).toBe(false);
    expect(
      equalQ(
        listDatum(listDatum(numDatum(1), numDatum(2)), numDatum(3)),
        listDatum(listDatum(numDatum(1), numDatum(2)), numDatum(3)),
      ),
    ).toBe(true);
    expect(equalQ(listDatum(), listDatum())).toBe(true);
    expect(equalQ(listDatum(), listDatum(numDatum(0)))).toBe(false);
  });
});
