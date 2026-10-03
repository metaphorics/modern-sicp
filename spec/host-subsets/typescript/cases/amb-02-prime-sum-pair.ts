// SPDX-License-Identifier: GPL-3.0-only
// case amb/02-prime-sum-pair: SICP 4.3.1 prime-sum-pair: an-element-of choice points filtered by a primality requirement.
type List = { readonly tag: "nil" } | { readonly tag: "cons"; readonly head: number; readonly tail: List };
const nil: List = { tag: "nil" };
const cons = (head: number, tail: List): List => ({ tag: "cons", head, tail });
function isPrime(k: number): boolean {
  let divisor = 2;
  while (divisor * divisor <= k) {
    if (k % divisor === 0) {
      return false;
    }
    divisor = divisor + 1;
  }
  return k > 1;
}
function anElementOf(items: List): number {
  if (items.tag === "nil") {
    require(false);
    return 0;
  }
  return choose(items.head, anElementOf(items.tail));
}
const a = anElementOf(cons(1, cons(3, cons(5, cons(8, nil)))));
const b = anElementOf(cons(20, cons(35, cons(110, nil))));
require(isPrime(a + b));
console.log(`${a} ${b}`);
