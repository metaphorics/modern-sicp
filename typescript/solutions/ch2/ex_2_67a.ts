// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { err, ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import { cons, type List, list } from "../../packages/ch2/src/02-picture-language.js";
import {
  type HuffTree,
  makeCodeTree,
  makeLeaf,
  type Symb,
  sym,
} from "../../packages/ch2/src/03-symbolic-data.js";

/**
 * Exercise 2.67a: the decoding walk re-expressed as one exhaustive
 * switch over the `HuffTree` union's two tags. Standing at a leaf, the
 * switch emits the leaf's symbol and restarts at the root; standing at
 * a branch, it consumes the next bit and descends. Unlike the book's
 * `decode`, which stops silently when the bits end between symbols,
 * this version refuses such input: running off the tree is an error,
 * not an answer.
 */

export const runOffTreeMessage = "bits ran off the tree: the walk stood at a branch";

const badBit = (bit: number): string => `bad bit: DECODE-NARROWED ${String(bit)}`;

export const decodeNarrowed = (bits: List<number>, tree: HuffTree): Result<List<Symb>, string> => {
  const walk = (rest: List<number>, current: HuffTree): Result<List<Symb>, string> => {
    switch (current._tag) {
      case "Leaf": {
        if (rest._tag === "Nil") {
          return ok(list(current.symbol));
        }
        const more = walk(rest, tree);
        return more._tag === "Ok" ? ok(cons(current.symbol, more.value)) : more;
      }
      case "Branch": {
        if (rest._tag === "Nil") {
          return err(runOffTreeMessage);
        }
        const bit = rest.head;
        if (bit !== 0 && bit !== 1) {
          return err(badBit(bit));
        }
        return walk(rest.tail, bit === 0 ? current.left : current.right);
      }
    }
  };
  return walk(bits, tree);
};

/** The sample tree and message of exercise 2.67, for the pins. */
export const sampleTree: HuffTree = makeCodeTree(
  makeLeaf(sym("A"), 4),
  makeCodeTree(makeLeaf(sym("B"), 2), makeCodeTree(makeLeaf(sym("D"), 1), makeLeaf(sym("C"), 1))),
);

export const sampleMessage: List<number> = list(0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0);
