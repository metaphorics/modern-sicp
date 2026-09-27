// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_4_66, wheelAccumulationCounts } from "./ex_4_66.js";

describe("exercise 4.66: accumulation over proof frames", () => {
  it("distinguishes derivation frames from distinct values", () => {
    expect(wheelAccumulationCounts()).toStrictEqual({ proofCount: 5, distinctAnswerCount: 2 });
  });

  it("describes why raw accumulation overcounts", () => {
    expect(ex_4_66()).toContain("one frame per proof");
    expect(ex_4_66()).toContain("deduplicate");
  });
});
