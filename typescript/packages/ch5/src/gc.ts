// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 5.3

import type { Value } from "@sicp-ts/ch4/core";
/**
 * The vector memory of section 5.3, as types. The host is garbage-collected,
 * so the section rebuilds by hand what V8 does for the reader: a flat vector
 * of cells, a free pointer, and the stop-and-copy collector. Chapter 5's
 * explicit-control evaluator and compiler exercises run on this memory, and
 * its "infinite memory illusion" is restated against the real collector.
 * Allocation and collection are section 5.3's lesson; the spine owns only
 * the memory vocabulary.
 */
import type { Effect, Ref } from "effect";

/** One memory word: a value or an empty cell. */
export type Cell = Value | undefined;

/**
 * The vector memory: `cells` holds both semispaces, `free` is the bump
 * pointer into the active space, and `scan` walks ahead of `free` while a
 * stop-and-copy collection relocates live cells.
 */
export interface VectorMemory {
  readonly cells: Ref.Ref<Array<Cell>>;
  readonly free: Ref.Ref<number>;
  readonly scan: Ref.Ref<number>;
}

/** A machine cell the collector treats as a root: a register holds one value. */
export type Root = Ref.Ref<Value>;

/**
 * Collects garbage by stop-and-copy over the semispaces, scanning `roots`;
 * the returned number is the new free pointer of the surviving space.
 */
export type Collect = (memory: VectorMemory, roots: ReadonlyArray<Root>) => Effect.Effect<number>;
