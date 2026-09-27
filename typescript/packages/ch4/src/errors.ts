// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.1

/**
 * The evaluator's checked errors. Wherever the book says "signal an error",
 * this edition never throws: failures travel through the `Effect` error
 * channel as `Schema.TaggedError` classes, so tests can assert on the exact
 * variant and the error channel is visible in every signature.
 */
import { Schema } from "effect";

/** A variable with no binding in the environment chain. */
export class UnboundVariable extends Schema.TaggedError<UnboundVariable>()("UnboundVariable", {
  name: Schema.String,
}) {}

/** Applying something that is not a procedure; `value` renders the offender. */
export class NotAProcedure extends Schema.TaggedError<NotAProcedure>()("NotAProcedure", {
  value: Schema.String,
}) {}

/** An expression outside the Scheme subset the evaluator understands. */
export class UnknownSyntax extends Schema.TaggedError<UnknownSyntax>()("UnknownSyntax", {
  expr: Schema.String,
}) {}

/** A procedure applied to the wrong number of arguments. */
export class ArityMismatch extends Schema.TaggedError<ArityMismatch>()("ArityMismatch", {
  expected: Schema.Number,
  given: Schema.Number,
}) {}

/** A failure signaled inside the evaluated program: the object-language
 * `error` primitive, a primitive applied to a bad argument, or the host
 * reader meeting text it cannot parse. */
export class RuntimeError extends Schema.TaggedError<RuntimeError>()("RuntimeError", {
  message: Schema.String,
  detail: Schema.String,
}) {}

/** Every way `evaluate` can fail. */
export type EvaluationError =
  | UnboundVariable
  | NotAProcedure
  | UnknownSyntax
  | ArityMismatch
  | RuntimeError;
