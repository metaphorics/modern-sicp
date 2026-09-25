// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 1.41: the double combinator puzzle.
 *
 * double(f) applies f twice. The puzzle's expression applies double to
 * itself: double(double(double)) is a function transformer that ends up
 * applying its input sixteen times, so with inc it answers 5 + 16 = 21.
 * The statement's own expression forces double to accept a procedure
 * transformer as well as a number-to-number procedure, so f's type is
 * a parameter - Scheme's universal application given its host-honest
 * spelling.
 */
export function double<T>(f: (x: T) => T): (x: T) => T {
  return (x: T): T => f(f(x));
}

const inc = (x: number): number => x + 1;

export function puzzleAnswer(): number {
  return double(double(double))(inc)(5);
}
