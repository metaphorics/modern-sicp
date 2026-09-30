// SPDX-License-Identifier: GPL-3.0-only
// case core/07-fibonacci: SICP 1.2.2 tree-recursive Fibonacci.
function fib(n: number): number {
  return n < 2 ? n : fib(n - 1) + fib(n - 2);
}
console.log(fib(10));
