// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { list } from "../../packages/ch2/src/02-picture-language.js";
import {
  emptyTreeSet,
  makeTreeSet,
  type TreeSet,
} from "../../packages/ch2/src/03-symbolic-data.js";
import { listToTree } from "./ex_2_64.js";

describe("exercise 2.64", () => {
  it("builds the balanced tree for (1 3 5 7 9 11)", () => {
    const t = listToTree(list(1, 3, 5, 7, 9, 11));
    // Root 5, left (1 () 3), right (9 (7) (11)): the book's drawing.
    expect(t).toStrictEqual(
      makeTreeSet(
        5,
        makeTreeSet(1, emptyTreeSet, makeTreeSet(3, emptyTreeSet, emptyTreeSet)),
        makeTreeSet(
          9,
          makeTreeSet(7, emptyTreeSet, emptyTreeSet),
          makeTreeSet(11, emptyTreeSet, emptyTreeSet),
        ),
      ),
    );
  });

  it("handles the small sizes", () => {
    expect(listToTree(list())).toStrictEqual(emptyTreeSet);
    const one: TreeSet = listToTree(list(9));
    expect(one).toStrictEqual(makeTreeSet(9, emptyTreeSet, emptyTreeSet));
  });
});
