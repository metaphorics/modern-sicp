// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: sections 5.1–5.3

/**
 * The storage allocation and garbage-collection model (host-subsets
 * grammar section 5): a finite vector store addressed by pointer words,
 * allocation through typed machine operations, and stop-and-copy
 * collection driven by root registers. Everything is ordinary
 * `Machine<MemoryWord>` instruction data — the same pinned instruction
 * vocabulary and simulator the register machines use, with memory words as
 * the register word type.
 */
import {
  assign,
  branch,
  constant,
  gotoLabel,
  type Input,
  type MachineStatement,
  type Operation,
  op,
  perform,
  register,
  test,
} from "./01-register-machines.ts";
import { Machine, makeMachine } from "./02-simulator.ts";

/** A memory word: a typed pointer or an immediate scalar. */
export type MemoryWord =
  | { readonly tag: "pointer"; index: number }
  | { readonly tag: "scalar"; readonly value: number | string | boolean | null }
  | boolean
  | undefined;

type PointerWord = Extract<MemoryWord, { readonly tag: "pointer" }>;

const isPointer = (word: MemoryWord): word is PointerWord =>
  typeof word === "object" && word !== null && word.tag === "pointer";

/** One allocated cell: two slots, plus collector bookkeeping. */
export interface Cell {
  car: MemoryWord;
  cdr: MemoryWord;
  forwarded: boolean;
  moved: number;
}

/** The two-space heap the allocator and collector share. */
export interface Heap {
  from: Cell[];
  to: Cell[];
  free: number;
  collections: number;
}

/** Creates a heap with `capacity` cells per space. */
export const makeHeap = (capacity: number): Heap => ({
  from: Array.from(
    { length: capacity },
    (): Cell => ({ car: undefined, cdr: undefined, forwarded: false, moved: -1 }),
  ),
  to: Array.from(
    { length: capacity },
    (): Cell => ({ car: undefined, cdr: undefined, forwarded: false, moved: -1 }),
  ),
  free: 0,
  collections: 0,
});

const pointer = (index: number): MemoryWord => ({ tag: "pointer", index });
const scalar = (value: number | string | boolean | null): MemoryWord => ({ tag: "scalar", value });
const cellOf = (heap: Heap, word: MemoryWord): Cell | undefined =>
  isPointer(word) ? heap.from[word.index] : undefined;

/** Stop-and-copy over the roots; returns the number of live cells. */
export const collect = (heap: Heap, roots: MemoryWord[]): number => {
  heap.collections += 1;
  for (const slot of heap.to) {
    slot.car = undefined;
    slot.cdr = undefined;
    slot.forwarded = false;
    slot.moved = -1;
  }
  let free = 0;
  let live = 0;
  const relocate = (word: MemoryWord): MemoryWord => {
    if (!isPointer(word)) {
      return word;
    }
    const cell = heap.from[word.index];
    if (cell === undefined) {
      return undefined;
    }
    if (cell.forwarded) {
      return pointer(cell.moved);
    }
    const target = heap.to[free];
    if (target === undefined) {
      return undefined;
    }
    target.car = cell.car;
    target.cdr = cell.cdr;
    target.moved = free;
    cell.forwarded = true;
    cell.moved = free;
    free += 1;
    live += 1;
    return pointer(free - 1);
  };
  const newRoots = roots.map(relocate);
  for (let index = 0; index < roots.length; index += 1) {
    const root = roots[index];
    const moved = newRoots[index];
    if (isPointer(root) && isPointer(moved)) {
      root.index = moved.index;
    } else {
      roots[index] = moved;
    }
  }
  let scan = 0;
  while (scan < free) {
    const cell = heap.to[scan];
    if (cell !== undefined) {
      cell.car = relocate(cell.car);
      cell.cdr = relocate(cell.cdr);
    }
    scan += 1;
  }
  const swap = heap.from;
  heap.from = heap.to;
  heap.to = swap;
  for (const slot of heap.to) {
    slot.car = undefined;
    slot.cdr = undefined;
    slot.forwarded = false;
    slot.moved = -1;
  }
  heap.free = free;
  return live;
};

/** The storage operations: allocation, access, and collection triggers. */
export const storageOperations = (
  heap: Heap,
  output: string[],
): Readonly<Record<string, Operation<MemoryWord>>> => {
  const roots: MemoryWord[] = [];
  return {
    "allocate-pair": (args) => {
      if (heap.free >= heap.from.length) {
        collect(heap, roots);
      }
      if (heap.free >= heap.from.length) {
        return undefined;
      }
      const index = heap.free;
      heap.free += 1;
      const cell = heap.from[index];
      if (cell === undefined) {
        return undefined;
      }
      cell.car = args[0] ?? undefined;
      cell.cdr = args[1] ?? undefined;
      cell.forwarded = false;
      cell.moved = -1;
      return pointer(index);
    },
    "car-of": (args) => {
      const cell = cellOf(heap, args[0]);
      return cell === undefined ? undefined : cell.car;
    },
    "cdr-of": (args) => {
      const cell = cellOf(heap, args[0]);
      return cell === undefined ? undefined : cell.cdr;
    },
    "set-car": (args) => {
      const cell = cellOf(heap, args[0]);
      if (cell === undefined) {
        return undefined;
      }
      cell.car = args[1];
      return args[0];
    },
    "set-cdr": (args) => {
      const cell = cellOf(heap, args[0]);
      if (cell === undefined) {
        return undefined;
      }
      cell.cdr = args[1];
      return args[0];
    },
    "collect-garbage": (args) => {
      const live = collect(heap, roots);
      output.push("collections " + String(heap.collections));
      output.push("live " + String(live));
      return undefined;
    },
    "register-as-root": (args) => {
      if (isPointer(args[0])) {
        roots.push(args[0]);
      }
      return args[0];
    },
    "word-equal": (args) => args[0] === args[1],
    "is-pointer": (args) => isPointer(args[0]),
  };
};

/** A demonstration controller: build a list, collect, then report. */
const scalarOf = (value: number): Input<MemoryWord> => constant<MemoryWord>(scalar(value));

export const storageController: ReadonlyArray<MachineStatement<MemoryWord>> = [
  { tag: "label", name: "start" },
  assign<MemoryWord>("val", op<MemoryWord>("allocate-pair", scalarOf(42), scalarOf(7))),
  perform<MemoryWord>("register-as-root", register<MemoryWord>("val")),
  assign<MemoryWord>("temp", op<MemoryWord>("allocate-pair", scalarOf(1), scalarOf(2))),
  perform<MemoryWord>("collect-garbage"),
  assign<MemoryWord>("val", op<MemoryWord>("car-of", register<MemoryWord>("val"))),
  test<MemoryWord>("is-pointer", register<MemoryWord>("val")),
  branch<MemoryWord>("done"),
  gotoLabel<MemoryWord>("done"),
  { tag: "label", name: "done" },
];

export interface StorageRun {
  readonly outcome: { tag: "ok" | "error" };
  readonly output: ReadonlyArray<string>;
  readonly heap: Heap;
}

/** Builds and runs the storage demonstration machine. */
export const runStorageModel = (capacity = 8): StorageRun => {
  const heap = makeHeap(capacity);
  const output: string[] = [];
  const machine = makeMachine<MemoryWord>({
    registers: ["val", "temp"],
    operations: storageOperations(heap, output),
    controller: storageController,
  });
  const run = machine.run();
  return {
    outcome: run.error === null ? { tag: "ok" } : { tag: "error" },
    output,
    heap,
  };
};

export type { MachineStatement, Operation };
export { assign, branch, gotoLabel, Machine, makeMachine, op, perform, register, test };
