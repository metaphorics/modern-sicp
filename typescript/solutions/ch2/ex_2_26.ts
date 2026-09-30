// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  append,
  cons,
  type List,
  list,
  type Showable,
  showList,
} from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.26: with x = [1, 2, 3] and y = [4, 5, 6], what does each
 * combination print? Each function builds the value with the module's
 * own combinator and reads the printed form back. The cons of two
 * lists is the mixed spine the book's picture shows, typed here as
 * List<Showable>.
 */

/** append concatenates: one six-element list. */
export function appendResult(): string {
  const x: List<number> = list(1, 2, 3);
  const y: List<number> = list(4, 5, 6);
  return showList(append(x, y));
}

/** cons makes x the first element of a list over y's spine. */
export function consResult(): string {
  const x: List<number> = list(1, 2, 3);
  const y: List<number> = list(4, 5, 6);
  return showList(cons<Showable>(x, y));
}

/** list makes a two-element list of the lists. */
export function listResult(): string {
  const x: List<number> = list(1, 2, 3);
  const y: List<number> = list(4, 5, 6);
  return showList(list<Showable>(x, y));
}
