// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 5.3

/**
 * The list-structured memory of section 5.3. The book's two vectors
 * `the-cars` and `the-cdrs` are the fields of one `Memory` record beside the
 * 5.2 simulator, a pointer to a pair is an index into the two vectors, and
 * the typed pointers of 5.3.1 are tagged words: the type field rides as the
 * word's `symbol` tag, so the pointer to the pair with index 5 is
 * `{ symbol: "p", index: 5 }`, drawn `p5`. Every word is a simulator `Value`,
 * so the words ride unmodified through the 5.2 machine's registers, stack,
 * and operations table; `listOperations` installs the list-structure
 * operations of 5.3.1 as machine primitives, the book's own assumption for
 * the exercises of this section. The allocation path is the book's expansion
 * of `cons`: store the two arguments at the free pointer, hand back the
 * pointer, increment the free pointer. An exhausted memory, an index outside
 * a vector, and a selector handed a non-pair are the section's typed faults.
 */

import type { Value } from "./02-simulator.js";

export type { Value };

/** The typed pointer to the pair stored at cell `index`, the book's `p5`:
 * the type field is the `symbol` tag, the index the payload. */
export interface PairPointer {
  readonly symbol: "p";
  readonly index: number;
}

/** The empty list, the book's `e0`: the type-e pointer to cell 0. */
export interface EmptyList {
  readonly symbol: "e";
  readonly index: 0;
}

/** One memory word, a simulator value seen as the book's typed pointer:
 * a pair pointer names a cell, a number is its own payload, a symbol is the
 * simulator's symbol word, and `e0` is the empty list. */
export type Word = Value;

/** The pointer to the pair with index `index`. */
export const pairPointer = (index: number): PairPointer => ({ symbol: "p", index });

/** The empty list, `e0`. */
export const emptyList: EmptyList = { symbol: "e", index: 0 };

/** True when the word is a pair pointer: the `pair?` predicate, which
 * checks only the type field. */
export const isPairPointer = (word: Value): word is PairPointer =>
  typeof word === "object" &&
  "index" in word &&
  word.symbol === "p" &&
  typeof word.index === "number";

/** True when the word is the empty list pointer `e0`: the `null?`
 * predicate. */
export const isEmptyList = (word: Value): word is EmptyList =>
  typeof word === "object" && "index" in word && word.symbol === "e" && word.index === 0;

/** True when the word is a symbol word: the `symbol?` predicate. Two
 * instances of a symbol are the same word, the obarray's interning reduced
 * to the simulator's symbol words. */
export const isSymbolWord = (word: Value): boolean =>
  typeof word === "object" && !("index" in word);

/** True when the word is a number word: the `number?` predicate. */
export const isNumberWord = (word: Value): boolean => typeof word === "number";

/** The book's `eq?` over typed words: two data objects are the same when
 * their pointers are identical, the same type field and the same index;
 * numbers compare by value and symbols by name. */
export const eqWords = (left: Value, right: Value): boolean => {
  if (isPairPointer(left) || isPairPointer(right))
    return isPairPointer(left) && isPairPointer(right) && left.index === right.index;
  if (isEmptyList(left) || isEmptyList(right)) return isEmptyList(left) && isEmptyList(right);
  if (typeof left === "object" || typeof right === "object")
    return (
      typeof left === "object" &&
      typeof right === "object" &&
      !("index" in left) &&
      !("index" in right) &&
      left.symbol === right.symbol
    );
  return left === right;
};

/** Renders a word the way the book writes pointers in the memory vector:
 * `p5` for the pair with index 5, `n4` for the number 4, `e0` for the
 * empty list, a symbol as its name. */
export const renderWord = (word: Value): string => {
  if (isPairPointer(word)) return `p${word.index}`;
  if (isEmptyList(word)) return "e0";
  if (typeof word === "object") return word.symbol;
  if (typeof word === "boolean") return String(word);
  return `n${word}`;
};

// The typed faults of the allocation path and the selectors.

/** Why a memory primitive refused: the free pointer reached the end, a
 * vector index left the vector, or a selector was handed a word that is
 * not a pair pointer. */
export type MemoryError =
  | { readonly tag: "MemoryExhausted"; readonly free: number; readonly size: number }
  | {
      readonly tag: "IndexOutOfRange";
      readonly op: string;
      readonly index: number;
      readonly size: number;
    }
  | { readonly tag: "NotAPair"; readonly op: string; readonly word: Value };

/** Renders a fault the way the memory's reports read. */
export const renderMemoryError = (error: MemoryError): string => {
  switch (error.tag) {
    case "MemoryExhausted":
      return "the memory is exhausted";
    case "IndexOutOfRange":
      return `${error.op}: memory index out of range: ${error.index}`;
    case "NotAPair":
      return `${error.op}: not a pair: ${renderWord(error.word)}`;
  }
};

/** The fault the memory primitives raise. The 5.2 machine's operations
 * signal run-time faults as host exceptions (the arithmetic throws the
 * same way), so the typed fault rides as a payload on the host error. */
export class MemoryFault extends Error {
  readonly fault: MemoryError;

  constructor(fault: MemoryError) {
    super(renderMemoryError(fault));
    this.name = "MemoryFault";
    this.fault = fault;
  }
}

const fault = (error: MemoryError): never => {
  throw new MemoryFault(error);
};

// The list-structured memory: the two vectors and the free pointer.

/** The list-structured memory of 5.3.1. A fresh cell holds `e0`, the
 * book's blank locations; `free` starts where the exercise says, and
 * exercise 5.20 starts its drawing at `p1`. */
export interface Memory {
  readonly size: number;
  free: number;
  readonly theCars: Value[];
  readonly theCdrs: Value[];
}

/** A fresh memory of `size` cells allocating from `free`. */
export const makeMemory = (size: number, free = 0): Memory => ({
  size,
  free,
  theCars: Array.from({ length: size }, () => emptyList),
  theCdrs: Array.from({ length: size }, () => emptyList),
});

/** The book's `vector-ref` over `the-cars`: a bounds-checked read. */
export const readTheCars = (memory: Memory, index: number): Value => {
  const word = memory.theCars[index];
  if (word === undefined)
    return fault({ tag: "IndexOutOfRange", op: "vector-ref", index, size: memory.size });
  return word;
};

/** The book's `vector-ref` over `the-cdrs`. */
export const readTheCdrs = (memory: Memory, index: number): Value => {
  const word = memory.theCdrs[index];
  if (word === undefined)
    return fault({ tag: "IndexOutOfRange", op: "vector-ref", index, size: memory.size });
  return word;
};

/** The book's `vector-set!` over `the-cars`. */
export const storeTheCars = (memory: Memory, index: number, word: Value): void => {
  if (index < 0 || index >= memory.size)
    fault({ tag: "IndexOutOfRange", op: "vector-set!", index, size: memory.size });
  memory.theCars[index] = word;
};

/** The book's `vector-set!` over `the-cdrs`. */
export const storeTheCdrs = (memory: Memory, index: number, word: Value): void => {
  if (index < 0 || index >= memory.size)
    fault({ tag: "IndexOutOfRange", op: "vector-set!", index, size: memory.size });
  memory.theCdrs[index] = word;
};

// The primitive list operations of 5.3.1.

/** The allocation path: store the two arguments at the free pointer's
 * index, hand back the pointer, increment the free pointer. An exhausted
 * memory is the typed fault. */
export const cons = (memory: Memory, carWord: Value, cdrWord: Value): PairPointer => {
  if (memory.free >= memory.size)
    fault({ tag: "MemoryExhausted", free: memory.free, size: memory.size });
  const index = memory.free;
  memory.theCars[index] = carWord;
  memory.theCdrs[index] = cdrWord;
  memory.free = index + 1;
  return pairPointer(index);
};

/** The book's `car`: the entry in `the-cars` at the pointer's index. */
export const car = (memory: Memory, word: Value): Value =>
  isPairPointer(word)
    ? readTheCars(memory, word.index)
    : fault({ tag: "NotAPair", op: "car", word });

/** The book's `cdr`: the entry in `the-cdrs` at the pointer's index. */
export const cdr = (memory: Memory, word: Value): Value =>
  isPairPointer(word)
    ? readTheCdrs(memory, word.index)
    : fault({ tag: "NotAPair", op: "cdr", word });

/** The book's `set-car!`, answering the unspecified word, `e0`. */
export const setCar = (memory: Memory, word: Value, value: Value): Value => {
  if (!isPairPointer(word)) return fault({ tag: "NotAPair", op: "set-car!", word });
  storeTheCars(memory, word.index, value);
  return emptyList;
};

/** The book's `set-cdr!`, answering the unspecified word, `e0`. */
export const setCdr = (memory: Memory, word: Value, value: Value): Value => {
  if (!isPairPointer(word)) return fault({ tag: "NotAPair", op: "set-cdr!", word });
  storeTheCdrs(memory, word.index, value);
  return emptyList;
};

// Reading structure back: the surface form and the memory-vector drawing.

/** The book's `write`: the surface form of the structure `word`
 * designates, walking `car` and `cdr` through the memory, so a planted
 * list reads back as `(1 2 3)`. */
export const write = (memory: Memory, word: Value): string => {
  if (!isPairPointer(word)) {
    if (isEmptyList(word)) return "()";
    return typeof word === "number" ? String(word) : renderWord(word);
  }
  const rest = readTheCdrs(memory, word.index);
  if (!isPairPointer(rest) && !isEmptyList(rest))
    return `(${write(memory, readTheCars(memory, word.index))} . ${write(memory, rest)})`;
  const parts: string[] = [];
  let cursor: Value = word;
  while (isPairPointer(cursor)) {
    parts.push(write(memory, readTheCars(memory, cursor.index)));
    cursor = readTheCdrs(memory, cursor.index);
  }
  if (isEmptyList(cursor)) return `(${parts.join(" ")})`;
  return `(${parts.join(" ")} . ${write(memory, cursor)})`;
};

/** The memory-vector drawing, the lower half of the book's Figure 5.14:
 * one column per cell, the index, then `the-cars`, then `the-cdrs`. */
export const dump = (memory: Memory): string => {
  const rows: ReadonlyArray<[string, ReadonlyArray<string>]> = [
    ["index", memory.theCars.map((_, index) => String(index))],
    ["the-cars", memory.theCars.map(renderWord)],
    ["the-cdrs", memory.theCdrs.map(renderWord)],
  ];
  const labelWidth = "the-cars".length + 1;
  const widths = memory.theCars.map(
    (_, index) => Math.max(...rows.map(([, cells]) => (cells[index] ?? "").length)) + 2,
  );
  return rows
    .map(([rowLabel, cells]) => {
      let line = rowLabel.padEnd(labelWidth);
      cells.forEach((cell, index) => {
        line += cell.padEnd(widths[index] ?? 2);
      });
      return line.trimEnd();
    })
    .join("\n");
};

// The operations table: the list operations as 5.2 machine primitives.

/** The list-structure operations of 5.3.1 as a machine operations table:
 * the selectors and mutators go through the index part of a pair pointer,
 * `cons` through the free pointer, and the predicates check only the type
 * field. Combine with `arithmeticOperations` for the arithmetic a machine
 * also names. */
export const listOperations = (
  memory: Memory,
): Record<string, (args: ReadonlyArray<Value>) => Value> => ({
  cons: (args) => cons(memory, args[0] ?? emptyList, args[1] ?? emptyList),
  car: (args) => car(memory, args[0] ?? emptyList),
  cdr: (args) => cdr(memory, args[0] ?? emptyList),
  "set-car!": (args) => setCar(memory, args[0] ?? emptyList, args[1] ?? emptyList),
  "set-cdr!": (args) => setCdr(memory, args[0] ?? emptyList, args[1] ?? emptyList),
  "eq?": (args) => eqWords(args[0] ?? emptyList, args[1] ?? emptyList),
  "pair?": (args) => isPairPointer(args[0] ?? emptyList),
  "null?": (args) => isEmptyList(args[0] ?? emptyList),
  "symbol?": (args) => isSymbolWord(args[0] ?? emptyList),
  "number?": (args) => isNumberWord(args[0] ?? emptyList),
});
