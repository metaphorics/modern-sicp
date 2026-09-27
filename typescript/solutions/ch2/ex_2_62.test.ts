// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { list, showList } from "../../packages/ch2/src/02-picture-language.js";
import { unionSetOrdered } from "./ex_2_62.js";

describe("exercise 2.62", () => {
  it("merges ordered sets in one pass", () => {
    expect(showList(unionSetOrdered(list(1, 3, 5, 7), list(2, 3, 6, 7, 9)))).toBe(
      "(1 2 3 5 6 7 9)",
    );
    expect(showList(unionSetOrdered(list(1, 2), list(3, 4)))).toBe("(1 2 3 4)");
    expect(showList(unionSetOrdered(list(), list(1, 2)))).toBe("(1 2)");
    expect(showList(unionSetOrdered(list(1, 2), list()))).toBe("(1 2)");
  });
});
