// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { list, showList } from "../../packages/ch2/src/02-picture-language.js";
import { dotProduct, matrixTimesMatrix, matrixTimesVector, transpose } from "./ex_2_37.js";

const m = list(list(1, 2, 3, 4), list(4, 5, 6, 6), list(6, 7, 8, 9));

describe("exercise 2.37", () => {
  it("dot-product sums the entrywise products", () => {
    expect(dotProduct(list(1, 2, 3), list(4, 5, 6))).toBe(32);
  });

  it("matrix-times-vector dots each row with the vector", () => {
    expect(showList(matrixTimesVector(m, list(1, 2, 3, 4)))).toBe("[30, 56, 80]");
  });

  it("transpose turns rows into columns", () => {
    expect(showList(transpose(m))).toBe("[[1, 4, 6], [2, 5, 7], [3, 6, 8], [4, 6, 9]]");
  });

  it("matrix-times-matrix dots rows with columns", () => {
    expect(showList(matrixTimesMatrix(m, m))).toBe(
      "[[27, 33, 39, 43], [60, 75, 90, 100], [82, 103, 124, 138]]",
    );
  });
});
