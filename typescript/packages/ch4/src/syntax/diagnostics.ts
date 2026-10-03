// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: sections 4.1 to 5.5

/**
 * Diagnostics for the shared checked syntax. Failure layers stay distinct by
 * category, never by compiler prose (host-subsets grammar section 8):
 * `UnsupportedSyntax` reports host-valid but excluded constructs with a span
 * and a construct kind; subset rules report `ExpectedBoolean`,
 * `ExpectedNumber`, and friends; malformed text reports `SyntaxError`; a
 * single-expression read with input left over reports `TrailingInput`.
 */

/** A half-open source range with the 1-based line/column of its start. */
export interface Span {
  readonly start: number;
  readonly end: number;
  readonly line: number;
  readonly column: number;
}

/** The stable diagnostic categories of the subset contract. */
export type DiagnosticKind =
  | "SyntaxError"
  | "TrailingInput"
  | "UnsupportedSyntax"
  | "UnsupportedType"
  | "ExpectedBoolean"
  | "ExpectedNumber"
  | "ForbiddenHostPrimitive"
  | "UseBeforeInitialization"
  | "ReassignConst"
  | "ReadOnlyField"
  | "DuplicateDeclaration";

/** One diagnostic: category, offending construct, span, and message text. */
export interface Diagnostic {
  readonly kind: DiagnosticKind;
  readonly construct: string;
  readonly span: Span;
  readonly message: string;
}

/** Builds a diagnostic from its parts. */
export const diagnostic = (
  kind: DiagnosticKind,
  construct: string,
  span: Span,
  message: string,
): Diagnostic => ({ kind, construct, span, message });

/** Renders a diagnostic for transcripts and driver output. */
export const formatDiagnostic = (d: Diagnostic): string =>
  `${d.kind} at ${d.span.line}:${d.span.column} (${d.construct}): ${d.message}`;

/** A span from `start`'s position through `end`'s position. */
export const spanOf = (start: Span, end: Span): Span => ({
  start: start.start,
  end: end.end,
  line: start.line,
  column: start.column,
});

/** The empty span, for synthesized nodes with no source text. */
export const noSpan: Span = { start: 0, end: 0, line: 1, column: 1 };
