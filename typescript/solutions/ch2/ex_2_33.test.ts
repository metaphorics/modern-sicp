// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { list, showList } from "../../packages/ch2/src/02-picture-language.js";
import { appendViaAccumulate, lengthViaAccumulate, mapViaAccumulate } from "./ex_2_33.js";

describe("exercise 2.33", () => {
  it("map accumulates the consed results", () => {
    expect(showList(mapViaAccumulate((x) => x * x, list(1, 2, 3, 4)))).toBe("[1, 4, 9, 16]");
  });

  it("append accumulates seq1 onto seq2", () => {
    expect(showList(appendViaAccumulate(list(1, 2, 3), list(4, 5, 6)))).toBe("[1, 2, 3, 4, 5, 6]");
  });

  it("length accumulates a count", () => {
    expect(lengthViaAccumulate(list(1, 3, 5, 7))).toBe(4);
  });
});
