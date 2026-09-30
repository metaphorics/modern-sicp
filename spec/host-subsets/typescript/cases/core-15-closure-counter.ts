// SPDX-License-Identifier: GPL-3.0-only
// case core/15-closure-counter: SICP 3.1.1 local state: a closure shares its captured `let` cell across calls.
function makeCounter(): () => number {
  let count = 0;
  return (): number => {
    count = count + 1;
    return count;
  };
}
const c1 = makeCounter();
const c2 = makeCounter();
c1();
c1();
console.log(c1());
console.log(c2());
