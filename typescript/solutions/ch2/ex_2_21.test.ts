// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { list, showList } from "../../packages/ch2/src/02-picture-language.js";

import { squareListDirect, squareListViaMap } from "./ex_2_21.js";

describe("exercise 2.21", () => {
  it("both spellings square each element", () => {
    expect(showList(squareListDirect(list(1, 2, 3, 4)))).toBe("[1, 4, 9, 16]");
    expect(showList(squareListViaMap(list(1, 2, 3, 4)))).toBe("[1, 4, 9, 16]");
  });
});
