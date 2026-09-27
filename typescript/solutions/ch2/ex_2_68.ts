// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { err, ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";
import { append, cons, type List, nil } from "../../packages/ch2/src/02-picture-language.js";
import type { HuffTree, Symb } from "../../packages/ch2/src/03-symbolic-data.js";

/**
 * Exercise 2.68: `encodeSymbol` walks from the root, prefixing 0 for a
 * left descent and 1 for a right descent, and returns an error when the
 * symbol is nowhere in the tree. `encode` is the book's given procedure,
 * threading the error channel this edition adds.
 */

export const encodeSymbol = (symbol: Symb, tree: HuffTree): Result<List<number>, string> => {
  if (tree._tag === "Leaf") {
    return tree.symbol === symbol ? ok(nil) : err(`symbol not in tree: ${String(symbol)}`);
  }
  const leftBits = encodeSymbol(symbol, tree.left);
  if (leftBits._tag === "Ok") {
    return ok(cons(0, leftBits.value));
  }
  const rightBits = encodeSymbol(symbol, tree.right);
  if (rightBits._tag === "Ok") {
    return ok(cons(1, rightBits.value));
  }
  return err(`symbol not in tree: ${String(symbol)}`);
};

export const encode = (message: List<Symb>, tree: HuffTree): Result<List<number>, string> => {
  if (message._tag === "Nil") {
    return ok(nil);
  }
  const first = encodeSymbol(message.head, tree);
  if (first._tag === "Error") {
    return first;
  }
  const rest = encode(message.tail, tree);
  if (rest._tag === "Error") {
    return rest;
  }
  return ok(append(first.value, rest.value));
};
