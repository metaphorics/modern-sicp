// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { list, showList } from "../../packages/ch2/src/02-picture-language.js";
import { adjoinBag, elementOfBag, intersectionBag, unionBag } from "./ex_2_60.js";

describe("exercise 2.60", () => {
  it("membership and adjoining keep duplicates", () => {
    const bag = list(2, 3, 2, 1, 3, 2, 2);
    expect(elementOfBag(2, bag)).toBe(true);
    expect(elementOfBag(4, bag)).toBe(false);
    expect(showList(adjoinBag(4, bag))).toBe("[4, 2, 3, 2, 1, 3, 2, 2]");
  });

  it("union appends and intersection filters by membership", () => {
    expect(showList(unionBag(list(2, 3, 2), list(3, 4, 4)))).toBe("[2, 3, 2, 3, 4, 4]");
    expect(showList(intersectionBag(list(2, 3, 2, 1), list(3, 1, 3)))).toBe("[3, 1]");
    expect(showList(intersectionBag(list(2, 3, 2, 1), list()))).toBe("[]");
  });
});
