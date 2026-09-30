// SPDX-License-Identifier: GPL-3.0-only
// case compiler/02-factorial: SICP 5.5.5 compiled recursive factorial agrees with direct evaluation.
function factorial(n: number): number {
  return n === 0 ? 1 : n * factorial(n - 1);
}
console.log(factorial(5));
