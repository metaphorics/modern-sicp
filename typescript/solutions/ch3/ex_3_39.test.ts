// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { exercise39Outcomes } from "./ex_3_39.js";

describe("exercise 3.39: which serialized outcomes remain", () => {
  it("three of the five possibilities remain: 100, 101, and 121", () => {
    expect(exercise39Outcomes()).toEqual([100, 101, 121]);
  });

  it("110 and 11 are gone because the serializer bars the increment from the product's reads", () => {
    const outcomes = exercise39Outcomes();
    expect(outcomes).not.toContain(110);
    expect(outcomes).not.toContain(11);
  });
});
