// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { answers } from "./ex_4_27.js";

describe("exercise 4.27: lazy identity with set!", () => {
  it("pins the book's sequence: 1, 10, 2, then an unchanging re-display", () => {
    const observed = answers();
    expect(observed.outcome.tag).toBe("ok");
    expect(observed.transcript).toEqual(["1", "10", "2", "10", "2"]);
  });
});
