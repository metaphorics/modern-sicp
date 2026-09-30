// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { solutions } from "./ex_4_39.js";

describe("exercise 4.39: the order of the restrictions", () => {
  it("both orders answer the same single assignment", () => {
    const book = solutions("book");
    const reordered = solutions("reordered");
    expect(book).toEqual(["{ baker: 3, cooper: 2, fletcher: 4, miller: 5, smith: 1 }"]);
    expect(reordered).toEqual(book);
  });
});
