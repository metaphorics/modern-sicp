// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { solutions } from "./ex_4_43.js";

describe("exercise 4.43: the yacht puzzle", () => {
  it("told that Mary Ann is Moore's, Lorna's father is Downing", () => {
    expect(solutions(true)).toEqual(['{ lornasFather: "downing" }']);
  });

  it("not told, Downing and Parker both work", () => {
    expect(solutions(false)).toEqual(['{ lornasFather: "downing" }', '{ lornasFather: "parker" }']);
  });
});
