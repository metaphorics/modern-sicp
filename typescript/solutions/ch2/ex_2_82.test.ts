// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { show } from "../../packages/ch2/src/05-generic-operations.js";
import {
  addReal82,
  applyGenericMulti,
  installCoercions82,
  type MultiOp,
  mulRational82,
  rat82,
  real82,
  sn82,
} from "./ex_2_82.js";

const mulOps: Record<string, MultiOp> = { "rational,rational": mulRational82 };
const addOps: Record<string, MultiOp> = { "real,real": addReal82 };

const mulLookup = (tags: ReadonlyArray<string>): MultiOp | undefined => mulOps[tags.join(",")];
const addLookup = (tags: ReadonlyArray<string>): MultiOp | undefined => addOps[tags.join(",")];

describe("exercise 2.82: multi-argument coercion", () => {
  it("coerces both arguments to the second argument's type", () => {
    installCoercions82();
    expect(show(applyGenericMulti(mulLookup, sn82(3n), rat82(1n, 2n)))).toBe("[rational, 3, 2]");
  });

  it("dispatches when every argument already shares a type", () => {
    expect(show(applyGenericMulti(mulLookup, rat82(3n, 2n), rat82(1n, 4n)))).toBe(
      "[rational, 3, 8]",
    );
    expect(show(applyGenericMulti(addLookup, real82(1.5), real82(2.25)))).toBe("[real, 3.75]");
  });

  it("is not sufficiently general: the (real, real) add is never tried", () => {
    // ts-number and rational can both reach real, but only through
    // a two-step chain; the strategy tries each argument's own type
    // and never the intermediate one.
    expect(applyGenericMulti(addLookup, sn82(1n), rat82(1n, 2n))).toEqual({
      _tag: "Error",
      error: { _tag: "NoMethod", op: "apply-generic", tags: ["ts-number", "rational"] },
    });
  });
});
