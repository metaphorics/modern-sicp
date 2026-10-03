// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { answers } from "./ex_4_30.js";

describe("exercise 4.30: forcing in eval-sequence", () => {
  it("pins the book's sessions under both sequence rules", () => {
    const observed = answers();
    expect(observed.forEachText).toEqual(["57", "321", "88", "done"]);
    expect(observed.forEachCy).toEqual(observed.forEachText);
    expect(observed.p1Text).toEqual(["[1, 2]"]);
    expect(observed.p1Cy).toEqual(observed.p1Text);
    expect(observed.p2Text).toEqual(["1"]);
    expect(observed.p2Cy).toEqual(["[1, 2]"]);
  });
});
