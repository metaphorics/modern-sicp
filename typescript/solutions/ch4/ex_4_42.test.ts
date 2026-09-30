// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { solutions } from "./ex_4_42.js";

describe("exercise 4.42: the Liars puzzle", () => {
  it("the unique assignment is the book's, in search order", () => {
    expect(solutions()).toEqual(["{ betty: 3, ethel: 5, joan: 2, kitty: 1, mary: 4 }"]);
  });
});
