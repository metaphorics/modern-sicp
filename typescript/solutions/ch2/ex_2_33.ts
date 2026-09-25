// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";
import { accumulate, cons, nil } from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.33: fill in the missing accumulation arguments so that map,
 * append, and length are listed as accumulations. Each list operation is
 * a fold-right: the element `x` combines with the already-built rest `y`
 * by consing the transformed element, consing onto `seq2`, or counting
 * one more.
 */

/** The book's map, rebuilt as an accumulation. */
export const mapViaAccumulate = (f: (x: number) => number, sequence: List<number>): List<number> =>
  accumulate<number, List<number>>((x, y) => cons(f(x), y), nil, sequence);

/** The book's append, rebuilt as an accumulation over `seq1`. */
export const appendViaAccumulate = (seq1: List<number>, seq2: List<number>): List<number> =>
  accumulate((x: number, y: List<number>) => cons(x, y), seq2, seq1);

/** The book's length, rebuilt as an accumulation. */
export const lengthViaAccumulate = (sequence: List<number>): number =>
  accumulate((_x, y) => y + 1, 0, sequence);
