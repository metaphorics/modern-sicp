// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { makeComplexFromRealImag, show } from "../../packages/ch2/src/05-generic-operations.js";
import { rat83, real83, sn83 } from "./ex_2_83.js";
import { applyGenericRaising, installTower84 } from "./ex_2_84.js";

describe("exercise 2.84: successive raising", () => {
  installTower84();
  it("raises the ordinary number to the rational's level", () => {
    expect(show(applyGenericRaising("add", sn83(1n), rat83(1n, 2n)))).toBe("[rational, 3, 2]");
    expect(show(applyGenericRaising("mul", sn83(2n), rat83(3n, 4n)))).toBe("[rational, 3, 2]");
  });

  it("raises across two levels when the operation needs it", () => {
    expect(show(applyGenericRaising("add", rat83(1n, 2n), makeComplexFromRealImag(2, 0)))).toBe(
      "[complex, rectangular, 2.5, 0]",
    );
  });

  it("meets the operation at the higher type from either side", () => {
    expect(show(applyGenericRaising("add", real83(1.5), sn83(1n)))).toBe("[real, 2.5]");
  });

  it("reports the miss when no chain reaches a common level with the op", () => {
    // exp exists only for ordinary numbers; the complex argument
    // cannot be lowered to meet it, so the ordinary argument is raised
    // up to complex, where exp still has no entry.
    expect(show(applyGenericRaising("exp", makeComplexFromRealImag(1, 1), sn83(2n)))).toBe(
      'No method for these types: applyGeneric("exp", ["complex", "complex"])',
    );
  });
});
