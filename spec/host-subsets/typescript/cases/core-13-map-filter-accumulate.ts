// SPDX-License-Identifier: GPL-3.0-only
// case core/13-map-filter-accumulate: SICP 2.2.3 sequences as conventional interfaces: map, filter, accumulate with closures.
type List = { readonly tag: "nil" } | { readonly tag: "cons"; readonly head: number; readonly tail: List };
const nil: List = { tag: "nil" };
const cons = (head: number, tail: List): List => ({ tag: "cons", head, tail });
function map(f: (x: number) => number, items: List): List {
  return items.tag === "nil" ? nil : cons(f(items.head), map(f, items.tail));
}
function filter(keep: (x: number) => boolean, items: List): List {
  if (items.tag === "nil") {
    return nil;
  }
  return keep(items.head) ? cons(items.head, filter(keep, items.tail)) : filter(keep, items.tail);
}
function accumulate(op: (x: number, acc: number) => number, initial: number, items: List): number {
  return items.tag === "nil" ? initial : op(items.head, accumulate(op, initial, items.tail));
}
function enumerateInterval(low: number, high: number): List {
  return low > high ? nil : cons(low, enumerateInterval(low + 1, high));
}
const evens = filter((x: number): boolean => x % 2 === 0, enumerateInterval(1, 6));
const squares = map((x: number): number => x * x, evens);
console.log(accumulate((x: number, acc: number): number => x + acc, 0, squares));
console.log(accumulate((x: number, acc: number): number => x * acc, 1, enumerateInterval(1, 5)));
