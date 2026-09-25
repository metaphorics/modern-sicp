// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";
import {
  enumerateInterval,
  filter,
  flatmap,
  list,
  map,
} from "../../packages/ch2/src/02-picture-language.js";

/**
 * Exercise 2.41: the ordered triples of distinct integers from 1..n
 * whose sum is a given s. Three nested mappings over the interval pick
 * i, then j, then k; the innermost map emits `list(i, j, k)` for the k
 * that survives the distinctness-and-sum filter, and flatmap flattens
 * the two outer levels.
 */

/** The ordered triples (i j k) of distinct 1..n integers summing to `s`. */
export const orderedTriples = (n: number, s: number): List<List<number>> =>
  flatmap(
    (i) =>
      flatmap(
        (j) =>
          map(
            (k) => list(i, j, k),
            filter(
              (k) => i !== j && k !== i && k !== j && i + j + k === s,
              enumerateInterval(1, n),
            ),
          ),
        enumerateInterval(1, n),
      ),
    enumerateInterval(1, n),
  );
