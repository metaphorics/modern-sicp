// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { car, getOrElse } from "../../packages/ch2/src/02-picture-language.js";
import {
  type Datum,
  qlist,
  qnum,
  qsym,
  showDatum,
} from "../../packages/ch2/src/03-symbolic-data.js";

/**
 * Exercise 2.55: the book's `(car ''abracadabra)`, built by hand. The
 * reader expands the typed expression to `(quote (quote abracadabra))`;
 * evaluating that wraps the inner list once more, so the value is the
 * two-element list whose first item is the symbol `quote`. The outer
 * `qlist` below plays the outer quote mark of `''abracadabra`, and the
 * `qsym("quote")` inside it is what the inner quote mark left behind.
 */
export const valueOfDoubleQuote: Datum = qlist(qsym("quote"), qsym("abracadabra"));

/** The book's `(car ''abracadabra)`: the symbol `quote`. */
export const carOfDoubleQuote = (): string => {
  if (valueOfDoubleQuote._tag !== "Lst") {
    throw new Error("the double-quoted value must be a list");
  }
  return showDatum(getOrElse(car(valueOfDoubleQuote.items), qnum(0)));
};
