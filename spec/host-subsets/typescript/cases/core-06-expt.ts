// SPDX-License-Identifier: GPL-3.0-only
// case core/06-expt: SICP 1.2.4 exponentiation: linear recursion and successive squaring.
function expt(b: number, n: number): number {
  return n === 0 ? 1 : b * expt(b, n - 1);
}
const isEven = (n: number): boolean => n % 2 === 0;
function fastExpt(b: number, n: number): number {
  if (n === 0) {
    return 1;
  }
  if (isEven(n)) {
    const half = fastExpt(b, n / 2);
    return half * half;
  }
  return b * fastExpt(b, n - 1);
}
console.log(expt(2, 10));
console.log(fastExpt(3, 7));
