// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 1.1

// The four primitive operations of exercise 1.2: infix arithmetic as calls.

/** Infix addition written as a call, for the nested-call exercise. */
export const add = (x: number, y: number): number => x + y;

/** Infix subtraction written as a call. */
export const sub = (x: number, y: number): number => x - y;

/** Infix multiplication written as a call. */
export const mul = (x: number, y: number): number => x * y;

/** Infix division written as a call. */
export const div = (x: number, y: number): number => x / y;

// Naming and the environment (1.1.2).

/** A binding names a value: the `size` session of the section. */
export const size = 2;

/** The area session's circle constant: the book's own 3.14159, not Math.PI. */
// biome-ignore lint/suspicious/noApproximativeNumericConstant: the book's session computes with 3.14159
export const pi = 3.14159;

/** The area session's radius. */
export const radius = 10;

/** A name for the result of a compound operation. */
export const circumference = 2 * pi * radius;

// Compound procedures (1.1.4).

/** To square something, multiply it by itself. */
export const square = (x: number): number => x * x;

/** Sums the squares of its two arguments. */
export const sumOfSquares = (x: number, y: number): number => square(x) + square(y);

/** Builds on sumOfSquares: sum of squares of a + 1 and a * 2. */
export const f = (a: number): number => sumOfSquares(a + 1, a * 2);

// Conditional expressions and predicates (1.1.6).

/** Case analysis as a nested conditional expression: three clauses. */
export const abs = (x: number): number => (x > 0 ? x : x === 0 ? 0 : -x);

/** The same case analysis with the else spelling. */
export const absElse = (x: number): number => (x < 0 ? -x : x);

/** The same case analysis as an if statement. */
export const absIf = (x: number): number => {
  if (x < 0) {
    return -x;
  }
  return x;
};

/** Greater-or-equal from the two primitive comparisons and `||`. */
export const greaterOrEqual = (x: number, y: number): boolean => x > y || x === y;

/** Greater-or-equal from `!` and `<`. */
export const greaterOrEqualNot = (x: number, y: number): boolean => !(x < y);

// Exercise 1.2: the four primitives above.

// Exercise 1.4: the conditional picks which procedure to apply.

/** Adds a and |b| by choosing the operation with a conditional expression. */
export const aPlusAbsB = (a: number, b: number): number =>
  (b > 0 ? (x: number, y: number): number => x + y : (x: number, y: number): number => x - y)(a, b);

// Exercise 1.5: Ben Bitdiddle's evaluation-order test.

/** Diverges: a body that only calls itself again. */
export const p = (): never => p();

/** Returns 0 when x is 0 and never looks at y; the argument still gets evaluated. */
export const test = (x: number, y: number): number => (x === 0 ? 0 : y);

// Exercise 1.6: the new-if experiment.

/** Alyssa's ordinary-procedure conditional: eager in all three arguments. */
export const newIf = <T>(predicate: boolean, thenClause: T, elseClause: T): T =>
  predicate ? thenClause : elseClause;

/** The square-root iteration rewritten with newIf: evaluates both branches. */
export const sqrtIterNewIf = (guess: number, x: number): number =>
  newIf(goodEnough(guess, x), guess, sqrtIterNewIf(improve(guess, x), x));

// Square roots by Newton's method (1.1.7), the flat four-procedure program.

/** Averages two numbers. */
export const average = (x: number, y: number): number => (x + y) / 2;

/** A guess is improved by averaging it with the quotient x / guess. */
export const improve = (guess: number, x: number): number => average(guess, x / guess);

/** The absolute-tolerance end test: square within 0.001 of the radicand. */
export const goodEnough = (guess: number, x: number): boolean => abs(square(guess) - x) < 0.001;

/** Newton's iteration: stop when the guess is good enough. */
export const sqrtIter = (guess: number, x: number): number =>
  goodEnough(guess, x) ? guess : sqrtIter(improve(guess, x), x);

/** The square root of x, starting from the guess 1.0. */
export const sqrt = (x: number): number => sqrtIter(1.0, x);

// Procedures as black-box abstractions (1.1.8).

/** The same squaring procedure with the parameter renamed. */
export const squareX = (x: number): number => x * x;

/** The parameter name is local: renaming it changes nothing. */
export const squareY = (y: number): number => y * y;

/** Squaring by way of logarithms: exp of the doubled log. */
export const squareByLog = (x: number): number => Math.exp(double(Math.log(x)));

/** Doubles its argument. */
export const double = (x: number): number => x + x;

// Internal definitions and block structure.

/** The sqrt program with the helpers nested inside, still passing x down. */
export const sqrtNested = (x: number): number => {
  function goodEnough(guess: number, x: number): boolean {
    return abs(square(guess) - x) < 0.001;
  }
  function improve(guess: number, x: number): number {
    return average(guess, x / guess);
  }
  function iter(guess: number, x: number): number {
    return goodEnough(guess, x) ? guess : iter(improve(guess, x), x);
  }
  return iter(1.0, x);
};

/** The sqrt program with x free in the internal definitions: lexical scoping. */
export const sqrtLexical = (x: number): number => {
  function goodEnough(guess: number): boolean {
    return abs(square(guess) - x) < 0.001;
  }
  function improve(guess: number): number {
    return average(guess, x / guess);
  }
  function iter(guess: number): number {
    return goodEnough(guess) ? guess : iter(improve(guess));
  }
  return iter(1.0);
};
