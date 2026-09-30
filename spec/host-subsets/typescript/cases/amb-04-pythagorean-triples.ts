// SPDX-License-Identifier: GPL-3.0-only
// case amb/04-pythagorean-triples: SICP 4.3.2 (exercise 4.35) Pythagorean triples: nested an-integer-between choices, each bounded by the previous one.
function anIntegerBetween(low: number, high: number): number {
  require(low <= high);
  return choose(low, anIntegerBetween(low + 1, high));
}
const i = anIntegerBetween(1, 13);
const j = anIntegerBetween(i, 13);
const k = anIntegerBetween(j, 13);
require(i * i + j * j === k * k);
console.log(`${i} ${j} ${k}`);
