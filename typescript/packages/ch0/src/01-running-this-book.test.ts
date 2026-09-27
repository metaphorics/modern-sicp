// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { square } from "./01-running-this-book.js";

describe("the tour listing", () => {
  it("the call returns the value the prose annotates", () => {
    expect(square(21)).toBe(441);
  });
});
