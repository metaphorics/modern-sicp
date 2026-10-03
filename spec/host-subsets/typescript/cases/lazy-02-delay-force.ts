// SPDX-License-Identifier: GPL-3.0-only
// case lazy/02-delay-force: SICP 4.2.2 (exercise 4.27) explicit delay/force: a memoized thunk runs its body once however often it is forced.
let count = 0;
function id(x: number): number {
  count = count + 1;
  return x;
}
const w = delay(id(id(10)));
console.log(count);
console.log(force(w));
console.log(count);
console.log(force(w));
console.log(count);
