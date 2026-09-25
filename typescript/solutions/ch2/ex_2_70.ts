// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type List, length, list } from "../../packages/ch2/src/02-picture-language.js";
import {
  type HuffTree,
  makeLeaf,
  type Symb,
  type SymbolFrequency,
  sym,
} from "../../packages/ch2/src/03-symbolic-data.js";
import { encode } from "./ex_2_68.js";
import { generateHuffmanTree } from "./ex_2_69.js";

/**
 * Exercise 2.70: the 1950s rock song. Generate the tree from the
 * eight-symbol alphabet, encode the 36-symbol song, and count the
 * bits: 84 against the fixed-length code's 108, the book's answer.
 */

export const rockPairs: List<SymbolFrequency> = list(
  [sym("A"), 2] as const,
  [sym("NA"), 16] as const,
  [sym("BOOM"), 1] as const,
  [sym("SHA"), 3] as const,
  [sym("GET"), 2] as const,
  [sym("YIP"), 9] as const,
  [sym("JOB"), 2] as const,
  [sym("WAH"), 1] as const,
);

export const song: List<Symb> = list(
  sym("GET"),
  sym("A"),
  sym("JOB"),
  sym("SHA"),
  sym("NA"),
  sym("NA"),
  sym("NA"),
  sym("NA"),
  sym("NA"),
  sym("NA"),
  sym("NA"),
  sym("NA"),
  sym("GET"),
  sym("A"),
  sym("JOB"),
  sym("SHA"),
  sym("NA"),
  sym("NA"),
  sym("NA"),
  sym("NA"),
  sym("NA"),
  sym("NA"),
  sym("NA"),
  sym("NA"),
  sym("WAH"),
  sym("YIP"),
  sym("YIP"),
  sym("YIP"),
  sym("YIP"),
  sym("YIP"),
  sym("YIP"),
  sym("YIP"),
  sym("YIP"),
  sym("YIP"),
  sym("SHA"),
  sym("BOOM"),
);

/** The generated tree, or a zero-weight placeholder if generation failed. */
export const rockTree: HuffTree = (() => {
  const generated = generateHuffmanTree(rockPairs);
  return generated._tag === "Ok" ? generated.value : makeLeaf(sym("?"), 0);
})();

/** The encoded song, or the empty bit list if encoding failed. */
export const encodedSong: List<number> = (() => {
  const bits = encode(song, rockTree);
  return bits._tag === "Ok" ? bits.value : list();
})();

/** Bits for the Huffman encoding: the book's 84. */
export const songBits: number = length(encodedSong);

/** Bits for the best fixed-length code over eight symbols: 36 * 3. */
export const fixedBits: number = length(song) * 3;
