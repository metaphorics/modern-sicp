// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 0.5: `parseAmount` returns the eager parser's outcomes as
 * `Result` values.
 *
 * The guards mirror `parseAmountEager` in order: blank after trimming is
 * `Blank`, a character outside `0..9` is `NotDigits` carrying the original
 * input, and the digits are their numeric value.
 */
export type ParseError =
  | { readonly _tag: "Blank" }
  | { readonly _tag: "NotDigits"; readonly input: string };

export type Result<A, E> =
  | { readonly _tag: "Ok"; readonly value: A }
  | { readonly _tag: "Error"; readonly error: E };

/** The eager original, kept beside the solution for the contrast test. */
export const parseAmountEager = (s: string): number => {
  const t = s.trim();
  if (t.length === 0) {
    throw new RangeError("blank amount");
  }
  if (!/^[0-9]+$/.test(t)) {
    throw new TypeError(`not digits: ${s}`);
  }
  return Number(t);
};

export function parseAmount(s: string): Result<number, ParseError> {
  const t = s.trim();
  if (t.length === 0) {
    return { _tag: "Error", error: { _tag: "Blank" } };
  }
  if (!/^[0-9]+$/.test(t)) {
    return { _tag: "Error", error: { _tag: "NotDigits", input: s } };
  }
  return { _tag: "Ok", value: Number(t) };
}
