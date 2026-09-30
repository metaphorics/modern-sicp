// SPDX-License-Identifier: GPL-3.0-only
// case core/03-factorial-iterative: SICP 1.2.1 iterative factorial: state variables updated by a while loop.
function factorial(n: number): number {
  let product = 1;
  let counter = 1;
  while (counter <= n) {
    product = product * counter;
    counter = counter + 1;
  }
  return product;
}
console.log(factorial(6));
