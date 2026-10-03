// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { showList } from "../../packages/ch2/src/02-picture-language.js";
import { emptyTreeSet, makeTreeSet } from "../../packages/ch2/src/03-symbolic-data.js";
import { treeToList1, treeToList2 } from "./ex_2_63.js";

const figureTrees = (): readonly [string, ReturnType<typeof makeTreeSet>][] => [
  [
    "left tree of figure 2.16",
    makeTreeSet(
      7,
      makeTreeSet(
        3,
        makeTreeSet(1, emptyTreeSet, emptyTreeSet),
        makeTreeSet(5, emptyTreeSet, emptyTreeSet),
      ),
      makeTreeSet(9, emptyTreeSet, makeTreeSet(11, emptyTreeSet, emptyTreeSet)),
    ),
  ],
  [
    "middle tree of figure 2.16",
    makeTreeSet(
      3,
      makeTreeSet(1, emptyTreeSet, emptyTreeSet),
      makeTreeSet(
        7,
        makeTreeSet(5, emptyTreeSet, emptyTreeSet),
        makeTreeSet(9, emptyTreeSet, makeTreeSet(11, emptyTreeSet, emptyTreeSet)),
      ),
    ),
  ],
  [
    "right tree of figure 2.16",
    makeTreeSet(
      5,
      makeTreeSet(3, makeTreeSet(1, emptyTreeSet, emptyTreeSet), emptyTreeSet),
      makeTreeSet(
        9,
        makeTreeSet(7, emptyTreeSet, emptyTreeSet),
        makeTreeSet(11, emptyTreeSet, emptyTreeSet),
      ),
    ),
  ],
];

describe("exercise 2.63", () => {
  it("a. both procedures produce the same ordered list for every tree", () => {
    for (const [name, tree] of figureTrees()) {
      expect(name && showList(treeToList1(tree))).toBe("[1, 3, 5, 7, 9, 11]");
      expect(showList(treeToList2(tree))).toBe("[1, 3, 5, 7, 9, 11]");
    }
    const balanced = makeTreeSet(
      5,
      makeTreeSet(3, makeTreeSet(1, emptyTreeSet, emptyTreeSet), emptyTreeSet),
      makeTreeSet(
        9,
        makeTreeSet(7, emptyTreeSet, emptyTreeSet),
        makeTreeSet(11, emptyTreeSet, emptyTreeSet),
      ),
    );
    expect(showList(treeToList1(balanced))).toBe("[1, 3, 5, 7, 9, 11]");
    expect(showList(treeToList2(balanced))).toBe("[1, 3, 5, 7, 9, 11]");
    expect(showList(treeToList1(emptyTreeSet))).toBe("[]");
    expect(showList(treeToList2(emptyTreeSet))).toBe("[]");
  });
});
