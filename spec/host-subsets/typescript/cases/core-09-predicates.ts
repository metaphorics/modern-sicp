// SPDX-License-Identifier: GPL-3.0-only
// case core/09-predicates: SICP 1.1.6 predicates and short-circuit `&&`/`||`: the right operand runs only when needed.
function isEven(n: number): boolean {
  return n % 2 === 0;
}
function noisy(flag: boolean): boolean {
  console.log("evaluated");
  return flag;
}
console.log(isEven(4));
console.log(isEven(7) && noisy(true));
console.log(isEven(4) || noisy(false));
console.log(isEven(4) && noisy(false));
console.log(!isEven(3));
