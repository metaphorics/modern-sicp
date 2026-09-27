// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type List, length, list } from "../../packages/ch2/src/02-picture-language.js";
import { type Symb, type SymbolFrequency, sym } from "../../packages/ch2/src/03-symbolic-data.js";
import { encodeSymbol } from "./ex_2_68.js";
import { generateHuffmanTree } from "./ex_2_69.js";

/**
 * Exercise 2.71: the geometric alphabet S0..S(n-1) with frequencies
 * 1, 2, 4, ..., 2^(n-1). Each merge unites the accumulated small tree
 * with the next power of two, so the tree is a maximally skewed chain:
 * the most frequent symbol sits at depth 1 and the least frequent at
 * depth n-1, giving 1-bit and (n-1)-bit codes.
 */

const symN = (i: number): Symb => sym(`S${String(i)}`);

/** The pairs ((S0 1) (S1 2) ... (S(n-1) 2^(n-1))). */
export const geometricPairs = (n: number): List<SymbolFrequency> => {
  const pairs: SymbolFrequency[] = [];
  for (let i = 0; i < n; i += 1) {
    pairs.push([symN(i), 2 ** i]);
  }
  return list(...pairs);
};

/** The generated tree, or an error for degenerate n. */
export const geometricTree = (n: number) => generateHuffmanTree(geometricPairs(n));

/** Bits for the most frequent symbol: 1. */
export const mostFrequentBits = (n: number): number => {
  const tree = geometricTree(n);
  if (tree._tag !== "Ok") {
    return 0;
  }
  const bits = encodeSymbol(symN(n - 1), tree.value);
  return bits._tag === "Ok" ? length(bits.value) : 0;
};

/** Bits for the least frequent symbol: n - 1. */
export const leastFrequentBits = (n: number): number => {
  const tree = geometricTree(n);
  if (tree._tag !== "Ok") {
    return 0;
  }
  const bits = encodeSymbol(symN(0), tree.value);
  return bits._tag === "Ok" ? length(bits.value) : 0;
};
