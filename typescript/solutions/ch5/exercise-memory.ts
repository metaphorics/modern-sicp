// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Operation } from "../../packages/ch5/src/01-register-machines.ts";

export type Pointer = { readonly tag: "pointer"; readonly index: number };
export type Value = number | string | boolean | null | undefined | Pointer;

export interface Memory {
  readonly size: number;
  cars: Value[];
  cdrs: Value[];
  free: number;
}

export type MemoryError =
  | { readonly tag: "MemoryExhausted"; readonly free: number; readonly size: number }
  | { readonly tag: "NotAPair"; readonly op: "car" | "cdr"; readonly word: Value }
  | {
      readonly tag: "IndexOutOfRange";
      readonly op: "vector-ref";
      readonly index: number;
      readonly size: number;
    };

export class MemoryFault extends Error {
  readonly fault: MemoryError;

  constructor(fault: MemoryError) {
    super(fault.tag);
    this.fault = fault;
  }
}

export const emptyList: Value = undefined;

export const makeMemory = (size: number, free = 0): Memory => ({
  size,
  cars: Array.from({ length: size }, () => emptyList),
  cdrs: Array.from({ length: size }, () => emptyList),
  free,
});

const isPointer = (word: Value): word is Pointer =>
  typeof word === "object" && word !== null && word.tag === "pointer";

export const pairPointer = (index: number): Pointer => ({ tag: "pointer", index });

export const renderWord = (word: Value): string => {
  if (word === undefined) return "e0";
  if (isPointer(word)) return `p${word.index}`;
  if (typeof word === "number") return `n${word}`;
  return String(word);
};

export const cons = (memory: Memory, head: Value, tail: Value): Pointer => {
  if (memory.free >= memory.size) {
    throw new MemoryFault({ tag: "MemoryExhausted", free: memory.free, size: memory.size });
  }
  const pointer = pairPointer(memory.free);
  memory.cars[memory.free] = head;
  memory.cdrs[memory.free] = tail;
  memory.free += 1;
  return pointer;
};

const pairIndex = (memory: Memory, word: Value, op: "car" | "cdr"): number => {
  if (!isPointer(word) || word.index < 0 || word.index >= memory.free) {
    throw new MemoryFault({ tag: "NotAPair", op, word });
  }
  return word.index;
};

export const car = (memory: Memory, word: Value): Value =>
  memory.cars[pairIndex(memory, word, "car")];

export const cdr = (memory: Memory, word: Value): Value =>
  memory.cdrs[pairIndex(memory, word, "cdr")];

export const readTheCars = (memory: Memory, index: number): Value => {
  if (!Number.isInteger(index) || index < 0 || index >= memory.size) {
    throw new MemoryFault({ tag: "IndexOutOfRange", op: "vector-ref", index, size: memory.size });
  }
  return memory.cars[index];
};

export const eqWords = (left: Value, right: Value): boolean =>
  isPointer(left) && isPointer(right) ? left.index === right.index : left === right;

export const dump = (memory: Memory): string => {
  const indexes = memory.cars.map((_word, index) => index);
  const cars = memory.cars.map(renderWord);
  const cdrs = memory.cdrs.map(renderWord);
  return [
    `index    ${indexes.join("   ")}`,
    `the-cars ${cars.join("  ")}`,
    `the-cdrs ${cdrs.join("  ")}`,
  ].join("\n");
};

const renderDatum = (memory: Memory, word: Value): string =>
  word === undefined ? "e0" : isPointer(word) ? write(memory, word) : String(word);

export const write = (memory: Memory, value: Value): string => {
  const items: string[] = [];
  const visited = new Set<number>();
  let tail = value;
  while (isPointer(tail)) {
    if (visited.has(tail.index)) {
      return `(${items.join(" ")}${items.length === 0 ? "" : " . "}${renderWord(tail)})`;
    }
    visited.add(tail.index);
    const head = car(memory, tail);
    items.push(renderDatum(memory, head));
    tail = cdr(memory, tail);
  }
  if (tail !== undefined) items.push(".", renderDatum(memory, tail));
  return `(${items.join(" ")})`;
};

const numberValue = (word: Value): number => {
  if (typeof word === "number") return word;
  throw new Error("the exercise arithmetic operation expected a numeric word");
};

const setCdr = (memory: Memory, pair: Value, tail: Value): Pointer => {
  const index = pairIndex(memory, pair, "cdr");
  memory.cdrs[index] = tail;
  return pairPointer(index);
};

export const listOperations = (memory: Memory): Readonly<Record<string, Operation<Value>>> => ({
  "null?": (args) => args[0] === undefined,
  "pair?": (args) => args[0] !== undefined && isPointer(args[0]),
  car: (args) => car(memory, args[0]),
  cdr: (args) => cdr(memory, args[0]),
  cons: (args) => cons(memory, args[0], args[1]),
  "set-cdr!": (args) => setCdr(memory, args[0], args[1]),
  "+": (args) => numberValue(args[0]) + numberValue(args[1]),
});
