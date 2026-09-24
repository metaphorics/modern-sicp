// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: Chapter 0 section 0.6

/** The book's two-sided result: a value or an error, never a throw. */
export type Result<A, E> =
  | { readonly _tag: "Ok"; readonly value: A }
  | { readonly _tag: "Error"; readonly error: E };

/** Wraps a success value. */
export const ok = <A>(value: A): Result<A, never> => ({ _tag: "Ok", value });

/** Wraps a failure value. */
export const err = <E>(error: E): Result<never, E> => ({ _tag: "Error", error });

/** The parse errors of the section's example: one variant per failure mode. */
export type PortError =
  | { readonly _tag: "Blank" }
  | { readonly _tag: "NotDigits"; readonly input: string }
  | { readonly _tag: "OutOfRange"; readonly n: number };

const isDigits = (s: string): boolean => /^[0-9]+$/.test(s);

/** Parses a port number in 1 through 65535, reporting every failure as a value. */
export const parsePort = (s: string): Result<number, PortError> => {
  const t = s.trim();
  if (t.length === 0) {
    return err({ _tag: "Blank" });
  }
  if (!isDigits(t)) {
    return err({ _tag: "NotDigits", input: s });
  }
  const n = Number(t);
  if (n < 1 || n > 65535) {
    return err({ _tag: "OutOfRange", n });
  }
  return ok(n);
};

/** Renders the error union; the switch must cover every variant. */
export const renderError = (e: PortError): string => {
  switch (e._tag) {
    case "Blank":
      return "blank input";
    case "NotDigits":
      return `not digits: ${e.input}`;
    case "OutOfRange":
      return `out of range: ${e.n}`;
  }
};
