// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { none, some } from "../../packages/ch2/src/02-picture-language.js";
import { adjoinRecordTree, emptyRecordTree, lookupTree, type RecordTree } from "./ex_2_66.js";

const db = (): RecordTree<string> =>
  adjoinRecordTree(
    [7, "Grace"],
    adjoinRecordTree(
      [3, "Ada"],
      adjoinRecordTree(
        [12, "Edsger"],
        adjoinRecordTree([1, "Dot"], adjoinRecordTree([5, "Linus"], emptyRecordTree)),
      ),
    ),
  );

describe("exercise 2.66", () => {
  it("finds records by key and misses cleanly", () => {
    expect(lookupTree(5, db())).toStrictEqual(some([5, "Linus"]));
    expect(lookupTree(1, db())).toStrictEqual(some([1, "Dot"]));
    expect(lookupTree(12, db())).toStrictEqual(some([12, "Edsger"]));
    expect(lookupTree(4, db())).toStrictEqual(none);
    expect(lookupTree(99, db())).toStrictEqual(none);
  });
});
