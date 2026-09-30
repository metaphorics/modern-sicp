// SPDX-License-Identifier: GPL-3.0-only
// case core/01-square: SICP 1.1.4 compound procedure `square` applied by call.
function square(x: number): number {
  return x * x;
}
console.log(square(12));
console.log(square(square(3)));
