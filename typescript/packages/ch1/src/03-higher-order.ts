// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 1.3

import { abs, average, square } from "./01-elements.js";
import { cube } from "./02-processes.js";

// Procedures as arguments (1.3.1).

/** Sum of the integers from a through b: a linear recursive process. */
export const sumIntegers = (a: number, b: number): number =>
  a > b ? 0 : a + sumIntegers(a + 1, b);

/** Sum of the cubes of the integers in the given range. */
export const sumCubes = (a: number, b: number): number =>
  a > b ? 0 : cube(a) + sumCubes(a + 1, b);

/** Sum of 1/(1·3) + 1/(5·7) + 1/(9·11) + ..., which converges to pi/8 very slowly. */
export const piSum = (a: number, b: number): number =>
  a > b ? 0 : 1.0 / (a * (a + 2)) + piSum(a + 4, b);

/** The summation concept itself: sigma notation as a procedure. */
export const sum = (
  term: (x: number) => number,
  a: number,
  next: (x: number) => number,
  b: number,
): number => (a > b ? 0 : term(a) + sum(term, next(a), next, b));

/** Adds one: the book's own increment helper of this section. */
export const inc = (n: number): number => n + 1;

/** Sum of cubes, recovered as one call to sum. */
export const sumCubesViaSum = (a: number, b: number): number => sum(cube, a, inc, b);

/** The procedure whose value is its input. */
export const identity = (x: number): number => x;

/** Sum of integers, recovered as one call to sum. */
export const sumIntegersViaSum = (a: number, b: number): number => sum(identity, a, inc, b);

/** pi-sum with the term and next helpers embedded as local definitions. */
export const piSumViaSum = (a: number, b: number): number => {
  const piTerm = (x: number): number => 1.0 / (x * (x + 2));
  const piNext = (x: number): number => x + 4;
  return sum(piTerm, a, piNext, b);
};

/** pi-sum with the helpers inlined as arrow functions: 1.3.2's spelling. */
export const piSumArrow = (a: number, b: number): number =>
  sum(
    (x) => 1.0 / (x * (x + 2)),
    a,
    (x) => x + 4,
    b,
  );

/** Definite integral by summation, with the add-dx helper embedded. */
export const integral = (f: (x: number) => number, a: number, b: number, dx: number): number => {
  const addDx = (x: number): number => x + dx;
  return sum(f, a + dx / 2.0, addDx, b) * dx;
};

/** Definite integral with the add-dx helper inlined as an arrow function. */
export const integralArrow = (f: (x: number) => number, a: number, b: number, dx: number): number =>
  sum(f, a + dx / 2.0, (x) => x + dx, b) * dx;

// Arrow functions (1.3.2).

/** The named spelling of the procedure that adds 4. */
export function plus4(x: number): number {
  return x + 4;
}

/** The anonymous spelling of the very same procedure. */
export const plus4Arrow = (x: number): number => x + 4;

/** A combination whose operator is an anonymous arrow: 1 + 2 + 3^2 is 12. */
export const anonymousApplication: number = ((x: number, y: number, z: number): number =>
  x + y + square(z))(1, 2, 3);

/** f(x, y) = x(1 + xy)^2 + y(1 - y) + (1 + xy)(1 - y), locals via a helper procedure. */
export const fxy = (x: number, y: number): number => {
  function fHelper(a: number, b: number): number {
    return x * square(a) + y * b + a * b;
  }
  return fHelper(1 + x * y, 1 - y);
};

/** The same f with the helper inlined as an anonymous arrow: the shape `let` desugars to. */
export const fxyAnonymous = (x: number, y: number): number =>
  ((a: number, b: number): number => x * square(a) + y * b + a * b)(1 + x * y, 1 - y);

/** The same f with the locals spelled directly: the host's own binding form. */
export const fxyBlock = (x: number, y: number): number => {
  const a = 1 + x * y;
  const b = 1 - y;
  return x * square(a) + y * b + a * b;
};

/** The scope lesson: the inner x is 3 (giving 33), the outer x is still 5, and 33 + 5 = 38. */
export const localBindingLesson = (): number => {
  const x = 5;
  return ((x: number): number => x + x * 10)(3) + x;
};

/** The initializer lesson: y is computed from the outer x (4), the block's x is 3, and 3 * 4 = 12. */
export const outsideInitLesson = (): number => {
  const x = 2;
  const y = x + 2;
  {
    const x = 3;
    return x * y;
  }
};

// Procedures as general methods (1.3.3).

/** The endpoints are close enough when they differ by less than 0.001. */
export const closeEnough = (x: number, y: number): boolean => abs(x - y) < 0.001;

/** Halves the interval of uncertainty until it is small enough; the depth is Theta(log(L/T)). */
export const search = (f: (x: number) => number, negPoint: number, posPoint: number): number => {
  const midpoint = average(negPoint, posPoint);
  if (closeEnough(negPoint, posPoint)) {
    return midpoint;
  }
  const testValue = f(midpoint);
  if (testValue > 0) {
    return search(f, negPoint, midpoint);
  }
  if (testValue < 0) {
    return search(f, midpoint, posPoint);
  }
  return midpoint;
};

/** The half-interval method's failure mode: f does not change sign between a and b. */
export type HalfIntervalResult =
  | { readonly _tag: "Ok"; readonly value: number }
  | { readonly _tag: "SameSign"; readonly a: number; readonly b: number };

/** A root of f between a and b, or a SameSign error when the endpoints do not bracket one. */
export const halfIntervalMethod = (
  f: (x: number) => number,
  a: number,
  b: number,
): HalfIntervalResult => {
  const aValue = f(a);
  const bValue = f(b);
  if (aValue < 0 && bValue > 0) {
    return { _tag: "Ok", value: search(f, a, b) };
  }
  if (bValue < 0 && aValue > 0) {
    return { _tag: "Ok", value: search(f, b, a) };
  }
  return { _tag: "SameSign", a, b };
};

/** Two successive values whose difference is under 0.00001 count as not changing much. */
const TOLERANCE = 0.00001;

/**
 * Approximates a fixed point of f by repeated application. Node gives no tail-call
 * guarantee, so the repeatedly-improving guess runs as a loop over the state.
 */
export const fixedPoint = (f: (x: number) => number, firstGuess: number): number => {
  let guess = firstGuess;
  for (;;) {
    const next = f(guess);
    if (abs(guess - next) < TOLERANCE) {
      return next;
    }
    guess = next;
  }
};

/**
 * Square root as a fixed point of y |-> x/y. This search does not converge: the
 * guesses oscillate between y and x/y forever. The book shows it to motivate
 * average damping, so the tests never call it.
 */
export const sqrtByFixedPointNaive = (x: number): number => fixedPoint((y) => x / y, 1.0);

/** Square root as a fixed point of y |-> (1/2)(y + x/y): the damped search converges. */
export const sqrtByAverageDamp = (x: number): number => fixedPoint((y) => average(y, x / y), 1.0);

// Procedures as returned values (1.3.4).

/** The function whose value at x is the average of x and f(x). */
export const averageDamp =
  (f: (x: number) => number): ((x: number) => number) =>
  (x) =>
    average(x, f(x));

/** Square root reformulated so the three ideas appear as separate, reusable pieces. */
export const sqrtByDampedFixedPoint = (x: number): number =>
  fixedPoint(
    averageDamp((y) => x / y),
    1.0,
  );

/** Cube root as a fixed point of y |-> x/y^2. */
export const cubeRoot = (x: number): number =>
  fixedPoint(
    averageDamp((y) => x / square(y)),
    1.0,
  );

/** The step size of the numerical derivative. */
export const dx = 0.00001;

/** The function whose value at x is the slope of g near x. */
export const deriv =
  (g: (x: number) => number): ((x: number) => number) =>
  (x) =>
    (g(x + dx) - g(x)) / dx;

/** The transformation g |-> x - g(x)/Dg(x), whose fixed points solve g(x) = 0. */
export const newtonTransform =
  (g: (x: number) => number): ((x: number) => number) =>
  (x) =>
    x - g(x) / deriv(g)(x);

/** Newton's method: the fixed-point search applied to the Newton transformation. */
export const newtonsMethod = (g: (x: number) => number, guess: number): number =>
  fixedPoint(newtonTransform(g), guess);

/** The fixed-point search applied to an arbitrary transformation of g. */
export const fixedPointOfTransform = (
  g: (x: number) => number,
  transform: (g: (x: number) => number) => (x: number) => number,
  guess: number,
): number => fixedPoint(transform(g), guess);

/** Square root as Newton's method on g(y) = y^2 - x. */
export const sqrtByNewtonsMethod = (x: number): number => newtonsMethod((y) => square(y) - x, 1.0);

/** Square root as a fixed point of the Newton transformation of g(y) = y^2 - x. */
export const sqrtByNewtonTransform = (x: number): number =>
  fixedPointOfTransform((y) => square(y) - x, newtonTransform, 1.0);

/** Square root as a fixed point of the average-damped y |-> x/y: the section's first form. */
export const sqrtByDampedTransform = (x: number): number =>
  fixedPointOfTransform((y) => x / y, averageDamp, 1.0);

// The chapter 2 preview's linear combination.

/** A linear combination of numbers. */
export const linearCombination = (a: number, b: number, x: number, y: number): number =>
  a * x + b * y;

/** A linear combination whenever addition and multiplication are defined. */
export const linearCombinationGeneric = (
  add: (x: number, y: number) => number,
  mul: (x: number, y: number) => number,
  a: number,
  b: number,
  x: number,
  y: number,
): number => add(mul(a, x), mul(b, y));
