// SPDX-License-Identifier: GPL-3.0-only
// case core/02-factorial-recursive: SICP 1.2.1 linear recursive factorial through host calls.
function factorial(n: number): number {
  if (n === 0) {
    return 1;
  }
  return n * factorial(n - 1);
}
console.log(factorial(6));
