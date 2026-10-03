// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { showList } from "../../packages/ch2/src/02-picture-language.js";
import {
  adjoinSetTree,
  elementOfSetTree,
  emptyTreeSet,
  type TreeSet,
} from "../../packages/ch2/src/03-symbolic-data.js";
import { intersectionSetTree, treeElements, unionSetTree } from "./ex_2_65.js";

const build = (xs: ReadonlyArray<number>): TreeSet => {
  let t = emptyTreeSet;
  for (const x of xs) {
    t = adjoinSetTree(x, t);
  }
  return t;
};

describe("exercise 2.65", () => {
  it("unions and intersects tree sets, results ordered and balanced-ish", () => {
    const evens = build([2, 4, 6, 8]);
    const odds = build([1, 3, 5, 7, 8]);
    expect(showList(treeElements(unionSetTree(evens, odds)))).toBe("[1, 2, 3, 4, 5, 6, 7, 8]");
    expect(showList(treeElements(intersectionSetTree(evens, odds)))).toBe("[8]");
    expect(elementOfSetTree(6, unionSetTree(evens, odds))).toBe(true);
    expect(elementOfSetTree(5, intersectionSetTree(evens, odds))).toBe(false);
    expect(treeElements(unionSetTree(emptyTreeSet, odds))).toEqual(treeElements(odds));
  });
});
