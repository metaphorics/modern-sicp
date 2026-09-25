// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type List, length, list } from "../../packages/ch2/src/02-picture-language.js";
import {
  type HuffTree,
  type Symb,
  type SymbolFrequency,
  sym,
  symbolsOf,
} from "../../packages/ch2/src/03-symbolic-data.js";
import { generateHuffmanTree } from "./ex_2_69.js";

/**
 * Exercise 2.72: the order of growth of `encodeSymbol`. The dominant
 * cost at each node visited is the membership search over a branch's
 * symbol list. The counter below charges each scan its full list
 * length (the worst case the book's question asks about); the early
 * exit of a hit only makes real runs cheaper. On exercise 2.71's
 * skewed alphabet the most frequent symbol costs one scan of the whole
 * alphabet, Theta(n); the least frequent sits at depth n-1 and pays a
 * scan at every level down, summing to Theta(n^2).
 */

const listHas = (items: List<Symb>, target: Symb): boolean =>
  items._tag === "Cons" && (items.head === target || listHas(items.tail, target));

/** Counts `encodeSymbol`'s steps for one symbol under the worst-case
 * scan convention: a leaf costs nothing, and each branch visited costs
 * the full length of both branch symbol lists it consults --- the size
 * of the subtree rooted there. */
export const encodeSymbolSteps = (target: Symb, tree: HuffTree): number => {
  if (tree._tag === "Leaf") {
    return 0;
  }
  const visitCost = length(symbolsOf(tree.left)) + length(symbolsOf(tree.right));
  if (listHas(symbolsOf(tree.left), target)) {
    return visitCost + encodeSymbolSteps(target, tree.left);
  }
  return visitCost + encodeSymbolSteps(target, tree.right);
};

/** The measured pair (most frequent, least frequent) at size n. */
export const stepsAt = (n: number): [number, number] => {
  const pairs: SymbolFrequency[] = [];
  for (let i = 0; i < n; i += 1) {
    pairs.push([sym(`S${String(i)}`), 2 ** i]);
  }
  const built = generateHuffmanTree(list(...pairs));
  if (built._tag !== "Ok") {
    return [0, 0];
  }
  return [
    encodeSymbolSteps(sym(`S${String(n - 1)}`), built.value),
    encodeSymbolSteps(sym("S0"), built.value),
  ];
};
