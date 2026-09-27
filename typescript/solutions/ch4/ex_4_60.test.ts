// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_4_60, neighborAnswers } from "./ex_4_60.js";

describe("exercise 4.60: lives-near", () => {
  it("produces both orientations of a nearby pair and permits canonical filtering", () => {
    const all = neighborAnswers();
    expect(all).toHaveLength(8);
    expect(all).toContain("(lives-near (Hacker Alyssa P) (Fect Cy D))");
    expect(all).toContain("(lives-near (Fect Cy D) (Hacker Alyssa P))");
    const uniqueOrientation = neighborAnswers(true);
    expect(uniqueOrientation).toHaveLength(4);
    expect(uniqueOrientation).toContain(
      "(and (lives-near (Fect Cy D) (Hacker Alyssa P)) (lisp-value < (Fect Cy D) (Hacker Alyssa P)))",
    );
  });

  it("explains symmetry and ordering", () => {
    expect(ex_4_60()).toContain("canonical ordering");
  });
});
