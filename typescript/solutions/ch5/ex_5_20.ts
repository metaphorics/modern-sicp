// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  car,
  cdr,
  cons,
  dump,
  emptyList,
  eqWords,
  type Memory,
  makeMemory,
  pairPointer,
  renderWord,
  type Value,
  write,
} from "./exercise-memory.ts";

/** The exercise's planted memory: the two definitions' structures and
 * their pointers. */
export interface ExerciseMemory {
  readonly memory: Memory;
  readonly x: Value;
  readonly y: Value;
}

/** The exercise's two definitions run through the allocation path: the
 * three conses against a memory whose free pointer starts at p1. A
 * register machine cannot fill the outer cell of `(list x x)` before it
 * has computed the inner one, so the cells come out at p1, p2, p3: x is
 * p1, the inner cell (x x)'s tail sits at p2, y is p3, and free ends at
 * p4. Both elements of y name the same cell p1, the sharing the
 * box-and-pointer drawing shows as two arrows into one box. */
export const exerciseMemory = (): ExerciseMemory => {
  const memory = makeMemory(8, 1);
  const x = cons(memory, 1, 2);
  const inner = cons(memory, x, emptyList);
  const y = cons(memory, x, inner);
  return { memory, x, y };
};

/** The exercise's answer: the the-cars/the-cdrs table cell for cell,
 * then the two pointer values and the final free pointer. */
export const memoryVectorDrawing = (): string[] => {
  const { memory, x, y } = exerciseMemory();
  return [
    ...dump(memory).split("\n"),
    `x = ${renderWord(x)}`,
    `y = ${renderWord(y)}`,
    `free = ${renderWord(pairPointer(memory.free))}`,
  ];
};

/** The sharing the drawing shows: both elements of y are the pointer p1
 * naming the same cell x's cons allocated, so each is `eq?` to x. */
export const yElementsShareX = (): boolean => {
  const { memory, x, y } = exerciseMemory();
  return (
    eqWords(car(memory, y), x) &&
    eqWords(car(memory, cdr(memory, y)), x) &&
    renderWord(car(memory, y)) === "p1"
  );
};

/** The box-and-pointer reading of the two structures, drawn through the
 * memory: x is (1 2) and y is ((1 2) (1 2)) with one shared copy. */
export const structureReadBack = (): string[] => {
  const { memory, x, y } = exerciseMemory();
  return [write(memory, x), write(memory, y)];
};
