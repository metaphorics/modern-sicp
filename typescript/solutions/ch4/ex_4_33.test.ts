// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { answers } from "./ex_4_33.js";

describe("exercise 4.33: list literals produce lazy lists", () => {
  it("the lifted literal answers 1, 2, and 4 through the lazy procedures", () => {
    const observed = answers();
    expect(observed.lifted).toEqual(["1", "2", "4"]);
  });

  it("the plain structure through the lazy procedures fails", () => {
    const observed = answers();
    expect(observed.plainOutcome).toBe("error");
  });
});
