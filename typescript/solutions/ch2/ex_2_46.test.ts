// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { makeVect as moduleVect, showVect } from "../../packages/ch2/src/02-picture-language.js";
import { addVect2, makeVect, scaleVect2, subVect2, xCorVect, yCorVect } from "./ex_2_46.js";

describe("exercise 2.46", () => {
  it("make-vect builds a pair and the selectors read its coordinates", () => {
    const v = makeVect(3, 4);
    expect(v).toStrictEqual([3, 4]);
    expect(xCorVect(v)).toBe(3);
    expect(yCorVect(v)).toBe(4);
  });

  it("add-vect, sub-vect and scale-vect work componentwise", () => {
    expect(addVect2([1, 2], [3, 4])).toStrictEqual([4, 6]);
    expect(subVect2([3, 4], [1, 2])).toStrictEqual([2, 2]);
    expect(scaleVect2(3, [2, 5])).toStrictEqual([6, 15]);
  });

  it("tuples render like the module's record vectors", () => {
    expect(showVect(moduleVect(1, 2))).toBe("(1, 2)");
    expect(`(${xCorVect(makeVect(1, 2))}, ${yCorVect(makeVect(1, 2))})`).toBe("(1, 2)");
  });
});
