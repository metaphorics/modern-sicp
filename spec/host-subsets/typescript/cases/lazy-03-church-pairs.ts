// SPDX-License-Identifier: GPL-3.0-only
// case lazy/03-church-pairs: SICP 4.2.3 pairs as procedures over delayed components: car/cdr force only the selected part, once.
function note(x: number): number {
  console.log(`computing ${x}`);
  return x;
}
type Pair = (pick: (x: number, y: number) => number) => number;
const cons = (x: number, y: number): Pair => (pick: (x: number, y: number) => number): number => pick(x, y);
const car = (p: Pair): number => p((x: number, y: number): number => force(x));
const cdr = (p: Pair): number => p((x: number, y: number): number => force(y));
const p = cons(delay(note(1)), delay(note(2)));
console.log(car(p));
console.log(car(p));
console.log(cdr(p));
