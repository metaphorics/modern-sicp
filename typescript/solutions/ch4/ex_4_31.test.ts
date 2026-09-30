// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { answers } from "./ex_4_31.js";

describe("exercise 4.31: lazy and lazy-memo parameter declarations", () => {
  it("pins the declared session: taken, the f run, and the two probes", () => {
    const observed = answers().transcript;
    expect(observed).toStrictEqual([
      "taken",
      "[1, 5, 5, 4, 30, 30]",
      "5",
      "[10, 10]",
      "7",
      "[10, 10]",
      "8",
    ]);
  });
});
