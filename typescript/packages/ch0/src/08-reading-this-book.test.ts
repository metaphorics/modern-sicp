// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { arithmeticSession, squareSession } from "./08-reading-this-book.js";

describe("reading this book", () => {
  it("the arithmetic session evaluates in order", () => {
    expect(arithmeticSession).toEqual([486, 100, 12, 1, 6]);
  });

  it("the square session evaluates in order", () => {
    expect(squareSession()).toEqual([441, 49, 81]);
  });
});
