// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 1.3

import { describe, expect, it } from "vitest";

import { square } from "./01-elements.js";
import { cube } from "./02-processes.js";
import {
  anonymousApplication,
  averageDamp,
  cubeRoot,
  deriv,
  fixedPoint,
  fixedPointOfTransform,
  fxy,
  fxyAnonymous,
  fxyBlock,
  halfIntervalMethod,
  identity,
  inc,
  integral,
  integralArrow,
  linearCombination,
  linearCombinationGeneric,
  localBindingLesson,
  newtonsMethod,
  outsideInitLesson,
  piSum,
  piSumArrow,
  piSumViaSum,
  plus4,
  plus4Arrow,
  sqrtByAverageDamp,
  sqrtByDampedFixedPoint,
  sqrtByDampedTransform,
  sqrtByNewtonsMethod,
  sqrtByNewtonTransform,
  sum,
  sumCubes,
  sumCubesViaSum,
  sumIntegers,
  sumIntegersViaSum,
} from "./03-higher-order.js";

describe("section 1.3: higher-order procedures", () => {
  it("the three introductory procedures land on the printed sums", () => {
    expect(sumIntegers(1, 10)).toBe(55);
    expect(sumCubes(1, 10)).toBe(3025);
    expect(8 * piSum(1, 1000)).toBe(3.139592655589783);
  });

  it("sum recovers each of the three as one call", () => {
    expect(sumCubesViaSum(1, 10)).toBe(3025);
    expect(sumIntegersViaSum(1, 10)).toBe(55);
    expect(8 * piSumViaSum(1, 1000)).toBe(3.139592655589783);
    expect(8 * piSumArrow(1, 1000)).toBe(3.139592655589783);
    expect(sum(identity, 1, inc, 10)).toBe(55);
  });

  it("the integral of cube over [0, 1] approaches 1/4", () => {
    expect(integral(cube, 0, 1, 0.01)).toBe(0.24998750000000042);
    expect(integral(cube, 0, 1, 0.001)).toBe(0.249999875000001);
    expect(integralArrow(cube, 0, 1, 0.01)).toBe(0.24998750000000042);
  });

  it("the two spellings of plus 4 are the same procedure", () => {
    expect(plus4(2)).toBe(6);
    expect(plus4Arrow(2)).toBe(6);
  });

  it("an anonymous arrow serves as the operator", () => {
    expect(anonymousApplication).toBe(12);
  });

  it("the three spellings of f(x, y) agree", () => {
    expect(fxy(2, 4)).toBe(123);
    expect(fxyAnonymous(2, 4)).toBe(123);
    expect(fxyBlock(2, 4)).toBe(123);
    expect(fxy(1, 3)).toBe(fxyAnonymous(1, 3));
    expect(fxy(1, 3)).toBe(fxyBlock(1, 3));
  });

  it("the two scope lessons print 38 and 12", () => {
    expect(localBindingLesson()).toBe(38);
    expect(outsideInitLesson()).toBe(12);
  });

  it("the half-interval method brackets pi and the cubic's root", () => {
    expect(halfIntervalMethod(Math.sin, 2.0, 4.0)).toStrictEqual({
      _tag: "Ok",
      value: 3.14111328125,
    });
    expect(halfIntervalMethod((x) => cube(x) - 2 * x - 3, 1.0, 2.0)).toStrictEqual({
      _tag: "Ok",
      value: 1.89306640625,
    });
  });

  it("the half-interval method refuses endpoints of equal sign", () => {
    expect(halfIntervalMethod(square, 1.0, 2.0)).toStrictEqual({
      _tag: "SameSign",
      a: 1.0,
      b: 2.0,
    });
  });

  it("cosine settles on its fixed point, and so does sin + cos", () => {
    expect(fixedPoint(Math.cos, 1.0)).toBe(0.7390822985224023);
    expect(fixedPoint((y) => Math.sin(y) + Math.cos(y), 1.0)).toBe(1.2587315962971173);
  });

  it("the undamped y |-> x/y oscillates with period two", () => {
    const guesses: number[] = [];
    let guess = 1.0;
    for (let i = 0; i < 6; i += 1) {
      guesses.push(guess);
      guess = 2 / guess;
    }
    expect(guesses).toStrictEqual([1.0, 2.0, 1.0, 2.0, 1.0, 2.0]);
  });

  it("the damped fixed-point search finds the square root", () => {
    expect(sqrtByAverageDamp(2.0)).toBe(1.4142135623746899);
    expect(sqrtByDampedFixedPoint(2.0)).toBe(sqrtByAverageDamp(2.0));
  });

  it("average damping of square at 10 averages 10 and 100", () => {
    expect(averageDamp(square)(10)).toBe(55);
  });

  it("the numerical derivative of cube at 5 is nearly 75", () => {
    expect(deriv(cube)(5)).toBe(75.00014999664018);
  });

  it("Newton's method converges on the square root two ways", () => {
    expect(sqrtByNewtonsMethod(2.0)).toBeCloseTo(Math.sqrt(2.0), 10);
    expect(sqrtByNewtonTransform(2.0)).toBe(sqrtByNewtonsMethod(2.0));
  });

  it("fixedPointOfTransform runs any transform to a fixed point", () => {
    expect(fixedPointOfTransform((y) => 2 / y, averageDamp, 1.0)).toBe(1.4142135623746899);
  });

  it("the cube-root fixed point lands within the tolerance", () => {
    expect(cubeRoot(8)).toBeCloseTo(2, 5);
    expect(cubeRoot(27)).toBeCloseTo(3, 5);
  });

  it("the two transformed square roots agree with the damped search", () => {
    expect(sqrtByNewtonTransform(2.0)).toBeCloseTo(Math.sqrt(2.0), 10);
    expect(sqrtByDampedTransform(2.0)).toBe(sqrtByAverageDamp(2.0));
  });

  it("the chapter 2 preview combines linearly", () => {
    expect(linearCombination(1, 2, 3, 4)).toBe(11);
    expect(
      linearCombinationGeneric(
        (x, y) => x + y,
        (x, y) => x * y,
        1,
        2,
        3,
        4,
      ),
    ).toBe(11);
  });

  it("newtonsMethod lands on the cubic root of the 1.3.3 example", () => {
    const g = (x: number): number => x * x * x - 2 * x - 3;
    const root = newtonsMethod(g, 1.0);
    expect(root).toBeCloseTo(1.893289196106493, 9);
    expect(g(root)).toBeCloseTo(0, 10);
  });
});
