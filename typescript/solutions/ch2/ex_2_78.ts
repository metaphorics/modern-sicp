// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  type ArithContents,
  type ArithDatum,
  makeTsNumber,
} from "../../packages/ch2/src/05-generic-operations.js";

/**
 * Exercise 2.78: the tower's base case is the bare number itself. The
 * statement's attach-tag operation builds `[ts-number, n]`; after this
 * exercise a
 * bare `bigint` names itself `ts-number`, so the pair wrapper is
 * only for the tagged levels. The `typeTag78` and `contents78` procedures test
 * for the bare shape with `typeof`, which identifies the `bigint`
 * exact-integer values.
 */

/** The statement's `attachTagTsNumber` after 2.78, for the ordinary-number case the
 * exercise covers: the number installs as itself, no wrapper. */
export const attachTagTsNumber = (n: bigint): ArithDatum => makeTsNumber(n);

/** The statement's `typeTag78` after 2.78: the bare number names itself;
 * everything else reads its tag field. */
export const typeTag78 = (datum: ArithDatum): string =>
  typeof datum === "bigint" ? "ts-number" : datum._tag;

/** The statement's `contents78` after 2.78: the bare number is its own
 * contents. */
export const contents78 = (datum: ArithDatum): ArithContents =>
  typeof datum === "bigint" ? datum : datum.contents;
