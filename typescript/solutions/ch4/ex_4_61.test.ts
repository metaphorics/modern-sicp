// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { nextToAnswers } from "./ex_4_61.js";

describe("exercise 4.61: next-to as atoms", () => {
  it("finds both adjacent pairs of the mixed list", () => {
    const [mixed] = nextToAnswers();
    expect(mixed).toStrictEqual([
      'next-to("1", ["2", "3"], ["1", ["2", "3"], "4"])',
      'next-to(["2", "3"], "4", ["1", ["2", "3"], "4"])',
    ]);
  });

  it("finds both predecessors of 1", () => {
    const [, before] = nextToAnswers();
    expect(before).toStrictEqual([
      'next-to("2", "1", ["2", "1", "3", "1"])',
      'next-to("3", "1", ["2", "1", "3", "1"])',
    ]);
  });
});
