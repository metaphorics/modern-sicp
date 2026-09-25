// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  cons,
  type List,
  leaf,
  nil,
  node,
  square,
  type Tree,
} from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.22: Louis Reasoner's two iterative rewrites of
 * square-list, and why neither produces (1 4 9 16). The first conses
 * each square onto the front of the answer, so the squares come out in
 * reverse order. In the second the cons arguments are interchanged; the
 * literal cons(answer, square) does not typecheck against the edition's
 * List — a List<number> is not a number — so the shape it builds is
 * shown with the tree union: the answer starts as the empty node and
 * every step wraps it around the next square.
 */

/** The first rewrite: the right iterative process, reversed order. */
export function squareListIter(items: List<number>): List<number> {
  let answer: List<number> = nil;
  let rest = items;
  while (rest._tag === "Cons") {
    answer = cons(square(rest.head), answer);
    rest = rest.tail;
  }
  return answer;
}

/** The second rewrite's shape: the answer grows to the left of each square. */
export function squareListSwapped(items: List<number>): Tree<number> {
  let answer: Tree<number> = node<number>();
  let rest = items;
  while (rest._tag === "Cons") {
    answer = node(answer, leaf(square(rest.head)));
    rest = rest.tail;
  }
  return answer;
}
