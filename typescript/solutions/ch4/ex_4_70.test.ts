// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { indexCounts } from "./ex_4_70.js";

describe("exercise 4.70: assertion storage without lazy cycles", () => {
  it("indexes the added fact and rule by relation", () => {
    expect(indexCounts()).toStrictEqual([1, 1]);
  });
});
