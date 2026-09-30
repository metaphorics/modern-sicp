// SPDX-License-Identifier: GPL-3.0-only
// case compiler/01-primitive-combination: SICP 5.5 compiling a primitive combination whose operands need register preservation.
function combine(a: number, b: number): number {
  return (a + b) * (a - b);
}
console.log(combine(10, 4));
