// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 1.2

import type { Random } from "../../../examples/ch1/random.js";
import { square } from "./01-elements.js";

// Linear recursion and iteration (1.2.1).

/** Factorial as a linear recursive process: the chain of deferred multiplications. */
export const factorialRecursive = (n: number): number =>
  n === 1 ? 1 : n * factorialRecursive(n - 1);

/** Factorial as a linear iterative process: the loop carries the state variables. */
export const factorialIter = (n: number): number => {
  let product = 1;
  let counter = 1;
  while (counter <= n) {
    product = counter * product;
    counter += 1;
  }
  return product;
};

/** The block-structured variant the section's footnote sketches: the stepper hides inside. */
export const factorialBlockStructured = (n: number): number => {
  function iter(product: number, counter: number): number {
    if (counter > n) {
      return product;
    }
    return iter(counter * product, counter + 1);
  }
  return iter(1, 1);
};

// Exercise 1.9: the two addition shapes, in terms of inc and dec.

/** Adds one. */
export const inc = (x: number): number => x + 1;

/** Subtracts one. */
export const dec = (x: number): number => x - 1;

/** Addition whose self-call waits inside an inc: a linear recursive process. */
export const plusRecursive = (a: number, b: number): number =>
  a === 0 ? b : inc(plusRecursive(dec(a), b));

/** Addition in the iterative shape: a loop that updates the state variables. */
export const plusIterative = (a: number, b: number): number => {
  let x = a;
  let y = b;
  while (x > 0) {
    x = dec(x);
    y = inc(y);
  }
  return y;
};

// Exercise 1.10: Ackermann's function.

/** Ackermann's function, the section's exercise on growing fast. */
export const ackermann = (x: number, y: number): number => {
  if (y === 0) {
    return 0;
  }
  if (x === 0) {
    return 2 * y;
  }
  if (y === 1) {
    return 2;
  }
  return ackermann(x - 1, ackermann(x, y - 1));
};

/** Ackermann applied at x = 0. */
export const ackermannF = (n: number): number => ackermann(0, n);

/** Ackermann applied at x = 1. */
export const ackermannG = (n: number): number => ackermann(1, n);

/** Ackermann applied at x = 2. */
export const ackermannH = (n: number): number => ackermann(2, n);

/** The comparison the exercise spells out: 5n^2. */
export const ackermannK = (n: number): number => 5 * n * n;

// Tree recursion (1.2.2).

/** The tree-recursive Fibonacci: instructive, and redundant. */
export const fibRecursive = (n: number): number =>
  n === 0 ? 0 : n === 1 ? 1 : fibRecursive(n - 1) + fibRecursive(n - 2);

/** Fibonacci in the iterative shape: a loop over the pair of state variables. */
export const fibIter = (n: number): number => {
  let a = 1;
  let b = 0;
  let count = n;
  while (count > 0) {
    const next = a + b;
    a = b;
    b = next;
    count -= 1;
  }
  return b;
};

/** The denomination of the first of `kindsOfCoins` kinds, largest first. */
const firstDenomination = (kindsOfCoins: number): number => {
  if (kindsOfCoins === 1) {
    return 1;
  }
  if (kindsOfCoins === 2) {
    return 5;
  }
  if (kindsOfCoins === 3) {
    return 10;
  }
  if (kindsOfCoins === 4) {
    return 25;
  }
  return 50;
};

/** Ways to change `amount` using `kindsOfCoins` kinds: the tree-recursive reduction. */
const cc = (amount: number, kindsOfCoins: number): number => {
  if (amount === 0) {
    return 1;
  }
  if (amount < 0 || kindsOfCoins === 0) {
    return 0;
  }
  return cc(amount, kindsOfCoins - 1) + cc(amount - firstDenomination(kindsOfCoins), kindsOfCoins);
};

/** Counts the ways to change an amount with half-dollars, quarters, dimes, nickels, pennies. */
export const countChange = (amount: number): number => cc(amount, 5);

// Exercise 1.15: the sine reduction.

/** The cube the sine reduction needs. */
export const cube = (x: number): number => x * x * x;

/** The trigonometric reduction step. */
export const p = (x: number): number => 3 * x - 4 * cube(x);

/** Sine by argument reduction until the angle is at most 0.1 radians. */
export const sine = (angle: number): number =>
  !(Math.abs(angle) > 0.1) ? angle : p(sine(angle / 3));

// Exponentiation (1.2.4).

/** Exponentiation as a linear recursive process. */
export const exptRecursive = (b: number, n: number): number =>
  n === 0 ? 1 : b * exptRecursive(b, n - 1);

/** Exponentiation in the iterative shape: a counter and a running product. */
export const exptIter = (b: number, n: number): number => {
  let counter = n;
  let product = 1;
  while (counter > 0) {
    product = b * product;
    counter -= 1;
  }
  return product;
};

/** Evenness in terms of the primitive remainder. */
export const isEven = (n: number): boolean => n % 2 === 0;

/** Fast exponentiation by successive squaring. */
export const fastExpt = (b: number, n: number): number => {
  if (n === 0) {
    return 1;
  }
  if (isEven(n)) {
    return square(fastExpt(b, n / 2));
  }
  return b * fastExpt(b, n - 1);
};

// Exercise 1.17: multiplication by repeated addition.

/** Multiplication as repeated addition, analogous to exptRecursive. */
export const multiply = (a: number, b: number): number => (b === 0 ? 0 : a + multiply(a, b - 1));

// Greatest common divisors (1.2.5).

/** Euclid's Algorithm in the iterative shape: the pair (a, b) is the whole state. */
export const gcd = (a: number, b: number): number => {
  let x = a;
  let y = b;
  while (y !== 0) {
    const r = x % y;
    x = y;
    y = r;
  }
  return x;
};

// Example: testing for primality (1.2.6).

/** Whether b divides a evenly. */
export const divides = (a: number, b: number): boolean => b % a === 0;

/** The smallest integral divisor greater than 1, found by testing upward. */
export const findDivisor = (n: number, testDivisor: number): number => {
  let d = testDivisor;
  while (d * d <= n) {
    if (divides(d, n)) {
      return d;
    }
    d += 1;
  }
  return n;
};

/** The smallest divisor greater than 1, search starting at 2. */
export const smallestDivisor = (n: number): number => findDivisor(n, 2);

/** n is prime exactly when it is its own smallest divisor. */
export const isPrime = (n: number): boolean => n === smallestDivisor(n);

/** The exponential of base modulo m by successive squaring. */
export const expmod = (base: number, exp: number, m: number): number => {
  if (exp === 0) {
    return 1;
  }
  if (isEven(exp)) {
    return square(expmod(base, exp / 2, m)) % m;
  }
  return (base * expmod(base, exp - 1, m)) % m;
};

/** One Fermat trial: a random a below n must satisfy a^n congruent to a modulo n. */
export const fermatTest = (n: number, rng: Random): boolean => {
  const a = 1 + rng.random(n - 1);
  return expmod(a, n, n) === a;
};

/** The Fermat test run `times` times: true only if every trial passes. */
export const fastPrime = (n: number, times: number, rng: Random): boolean => {
  let remaining = times;
  while (remaining > 0) {
    if (!fermatTest(n, rng)) {
      return false;
    }
    remaining -= 1;
  }
  return true;
};

// Exercise 1.25: Alyssa P. Hacker's simplification.

/** Alyssa's expmod: the whole exponential first, remainder after. */
export const expmodSimplified = (base: number, exp: number, m: number): number =>
  fastExpt(base, exp) % m;

// Exercise 1.26: Louis Reasoner's explicit multiplication.

/** Louis's expmod: the squaring written as two self-calls, which doubles the work. */
export const expmodExplicit = (base: number, exp: number, m: number): number => {
  if (exp === 0) {
    return 1;
  }
  if (isEven(exp)) {
    return (expmodExplicit(base, exp / 2, m) * expmodExplicit(base, exp / 2, m)) % m;
  }
  return (base * expmodExplicit(base, exp - 1, m)) % m;
};
