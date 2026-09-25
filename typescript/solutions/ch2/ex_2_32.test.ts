// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { length, list, showList } from "../../packages/ch2/src/02-picture-language.js";
import { subsets } from "./ex_2_32.js";

describe("exercise 2.32", () => {
  it("produces the subsets in the book's doubling order", () => {
    expect(showList(subsets(list(1, 2, 3)))).toBe("(() (3) (2) (2 3) (1) (1 3) (1 2) (1 2 3))");
  });

  it("a three-element set has all 2^3 subsets", () => {
    expect(length(subsets(list(1, 2, 3)))).toBe(8);
  });
});
