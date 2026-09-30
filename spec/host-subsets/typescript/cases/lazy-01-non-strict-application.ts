// SPDX-License-Identifier: GPL-3.0-only
// case lazy/01-non-strict-application: SICP 4.2.1 non-strict application (lazy-memoized-experiment): delayed operands of `unless` are forced only on the chosen arm.
function note(x: number): number {
  console.log(`computing ${x}`);
  return x;
}
function unless(condition: boolean, usualValue: number, exceptionalValue: number): number {
  return condition ? force(exceptionalValue) : force(usualValue);
}
function tryIt(a: number, b: number): number {
  return a === 0 ? 1 : force(b);
}
console.log(unless(true, delay(note(1)), delay(note(2))));
console.log(unless(false, delay(note(3)), delay(note(4))));
console.log(tryIt(0, delay(note(5))));
