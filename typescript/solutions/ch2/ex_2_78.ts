// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  type ArithContents,
  type ArithDatum,
  makeSchemeNumber,
} from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.78: the tower's base case is the bare number itself. The
 * book's `attach-tag` builds `(scheme-number n)`; after this exercise a
 * bare `bigint` names itself `scheme-number`, so the pair wrapper is
 * only for the tagged levels. The type-tag and contents procedures test
 * for the bare shape with `typeof`, the book's `number?` predicate.
 */

/** The book's attach-tag after 2.78, for the ordinary-number case the
 * exercise covers: the number installs as itself, no wrapper. */
export const attachTagSchemeNumber = (n: bigint): ArithDatum => makeSchemeNumber(n);

/** The book's type-tag after 2.78: the bare number names itself;
 * everything else reads its tag field. */
export const typeTag78 = (datum: ArithDatum): string =>
  typeof datum === "bigint" ? "scheme-number" : datum._tag;

/** The book's contents after 2.78: the bare number is its own
 * contents. */
export const contents78 = (datum: ArithDatum): ArithContents =>
  typeof datum === "bigint" ? datum : datum.contents;
