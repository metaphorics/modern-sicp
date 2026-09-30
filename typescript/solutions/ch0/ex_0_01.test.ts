// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { ex_0_01 } from "./ex_0_01.js";

describe("exercise 0.1", () => {
  it("the three sessions evaluate to the stated values in order", () => {
    expect(ex_0_01()).toEqual([486, 100, 12, 1, 6, 19, 4, 16, 6, 16, 441, 49, 81]);
  });
});
