// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { list, showList } from "../../packages/ch2/src/02-picture-language.js";

import { lastPair } from "./ex_2_17.js";

describe("exercise 2.17", () => {
  it("the last pair of (23 72 149 34) is (34)", () => {
    expect(showList(lastPair(list(23, 72, 149, 34)))).toBe("(34)");
  });

  it("the last pair of a single-element list is the list itself", () => {
    expect(showList(lastPair(list(34)))).toBe("(34)");
  });
});
