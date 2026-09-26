// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { tracedRegisterGcdLog } from "./ex_5_18.js";

describe("exercise 5.18 traced registers", () => {
  it("reports every store to a traced register, old and new", () => {
    expect(tracedRegisterGcdLog()).toEqual([
      "a: *unassigned* -> 12",
      "b: *unassigned* -> 8",
      "t: *unassigned* -> 4",
      "a: 12 -> 8",
      "b: 8 -> 4",
      "t: 4 -> 0",
      "a: 8 -> 4",
      "b: 4 -> 0",
      "gcd(12, 8) = 4",
    ]);
  });
});
