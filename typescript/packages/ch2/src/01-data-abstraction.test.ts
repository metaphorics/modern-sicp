// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 2.1

import { describe, expect, it } from "vitest";

import {
  addRat,
  car,
  carProc,
  cdr,
  cdrProc,
  center,
  cons,
  consProc,
  denomAtAccess,
  divInterval,
  equalRat,
  makeCenterWidth,
  makeInterval,
  makeRat,
  makeRatAtAccess,
  makeRatUnreduced,
  mulRat,
  numerAtAccess,
  ok,
  par1,
  par2,
  printRat,
  width,
} from "./01-data-abstraction.js";

describe("section 2.1: introduction to data abstraction", () => {
  it("the pair glue retrieves the two parts it glued", () => {
    const x = cons(1, 2);
    expect(car(x)).toBe(1);
    expect(cdr(x)).toBe(2);

    const y = cons(3, 4);
    const z = cons(x, y);
    expect(car(car(z))).toBe(1);
    expect(car(cdr(z))).toBe(3);
  });

  it("the rational-number interactions print the book's values", () => {
    const oneHalf = makeRatUnreduced(1n, 2n);
    expect(printRat(oneHalf)).toBe("1/2");
    const oneThird = makeRatUnreduced(1n, 3n);
    expect(printRat(addRat(oneHalf, oneThird))).toBe("5/6");
    expect(printRat(mulRat(oneHalf, oneThird))).toBe("1/6");
    // With the first-listing bare-pair constructor, the sum of one-third and
    // one-third comes out unreduced. The module's addRat routes through the
    // reducing makeRat, so the book's 6/9 is the addition formula on the bare
    // pair, exactly what the first-listing constructor hands back:
    expect(printRat(cons(1n * 3n + 1n * 3n, 3n * 3n))).toBe("6/9");
    expect(equalRat(makeRatUnreduced(1n, 2n), makeRatUnreduced(2n, 4n))).toBe(true);
  });

  it("the gcd-reducing constructor turns 6/9 into 2/3 without touching addRat", () => {
    const oneThird = makeRat(1n, 3n);
    expect(printRat(addRat(oneThird, oneThird))).toBe("2/3");
  });

  it("reducing at access time lands on the same rational number", () => {
    expect(numerAtAccess(makeRatAtAccess(6n, 8n))).toBe(3n);
    expect(denomAtAccess(makeRatAtAccess(6n, 8n))).toBe(4n);
    expect(printRat(makeRat(6n, 8n))).toBe("3/4");
  });

  it("the procedural pair answers messages 0 and 1 and refuses the rest", () => {
    expect(carProc(consProc(17, 42))).toStrictEqual(ok(17));
    expect(cdrProc(consProc(17, 42))).toStrictEqual(ok(42));
    expect(consProc(17, 42)(7)).toStrictEqual({
      _tag: "Error",
      error: { _tag: "UnknownMessage", m: 7 },
    });
  });

  it("makeInterval enforces the ordered-bounds invariant", () => {
    expect(makeInterval(3.35, 3.65)).toStrictEqual(ok({ lo: 3.35, hi: 3.65 }));
    expect(makeInterval(3.65, 3.35)).toStrictEqual({
      _tag: "Error",
      error: { _tag: "UnorderedBounds", a: 3.65, b: 3.35 },
    });
  });

  it("the resistor example's true range and the wider interval answer", () => {
    const r1 = { lo: 6.12, hi: 7.48 };
    const r2 = { lo: 4.465, hi: 4.935 };
    // The formula evaluated at the endpoints gives the book's 2.58 to 2.97;
    // par2, which mentions each resistor once, lands on that range.
    expect(par2(r1, r2).lo).toBeCloseTo(2.58, 2);
    expect(par2(r1, r2).hi).toBeCloseTo(2.97, 2);
    // par1 treats every occurrence as independent and comes out wider.
    expect(par1(r1, r2).lo).toBeCloseTo(2.2, 2);
    expect(par1(r1, r2).hi).toBeCloseTo(3.49, 2);
  });

  it("center-width form round-trips the resistor's tolerance", () => {
    const i = makeCenterWidth(3.5, 0.15);
    expect(i).toStrictEqual({ lo: 3.35, hi: 3.65 });
    expect(center(i)).toBe(3.5);
    expect(width(i)).toBeCloseTo(0.15, 10);
    expect(divInterval({ lo: 6, hi: 8 }, { lo: 2, hi: 4 })).toStrictEqual({ lo: 1.5, hi: 4 });
  });
});
