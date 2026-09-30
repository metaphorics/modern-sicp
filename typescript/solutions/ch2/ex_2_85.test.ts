// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  makeComplexFromRealImag,
  makeRational,
  show,
  typeTagOf,
} from "../../packages/ch2/src/05-generic-operations.js";
import { real83 } from "./ex_2_83.js";
import { installTower84 } from "./ex_2_84.js";
import { applyGenericDropping, drop, installProject } from "./ex_2_85.js";

describe("exercise 2.85: drop", () => {
  installTower84();
  installProject();

  it("lowers (1.5 + 0i) as far as real", () => {
    const dropped = drop(makeComplexFromRealImag(1.5, 0));
    expect(dropped._tag === "Ok" && typeTagOf(dropped.value) === "real").toBe(true);
    expect(show(dropped)).toBe("[real, 1.5]");
  });

  it("lowers (1 + 0i) as far as the integers", () => {
    expect(show(drop(makeComplexFromRealImag(1, 0)))).toBe("1");
  });

  it("cannot lower (2 + 3i) at all", () => {
    const dropped = drop(makeComplexFromRealImag(2, 3));
    expect(dropped._tag === "Ok" && typeTagOf(dropped.value) === "complex").toBe(true);
  });

  it("keeps a real that is not integer-valued", () => {
    expect(show(drop(real83(2.5)))).toBe("[real, 2.5]");
  });

  it("lowers a whole rational to the integers it hides", () => {
    expect(show(drop(makeRational(6n, 3n)))).toBe("2");
    expect(show(drop(makeRational(1n, 2n)))).toBe("[rational, 1, 2]");
  });

  it("simplifies the answers of apply-generic: (2 + 3i) + (4 - 3i) is 6", () => {
    expect(
      show(
        applyGenericDropping("add", makeComplexFromRealImag(2, 3), makeComplexFromRealImag(4, -3)),
      ),
    ).toBe("6");
    expect(
      show(
        applyGenericDropping("add", makeComplexFromRealImag(1, 2), makeComplexFromRealImag(3, 4)),
      ),
    ).toBe("[complex, rectangular, 4, 6]");
  });
});
