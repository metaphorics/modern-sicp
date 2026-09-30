// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { err, ok, type Result } from "../../packages/ch2/src/01-data-abstraction.js";

/**
 * Exercise 2.77: Louis Reasoner's `magnitude` surprise. The section dispatch strips
 * exactly one type tag per dispatch, so an
 * operation must be installed at every tagged level its datum carries.
 * The answer is the layered-dispatch spelling: complex-level selectors
 * that strip the outer `complex` tag and re-dispatch on the inner
 * `rectangular`/`polar` tag.
 */

/** The inner representation: Ben's or Alyssa's tagged pair. */
export type InnerRep = {
  readonly _tag: "rectangular" | "polar";
  readonly contents: readonly [number, number];
};

/** A two-level complex datum: the outer tag the system dispatches on. */
export type Complex77 = { readonly _tag: "complex"; readonly contents: InnerRep };

/** Why a one-level dispatch on a two-level datum cannot answer. */
export type Miss77 = { readonly _tag: "NoComplexSelector"; readonly op: string };

/** The magnitude of the bare pair: the representation-level entry the
 * rectangular and polar packages install. */
const repMagnitude = (z: InnerRep): number =>
  z._tag === "rectangular"
    ? Math.sqrt(z.contents[0] * z.contents[0] + z.contents[1] * z.contents[1])
    : z.contents[0];

/** The magnitude selectors by the tag they are installed under, before
 * and after exercise 2.77's fix: only the representation tags first,
 * the complex tag added with the fix. */
const magnitudeByTag: Record<string, (z: InnerRep) => number> = {
  rectangular: repMagnitude,
  polar: (z) => z.contents[0],
};

const magnitudeByTagAfterFix: Record<string, (z: InnerRep) => number> = {
  ...magnitudeByTag,
  complex: repMagnitude,
};

const lookupMagnitude = (
  byTag: Record<string, (z: InnerRep) => number>,
  z: Complex77,
): Result<number, Miss77> => {
  // The dispatch reads the OUTER tag first: the lookup key is the
  // datum's own tag, and the found selector works one tag down.
  const selector = byTag[z._tag];
  return selector === undefined
    ? err({ _tag: "NoComplexSelector", op: "magnitude" })
    : ok(selector(z.contents));
};

/** Louis's system: the representation packages installed, but no
 * complex-level selectors. The lookup at the datum's outer tag misses
 * --- the book's "No method for these types". */
export const magnitudeWithoutSelectors = (z: Complex77): Result<number, Miss77> =>
  lookupMagnitude(magnitudeByTag, z);

/** The 2.77 fix: the complex-level selectors installed under the outer
 * tag, each stripping one tag and re-dispatching on the next. */
export const magnitudeWithSelectors = (z: Complex77): Result<number, Miss77> =>
  lookupMagnitude(magnitudeByTagAfterFix, z);

/** Builds the statement's [complex, rectangular, 3, 4] at this exercise's
 * scale. */
export const makeComplex77 = (x: number, y: number): Complex77 => ({
  _tag: "complex",
  contents: { _tag: "rectangular", contents: [x, y] },
});
