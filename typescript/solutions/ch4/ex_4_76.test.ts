// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { makeBinding } from "../../packages/ch4/src/04-logic.js";
import { compatiblePair, conflictingPair, mergedAndAgreement, mergeFrames } from "./ex_4_76.js";

describe("exercise 4.76: merging frames for and", () => {
  it("merges compatible frames across shared variables", () => {
    const merged = mergeFrames(compatiblePair[0], compatiblePair[1]);
    expect(merged).toHaveLength(2);
    expect(merged?.map((binding) => binding.name).sort()).toStrictEqual(["x", "y"]);
  });

  it("rejects frames disagreeing on a shared variable", () => {
    expect(mergeFrames(conflictingPair[0], conflictingPair[1])).toBeUndefined();
  });

  it("unions disjoint frames", () => {
    const merged = mergeFrames(
      [makeBinding("a", { tag: "text", value: 1 })],
      [makeBinding("b", { tag: "text", value: 2 })],
    );
    expect(merged).toHaveLength(2);
  });

  it("agrees clause-at-a-time merging with the delivered conjunction", () => {
    const [pairwise, direct] = mergedAndAgreement();
    expect(pairwise).toStrictEqual([
      'and(supervisor(["Hacker", "Alyssa", "P"], ["Bitdiddle", "Ben"]), address(["Hacker", "Alyssa", "P"], ["Cambridge", ["Mass", "Ave"], "78"]))',
      'and(supervisor(["Fect", "Cy", "D"], ["Bitdiddle", "Ben"]), address(["Fect", "Cy", "D"], ["Cambridge", ["Ames", "Street"], "3"]))',
      'and(supervisor(["Tweakit", "Lem", "E"], ["Bitdiddle", "Ben"]), address(["Tweakit", "Lem", "E"], ["Boston", ["Bay", "State", "Road"], "22"]))',
    ]);
    expect(direct).toStrictEqual(pairwise);
  });
});
