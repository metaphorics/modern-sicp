// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";
import {
  enumerateInterval,
  filter,
  flatmap,
  list,
  makePairSum,
  map,
  primeSum,
} from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.40: define unique-pairs so prime-sum-pairs reads as its
 * composition. The pair generation - i from 1..n, j from 1..i-1 - moves
 * into unique-pairs, leaving prime-sum-pairs a filter, a map, and
 * nothing about pairing order left in the open.
 */

/** The pairs (i j) with 1 <= j < i <= n, i descending outermost. */
export const uniquePairs = (n: number): List<List<number>> =>
  flatmap((i) => map((j) => list(i, j), enumerateInterval(1, i - 1)), enumerateInterval(1, n));

/** The book's prime-sum-pairs, assembled from unique-pairs. */
export const primeSumPairsSimplified = (n: number): List<List<number>> =>
  map(makePairSum, filter(primeSum, uniquePairs(n)));
