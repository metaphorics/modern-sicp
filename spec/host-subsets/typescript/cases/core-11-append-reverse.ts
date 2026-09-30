// SPDX-License-Identifier: GPL-3.0-only
// case core/11-append-reverse: SICP 2.2.1 append and reverse over cons lists.
type List = { readonly tag: "nil" } | { readonly tag: "cons"; readonly head: number; readonly tail: List };
const nil: List = { tag: "nil" };
const cons = (head: number, tail: List): List => ({ tag: "cons", head, tail });
function append(left: List, right: List): List {
  return left.tag === "nil" ? right : cons(left.head, append(left.tail, right));
}
function reverse(items: List): List {
  return items.tag === "nil" ? nil : append(reverse(items.tail), cons(items.head, nil));
}
function show(items: List): string {
  if (items.tag === "nil") {
    return "";
  }
  if (items.tail.tag === "nil") {
    return `${items.head}`;
  }
  return `${items.head},` + show(items.tail);
}
console.log(show(append(cons(1, cons(2, nil)), cons(3, cons(4, nil)))));
console.log(show(reverse(cons(1, cons(2, cons(3, nil))))));
