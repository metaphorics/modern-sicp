// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  car,
  cdr,
  getOrElse,
  type List,
  list,
  nil,
  type Option,
} from "../../packages/ch2/src/02-picture-language.js";
import {
  type Datum,
  listDatum,
  memq,
  numDatum,
  showDataList,
  showDatum,
  symDatum,
} from "../../packages/ch2/src/03-symbolic-data.js";

/**
 * Exercise 2.53: the statement's seven printed-output predictions,
 * restated over this edition's constructors. Each predictor returns the
 * printed value this edition's renderers produce for one of the seven
 * shapes; the answers are pinned by the colocated test.
 */

const xy = list(
  listDatum(symDatum("x1"), symDatum("x2")),
  listDatum(symDatum("y1"), symDatum("y2")),
);

/** The flat list `[a, b, c]`. */
export const predictList = (): string =>
  showDatum(listDatum(symDatum("a"), symDatum("b"), symDatum("c")));

/** The nested list `[[george]]`. */
export const predictNested = (): string => showDatum(listDatum(listDatum(symDatum("george"))));

/** Tail of `[[x1, x2], [y1, y2]]`. */
export const predictCdr = (): string => showDataList(getOrElse(cdr(xy), nil));

/** Head of the tail of `[[x1, x2], [y1, y2]]`. */
export const predictCadr = (): string =>
  showDatum(getOrElse(car(getOrElse(cdr(xy), nil)), numDatum(0)));

/** Is the head of `[a, short, list]` itself a list? */
export const predictHeadIsList = (headOfA: Datum): boolean => headOfA._tag === "Lst";

/** Membership of `red` in `[[red, shoes], [blue, socks]]`. */
export const predictMemqSublists = (): Option<List<Datum>> =>
  memq(
    symDatum("red"),
    list(
      listDatum(symDatum("red"), symDatum("shoes")),
      listDatum(symDatum("blue"), symDatum("socks")),
    ),
  );

/** Membership of `red` in `[red, shoes, blue, socks]`. */
export const predictMemqFlat = (): Option<List<Datum>> =>
  memq(
    symDatum("red"),
    list(symDatum("red"), symDatum("shoes"), symDatum("blue"), symDatum("socks")),
  );
