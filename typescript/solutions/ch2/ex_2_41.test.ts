// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { length, showList } from "../../packages/ch2/src/02-picture-language.js";
import { orderedTriples } from "./ex_2_41.js";

describe("exercise 2.41", () => {
  it("finds the 12 ordered triples of 1..5 summing to 10", () => {
    expect(length(orderedTriples(5, 10))).toBe(12);
  });

  it("keeps both order directions of a qualifying set", () => {
    const shown = showList(orderedTriples(5, 10));
    expect(shown.includes("(1 4 5)")).toBe(true);
    expect(shown.includes("(5 4 1)")).toBe(true);
  });
});
