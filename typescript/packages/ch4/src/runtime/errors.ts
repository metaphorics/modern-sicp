// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: sections 4.1 to 5.5

/**
 * Checked evaluator results and completions (host-subsets grammar section 8).
 * Wherever the book says "signal an error", this edition returns the declared
 * typed variant on a result union: `Outcome` is the guest-visible result of
 * evaluation, `Completion` is the internal statement-level control state.
 * A guest `throw` that no `catch` handles surfaces as `guest-throw`, never as
 * a host exception and never as a plausible guest value.
 */
import type { Value } from "./value.ts";

/** Every way guest evaluation can fail. */
export type GuestError =
  | { readonly tag: "unbound-name"; readonly name: string }
  | { readonly tag: "tdz-access"; readonly name: string }
  | { readonly tag: "not-callable"; readonly detail: string }
  | { readonly tag: "wrong-arity"; readonly expected: number; readonly given: number }
  | { readonly tag: "bad-operand"; readonly operator: string; readonly detail: string }
  | { readonly tag: "unknown-syntax"; readonly construct: string }
  | { readonly tag: "unknown-field"; readonly field: string }
  | { readonly tag: "unresolved-import"; readonly module: string; readonly name: string }
  | { readonly tag: "readonly-field"; readonly field: string }
  | { readonly tag: "guest-throw"; readonly value: Value };

/** The result of evaluating a guest program or expression. */
export type Outcome =
  | { readonly tag: "ok"; readonly value: Value }
  | { readonly tag: "error"; readonly error: GuestError };

/** The internal statement-level completion state. */
export type Completion =
  | { readonly tag: "normal"; readonly value: Value }
  | { readonly tag: "return"; readonly value: Value }
  | { readonly tag: "break" }
  | { readonly tag: "continue" }
  | { readonly tag: "throw"; readonly value: Value }
  | { readonly tag: "error"; readonly error: GuestError };

/** A successful outcome. */
export const ok = (value: Value): Outcome => ({ tag: "ok", value });

/** A failed outcome. */
export const fail = (error: GuestError): Outcome => ({ tag: "error", error });

/** A normally completed statement. */
export const normal = (value: Value): Completion => ({ tag: "normal", value });

/** An errored statement. */
export const failed = (error: GuestError): Completion => ({ tag: "error", error });

/** Maps a statement completion onto the evaluator outcome. */
export const outcomeOf = (completion: Completion): Outcome =>
  completion.tag === "normal"
    ? ok(completion.value)
    : completion.tag === "return"
      ? ok(completion.value)
      : completion.tag === "throw"
        ? fail({ tag: "guest-throw", value: completion.value })
        : completion.tag === "error"
          ? fail(completion.error)
          : fail({ tag: "unknown-syntax", construct: `unexpected-${completion.tag}` });
