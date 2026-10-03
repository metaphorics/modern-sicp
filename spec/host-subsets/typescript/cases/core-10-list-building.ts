// SPDX-License-Identifier: GPL-3.0-only
// case core/10-list-building: SICP 2.2.1 lists built from tagged cons/nil records and consumed recursively.
type List = { readonly tag: "nil" } | { readonly tag: "cons"; readonly head: number; readonly tail: List };
const nil: List = { tag: "nil" };
const cons = (head: number, tail: List): List => ({ tag: "cons", head, tail });
function sum(items: List): number {
  return items.tag === "nil" ? 0 : items.head + sum(items.tail);
}
function length(items: List): number {
  return items.tag === "nil" ? 0 : 1 + length(items.tail);
}
const oneToFour = cons(1, cons(2, cons(3, cons(4, nil))));
console.log(sum(oneToFour));
console.log(length(oneToFour));
console.log(sum(cons(5, cons(6, nil))));
