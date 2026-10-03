// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { solutions } from "./ex_4_44.js";

describe("exercise 4.44: queens under the search experiment", () => {
  it("the 4x4 board answers its two solutions in search order", () => {
    expect(solutions(4)).toEqual(["[3, 1, 4, 2]", "[2, 4, 1, 3]"]);
  });

  it("the 6x6 board answers four solutions beginning [5, 3, 1, 6, 4, 2]", () => {
    const boards = solutions(6);
    expect(boards).toHaveLength(4);
    expect(boards[0]).toBe("[5, 3, 1, 6, 4, 2]");
  });

  it("the 8x8 board answers 92 solutions, the first [4, 2, 7, 3, 6, 8, 5, 1]", () => {
    const boards = solutions(8);
    expect(boards).toHaveLength(92);
    expect(boards[0]).toBe("[4, 2, 7, 3, 6, 8, 5, 1]");
  });
});
