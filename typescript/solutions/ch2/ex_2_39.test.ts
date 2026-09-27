// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { list, showList } from "../../packages/ch2/src/02-picture-language.js";
import { reverseViaFoldLeft, reverseViaFoldRight } from "./ex_2_39.js";

describe("exercise 2.39", () => {
  it("fold-right reverses by appending each element last", () => {
    expect(showList(reverseViaFoldRight(list(1, 4, 9, 16, 25)))).toBe("(25 16 9 4 1)");
  });

  it("fold-left reverses by consing each element in front", () => {
    expect(showList(reverseViaFoldLeft(list(1, 4, 9, 16, 25)))).toBe("(25 16 9 4 1)");
  });
});
