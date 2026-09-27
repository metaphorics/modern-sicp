// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { ex_1_01 } from "./ex_1_01.js";

describe("exercise 1.1", () => {
  it("the sequence evaluates to the eleven transcript values, in order", () => {
    expect(ex_1_01()).toEqual([10, 12, 8, 3, 6, 19, false, 4, 16, 6, 16]);
  });
});
