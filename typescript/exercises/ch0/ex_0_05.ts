// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 0.5: rewrite the throwing `parseAmountEager` as `parseAmount`,
 * which returns the same outcomes as `Result` values instead of throwing.
 * The statement lives in the section 0.6 chapter text.
 */
export class PendingSolution extends Error {
  constructor() {
    super("exercise 0.5 is not solved yet");
    this.name = "PendingSolution";
  }
}

/** The two-variant error set of the exercise. */
export type ParseError =
  | { readonly _tag: "Blank" }
  | { readonly _tag: "NotDigits"; readonly input: string };

/** The section 0.6 result: a value or an error, never a throw. */
export type Result<A, E> =
  | { readonly _tag: "Ok"; readonly value: A }
  | { readonly _tag: "Error"; readonly error: E };

/** The eager original: throws where your version returns a value. */
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

export function parseAmount(_s: string): Result<number, ParseError> {
  throw new PendingSolution();
}
