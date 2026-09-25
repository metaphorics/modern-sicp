// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { list, showList } from "../../packages/ch2/src/02-picture-language.js";
import { adjoinSetOrdered } from "./ex_2_61.js";

describe("exercise 2.61", () => {
  it("inserts at the ordered position", () => {
    expect(showList(adjoinSetOrdered(6, list(1, 3, 5, 7, 9)))).toBe("(1 3 5 6 7 9)");
    expect(showList(adjoinSetOrdered(0, list(1, 3)))).toBe("(0 1 3)");
    expect(showList(adjoinSetOrdered(10, list(1, 3)))).toBe("(1 3 10)");
  });

  it("returns the set unchanged on an exact hit", () => {
    expect(showList(adjoinSetOrdered(3, list(1, 3, 5)))).toBe("(1 3 5)");
    expect(showList(adjoinSetOrdered(1, list(1)))).toBe("(1)");
  });
});
