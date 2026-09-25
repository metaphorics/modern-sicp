// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { list, showList } from "../../packages/ch2/src/02-picture-language.js";

import { reverse } from "./ex_2_18.js";

describe("exercise 2.18", () => {
  it("the reverse of (1 4 9 16 25) is (25 16 9 4 1)", () => {
    expect(showList(reverse(list(1, 4, 9, 16, 25)))).toBe("(25 16 9 4 1)");
  });

  it("the empty list reverses to the empty list", () => {
    expect(showList(reverse(list()))).toBe("()");
  });
});
