// SPDX-License-Identifier: GPL-3.0-only
// case core/17-dotted-pairs: SICP 2.1.3 pairs as data: car/cdr selectors over nested pair records.
interface Pair {
  readonly car: number | Pair;
  readonly cdr: number | Pair;
}
const cons = (car: number | Pair, cdr: number | Pair): Pair => ({ car, cdr });
function show(value: number | Pair): string {
  return typeof value === "number" ? `${value}` : "(" + show(value.car) + " . " + show(value.cdr) + ")";
}
const pair = cons(cons(1, 2), cons(3, 4));
console.log(show(pair));
const left = pair.car;
console.log(typeof left === "number" ? left : show(left.cdr));
