// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_4_65, wheelAnswers } from "./ex_4_65.js";

describe("exercise 4.65: wheel proof multiplicity", () => {
  it("preserves all five proofs in query order", () => {
    expect(wheelAnswers()).toStrictEqual([
      "(wheel (Bitdiddle Ben))",
      "(wheel (Warbucks Oliver))",
      "(wheel (Warbucks Oliver))",
      "(wheel (Warbucks Oliver))",
      "(wheel (Warbucks Oliver))",
    ]);
  });

  it("counts Oliver's four proofs rather than collapsing them", () => {
    expect(ex_4_65()).toContain("produced 4 times");
  });
});
