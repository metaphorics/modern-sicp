// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { type List, list } from "../../packages/ch2/src/02-picture-language.js";
import type { Vect2 } from "./ex_2_46.js";

/**
 * Exercise 2.47: exercise 2.46's vectors supply the frame
 * constructor's arguments. The book proposes two representations, each
 * with its own selectors: a frame is the three-element list
 * `(origin edge1 edge2)`, or the pair `(origin (edge1 edge2))`. A
 * painter only ever calls selectors, so the two representations are
 * interchangeable below them - which the data abstraction guarantees.
 */
export type FramePair = readonly [Vect2, readonly [Vect2, Vect2]];

/** The list representation: the frame is the list (origin edge1 edge2). */
export const makeFrameList = (origin: Vect2, e1: Vect2, e2: Vect2): List<Vect2> =>
  list(origin, e1, e2);

/** The frame vector at `index` - 0 reads the origin, 1 the first edge,
 * 2 the second - by cdr-ing down the list the book's selectors walk.
 * A short frame reads as the zero vector; `makeFrameList` always
 * builds all three. */
const frameVectorAt = (frame: List<Vect2>, index: number): Vect2 => {
  let rest = frame;
  for (let i = 0; i < index; i += 1) {
    rest = rest._tag === "Cons" ? rest.tail : rest;
  }
  return rest._tag === "Cons" ? rest.head : [0, 0];
};

/** The origin selector for the list representation. */
export const originFrameList = (frame: List<Vect2>): Vect2 => frameVectorAt(frame, 0);

/** The first edge selector for the list representation. */
export const edge1FrameList = (frame: List<Vect2>): Vect2 => frameVectorAt(frame, 1);

/** The second edge selector for the list representation. */
export const edge2FrameList = (frame: List<Vect2>): Vect2 => frameVectorAt(frame, 2);

/** The pair representation: the frame is (origin (edge1 edge2)). */
export const makeFramePair = (origin: Vect2, e1: Vect2, e2: Vect2): FramePair => [origin, [e1, e2]];

/** The origin selector for the pair representation. */
export const originFramePair = (frame: FramePair): Vect2 => frame[0];

/** The first edge selector for the pair representation. */
export const edge1FramePair = (frame: FramePair): Vect2 => frame[1][0];

/** The second edge selector for the pair representation. */
export const edge2FramePair = (frame: FramePair): Vect2 => frame[1][1];
