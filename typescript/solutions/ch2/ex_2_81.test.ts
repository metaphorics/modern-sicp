// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  makeComplexFromRealImag,
  makeTsNumber,
  show,
} from "../../packages/ch2/src/05-generic-operations.js";
import {
  add81,
  applyGenericGuarded,
  applyGenericLouis,
  exp81,
  installSelfCoercions81,
  lookupCoercion81,
} from "./ex_2_81.js";

const sn = (n: bigint): bigint => makeTsNumber(n);
const cpx = (x: number, y: number) => makeComplexFromRealImag(x, y);

describe("exercise 2.81: self-coercion", () => {
  it("(b) without self-coercions a same-type miss just fails, once", () => {
    // exp has no complex entry and no complex->complex coercion exists
    // yet: Louis's own dispatch gives up after one miss.
    expect(show(applyGenericLouis("exp", cpx(2, 3), cpx(1, 1)))).toBe(
      'No method for these types: applyGeneric("exp", ["complex", "complex"])',
    );
  });

  it("(a) Louis's identities land in the coercion table", () => {
    expect(lookupCoercion81("ts-number", "complex")._tag).toBe("Some");
    installSelfCoercions81();
    expect(lookupCoercion81("ts-number", "ts-number")._tag).toBe("Some");
    expect(lookupCoercion81("complex", "complex")._tag).toBe("Some");
  });

  it("(a, c) with self-coercions the guarded dispatch breaks the retry loop", () => {
    // exp(complex, complex) with a complex->complex identity entry:
    // the unguarded dispatch would coerce and retry forever; the guard
    // refuses to coerce same-type arguments and fails immediately.
    expect(show(exp81(cpx(2, 3), cpx(1, 1)))).toBe(
      'No method for these types: applyGeneric("exp", ["complex", "complex"])',
    );
  });

  it("(b, c) cross-type addition still coerces under the guard", () => {
    expect(show(add81(cpx(1, 2), sn(4n)))).toBe("[complex, rectangular, 5, 2]");
    expect(show(add81(sn(4n), cpx(1, 2)))).toBe("[complex, rectangular, 5, 2]");
  });

  it("(c) same-type addition dispatches directly, coercion never consulted", () => {
    expect(show(add81(sn(3n), sn(4n)))).toBe("7");
    expect(show(applyGenericGuarded("add", cpx(1, 2), cpx(3, 4)))).toBe(
      "[complex, rectangular, 4, 6]",
    );
  });

  it("the unguarded dispatch still answers every terminating call", () => {
    expect(show(applyGenericLouis("add", sn(3n), sn(4n)))).toBe("7");
    expect(show(applyGenericLouis("add", cpx(1, 2), sn(4n)))).toBe("[complex, rectangular, 5, 2]");
  });
});
