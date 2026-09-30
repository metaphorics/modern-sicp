// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import type { List, Showable } from "../../packages/ch2/src/02-picture-language.js";
import { accumulate, list, nil, showList } from "../../packages/ch2/src/02-picture-language.js";
import { foldLeft } from "./ex_2_38.js";

describe("exercise 2.38", () => {
  it("division folds right to 3/2", () => {
    expect(accumulate((a, b) => a / b, 1, list(1, 2, 3))).toBe(1.5);
  });

  it("division folds left to 1/6", () => {
    expect(foldLeft((a, b) => a / b, 1, list(1, 2, 3))).toBe(1 / 6);
  });

  it("the pairing operation nests right under fold-right", () => {
    const nest = (x: number, y: List<Showable>): List<Showable> =>
      list<number | List<Showable>>(x, y);
    expect(showList(accumulate<number, List<Showable>>(nest, nil, list(1, 2, 3)))).toBe(
      "[1, [2, [3, []]]]",
    );
  });

  it("the pairing operation nests left under fold-left", () => {
    const nest = (x: List<Showable>, y: number): List<Showable> =>
      list<List<Showable> | number>(x, y);
    expect(showList(foldLeft<number, List<Showable>>(nest, nil, list(1, 2, 3)))).toBe(
      "[[[[], 1], 2], 3]",
    );
  });
});
