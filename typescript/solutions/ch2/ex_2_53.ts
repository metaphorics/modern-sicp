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
  memq,
  qlist,
  qnum,
  qsym,
  showDataList,
  showDatum,
} from "../../packages/ch2/src/03-symbolic-data.js";

/**
 * Exercise 2.53: the book's seven printed-output predictions, restated
 * over this edition's constructors. Each predictor returns the printed
 * value this edition's renderers produce for one of the book's
 * expressions; the answers are pinned by the colocated test.
 */

const xy = list(qlist(qsym("x1"), qsym("x2")), qlist(qsym("y1"), qsym("y2")));

/** The book's `(list 'a 'b 'c)`. */
export const predictList = (): string => showDatum(qlist(qsym("a"), qsym("b"), qsym("c")));

/** The book's `(list (list 'george))`. */
export const predictNested = (): string => showDatum(qlist(qlist(qsym("george"))));

/** The book's `(cdr '((x1 x2) (y1 y2)))`. */
export const predictCdr = (): string => showDataList(getOrElse(cdr(xy), nil));

/** The book's `(cadr '((x1 x2) (y1 y2)))`. */
export const predictCadr = (): string =>
  showDatum(getOrElse(car(getOrElse(cdr(xy), nil)), qnum(0)));

/** The book's `(pair? (car '(a short list)))`: is the head a list? */
export const predictHeadIsList = (headOfA: Datum): boolean => headOfA._tag === "Lst";

/** The book's `(memq 'red '((red shoes) (blue socks)))`. */
export const predictMemqSublists = (): Option<List<Datum>> =>
  memq(qsym("red"), list(qlist(qsym("red"), qsym("shoes")), qlist(qsym("blue"), qsym("socks"))));

/** The book's `(memq 'red '(red shoes blue socks))`. */
export const predictMemqFlat = (): Option<List<Datum>> =>
  memq(qsym("red"), list(qsym("red"), qsym("shoes"), qsym("blue"), qsym("socks")));
