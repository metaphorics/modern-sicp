// SPDX-License-Identifier: GPL-3.0-only
// case core/14-higher-order: SICP 1.3 procedures as arguments and returned values: compose and twice.
const compose = (f: (x: number) => number, g: (x: number) => number): ((x: number) => number) =>
  (x: number): number => f(g(x));
const twice = (f: (x: number) => number): ((x: number) => number) => compose(f, f);
const inc = (x: number): number => x + 1;
function sumTerms(term: (x: number) => number, a: number, next: (x: number) => number, b: number): number {
  return a > b ? 0 : term(a) + sumTerms(term, next(a), next, b);
}
console.log(twice(inc)(5));
console.log(compose((x: number): number => x * 2, inc)(5));
console.log(sumTerms((x: number): number => x * x * x, 1, inc, 4));
