// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { list, showList, showTree } from "../../packages/ch2/src/02-picture-language.js";

import { squareListIter, squareListSwapped } from "./ex_2_22.js";

describe("exercise 2.22", () => {
  it("the while-loop rewrite conses onto the front, so the order flips", () => {
    expect(showList(squareListIter(list(1, 2, 3, 4)))).toBe("(16 9 4 1)");
  });

  it("the swapped rewrite wraps the answer around each square", () => {
    expect(showTree(squareListSwapped(list(1, 2, 3, 4)))).toBe("((((() 1) 4) 9) 16)");
  });
});
