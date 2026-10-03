// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { List } from "../../packages/ch2/src/02-picture-language.js";
import type { Vect2 } from "./ex_2_46.js";

/** Exercise 2.47: two frame representations, each with its selectors. */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 2.47 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The list representation: the frame is the list (origin edge1 edge2). */
export function makeFrameList(_origin: Vect2, _e1: Vect2, _e2: Vect2): List<Vect2> {
  throw new PendingSolution();
}

/** The origin selector for the list representation. */
export function originFrameList(_frame: List<Vect2>): Vect2 {
  throw new PendingSolution();
}

/** The first edge selector for the list representation. */
export function edge1FrameList(_frame: List<Vect2>): Vect2 {
  throw new PendingSolution();
}

/** The list representation's second-edge selector. */
export function edge2FrameList(_frame: List<Vect2>): Vect2 {
  throw new PendingSolution();
}

/** The pair representation's frame. */
export type FramePair = readonly [Vect2, readonly [Vect2, Vect2]];

/** The pair representation: the frame is (origin (edge1 edge2)). */
export function makeFramePair(_origin: Vect2, _e1: Vect2, _e2: Vect2): FramePair {
  throw new PendingSolution();
}

/** The origin selector for the pair representation. */
export function originFramePair(_frame: FramePair): Vect2 {
  throw new PendingSolution();
}

/** The first edge selector for the pair representation. */
export function edge1FramePair(_frame: FramePair): Vect2 {
  throw new PendingSolution();
}

/** The pair representation's second-edge selector. */
export function edge2FramePair(_frame: FramePair): Vect2 {
  throw new PendingSolution();
}
