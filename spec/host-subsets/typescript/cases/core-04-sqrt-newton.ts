// SPDX-License-Identifier: GPL-3.0-only
// case core/04-sqrt-newton: SICP 1.1.7 square roots by Newton's method over binary64 numbers.
const square = (x: number): number => x * x;
const average = (a: number, b: number): number => (a + b) / 2;
const goodEnough = (guess: number, x: number): boolean => Math.abs(square(guess) - x) < 0.001;
const improve = (guess: number, x: number): number => average(guess, x / guess);
function sqrtIter(guess: number, x: number): number {
  return goodEnough(guess, x) ? guess : sqrtIter(improve(guess, x), x);
}
console.log(sqrtIter(1, 2));
console.log(sqrtIter(1, 9));
