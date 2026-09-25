// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Result } from "../../packages/ch2/src/01-data-abstraction.js";
import { type List, list } from "../../packages/ch2/src/02-picture-language.js";
import {
  decode,
  type HuffTree,
  makeCodeTree,
  makeLeaf,
  type Symb,
  showSymbols,
  sym,
} from "../../packages/ch2/src/03-symbolic-data.js";

/**
 * Exercise 2.67: the book's sample tree and sample message, decoded.
 * The tree gives A the 1-bit code, B the 2-bit code, and D and C the
 * two 3-bit codes; the 13-bit sample message decodes to (A D A B B C A).
 */

export const sampleTree: HuffTree = makeCodeTree(
  makeLeaf(sym("A"), 4),
  makeCodeTree(makeLeaf(sym("B"), 2), makeCodeTree(makeLeaf(sym("D"), 1), makeLeaf(sym("C"), 1))),
);

export const sampleMessage: List<number> = list(0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0);

/** The decode of the sample message: the exercise's answer. */
export const decodedSample = decode(sampleMessage, sampleTree);

/** Prints a successful decode as the book's symbol list. */
export const printDecoded = (decoded: Result<List<Symb>, string>): string =>
  decoded._tag === "Ok" ? showSymbols(decoded.value) : `Error: ${decoded.error}`;
