// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";
import { accumulate, append, cons, list, nil } from "../../packages/ch2/src/02-picture-language.js";
import { foldLeft } from "./ex_2_38.js";

/**
 * Exercise 2.39: reverse in terms of fold-right and fold-left. Fold-right
 * meets the first element first, so it cannot cons in front: each
 * element must be appended behind the already-reversed rest. Fold-left
 * meets the elements in order, so it conses each one onto the front of
 * the accumulated result, which is why reverse is fold-left's natural
 * shape. The fold-left version imports exercise 2.38's foldLeft rather
 * than redefining it; the module's accumulate stands in for fold-right.
 */

/** Reverses `sequence` by folding right, appending each element last. */
export const reverseViaFoldRight = (sequence: List<number>): List<number> =>
  accumulate<number, List<number>>((x, y) => append(y, list(x)), nil, sequence);

/** Reverses `sequence` by folding left, consing each element in front. */
export const reverseViaFoldLeft = (sequence: List<number>): List<number> =>
  foldLeft<number, List<number>>((x, y) => cons(y, x), nil, sequence);
