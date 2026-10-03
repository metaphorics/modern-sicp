// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { list, showList } from "../../packages/ch2/src/02-picture-language.js";
import { unionSetUnordered } from "./ex_2_59.js";

describe("exercise 2.59", () => {
  it("unions unordered sets, keeping first occurrences and no duplicates", () => {
    expect(showList(unionSetUnordered(list(1, 3, 5), list(4, 3, 6)))).toBe("[1, 5, 4, 3, 6]");
    expect(showList(unionSetUnordered(list(1, 2, 3), list(2, 3, 4)))).toBe("[1, 2, 3, 4]");
    expect(showList(unionSetUnordered(list(), list(2, 1)))).toBe("[2, 1]");
    expect(showList(unionSetUnordered(list(2, 1), list()))).toBe("[2, 1]");
  });
});
