// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { addPoly90, denseB, denseOps, showPoly90, sparseB, sparseOps } from "./ex_2_90.js";

describe("exercise 2.90: sparse and dense term lists", () => {
  it("keeps the sparse list at three terms", () => {
    const b = sparseB();
    expect(b.contents.terms.length).toBe(3);
    expect(b.contents.rep).toBe("sparse");
  });

  it("stores the dense list as a hundred and one coefficients", () => {
    const b = denseB();
    expect(b._tag === "Ok" && b.value.contents.terms.length).toBe(101);
    expect(b._tag === "Ok" && b.value.contents.rep).toBe("dense");
  });

  it("doubles a polynomial through either interface", () => {
    const doubledSparse = addPoly90(sparseB(), sparseB());
    expect(doubledSparse._tag === "Ok" && showPoly90(doubledSparse.value)).toBe(
      "(polynomial x (100 2) (2 4) (0 2))",
    );
    const dense = denseB();
    if (dense._tag !== "Ok") {
      throw new Error("denseB failed");
    }
    const doubledDense = addPoly90(dense.value, dense.value);
    expect(doubledDense._tag === "Ok" && showPoly90(doubledDense.value)).toBe(
      "(polynomial x (100 2) (2 4) (0 2))",
    );
  });

  it("both packages answer the same selectors for the same polynomial", () => {
    const s = sparseB();
    const d = denseB();
    if (d._tag !== "Ok") {
      throw new Error("denseB failed");
    }
    if (s.contents.rep !== "sparse" || d.value.contents.rep !== "dense") {
      throw new Error("unexpected representations");
    }
    expect(sparseOps.first(s.contents.terms)).toEqual(denseOps.first(d.value.contents.terms));
  });
});
