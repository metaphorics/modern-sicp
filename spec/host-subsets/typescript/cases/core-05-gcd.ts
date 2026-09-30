// SPDX-License-Identifier: GPL-3.0-only
// case core/05-gcd: SICP 1.2.5 Euclid's algorithm by remainder recursion.
function gcd(a: number, b: number): number {
  return b === 0 ? a : gcd(b, a % b);
}
console.log(gcd(206, 40));
console.log(gcd(28, 16));
