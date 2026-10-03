// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { car, getOrElse } from "../../packages/ch2/src/02-picture-language.js";
import {
  type Datum,
  listDatum,
  numDatum,
  showDatum,
  symDatum,
} from "../../packages/ch2/src/03-symbolic-data.js";

/**
 * Exercise 2.55: the doubled quotation mark, built by hand. Applying the
 * quotation operation to the symbol `abracadabra` twice in a row
 * abbreviates a quotation of a quotation; evaluating it once leaves the
 * two-element list whose first item is the symbol `quote` and whose
 * second item is `abracadabra`. The outer `listDatum` below plays the
 * outer quotation mark, and the `symDatum("quote")` inside it is what the
 * inner mark left behind.
 */
export const valueOfDoubleQuote: Datum = listDatum(symDatum("quote"), symDatum("abracadabra"));

/** Head of the hand-built double quotation: the symbol `quote`. */
export const carOfDoubleQuote = (): string => {
  if (valueOfDoubleQuote._tag !== "Lst") {
    throw new Error("the double-quoted value must be a list");
  }
  return showDatum(getOrElse(car(valueOfDoubleQuote.items), numDatum(0)));
};
