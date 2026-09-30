// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: sections 4.1 to 5.5

import {
  isArrayValue,
  isErrorValue,
  isMapValue,
  isNilValue,
  isPairValue,
  isPrimitive,
  isRecordValue,
  isSetValue,
  isThunkValue,
  type RecordValue,
  type Value,
} from "./runtime/value.ts";
/**
 * The shared source entry points (host-subsets grammar section 1.2): `read`
 * reads exactly one expression and rejects trailing input, `readAll` reads a
 * program as zero or more forms in source order, and `readProgram` is the
 * program entry the chapter 5 consumers re-export. All three lower accepted
 * source to the one typed AST of `syntax/ast.ts`; there is no second reader
 * and no old dynamic-language lowering anywhere. `format` renders runtime
 * values in the edition's neutral notation (never legacy list or `#t`/`()`
 * display).
 */
import type { Expr, Program } from "./syntax/ast.ts";
import type { Diagnostic, DiagnosticKind, Span } from "./syntax/diagnostics.ts";
import { parseExpression, parseProgram, ReadError } from "./syntax/parse.ts";

export type { Diagnostic, DiagnosticKind, Expr, Program, Span };
export { ReadError };

/** Reads exactly one expression from `text`; any trailing input is rejected. */
export const read = (text: string): Expr => parseExpression(text);

/** Reads every form in `text`, left to right: a whole program. */
export const readAll = (text: string): Program => parseProgram(text);

/** Reads a program; the chapter 5 entry the machine and compiler consumers re-export. */
export const readProgram = (text: string): Program => parseProgram(text);

const FORMAT_DEPTH = 8;

const formatRecord = (record: RecordValue, depth: number): string => {
  const parts: string[] = [];
  for (const [key, value] of record.fields) {
    parts.push(`${key}: ${formatInner(value, depth - 1)}`);
  }
  return `{ ${parts.join(", ")} }`;
};

const formatInner = (value: Value, depth: number): string => {
  if (depth <= 0) {
    return "...";
  }
  if (typeof value === "string") {
    return JSON.stringify(value);
  }
  if (value === null) {
    return "null";
  }
  if (value === undefined) {
    return "undefined";
  }
  if (typeof value === "number" || typeof value === "boolean") {
    return String(value);
  }
  if (isNilValue(value)) {
    return "nil";
  }
  if (isPairValue(value)) {
    return `cons(${formatInner(value.fields.get("head"), depth - 1)}, ${formatInner(value.fields.get("tail"), depth - 1)})`;
  }
  if (isArrayValue(value)) {
    return `[${value.items.map((item) => formatInner(item, depth - 1)).join(", ")}]`;
  }
  if (isRecordValue(value)) {
    return formatRecord(value, depth);
  }
  if (isMapValue(value)) {
    return `map(${[...value.entries].map(([key, item]) => `${formatInner(key, depth - 1)}: ${formatInner(item, depth - 1)}`).join(", ")})`;
  }
  if (isSetValue(value)) {
    return `set(${[...value.items].map((item) => formatInner(item, depth - 1)).join(", ")})`;
  }
  if (isErrorValue(value)) {
    return `error(${JSON.stringify(value.message)})`;
  }
  if (isPrimitive(value)) {
    return `#[primitive ${value.name}]`;
  }
  if (isThunkValue(value)) {
    return "#[thunk]";
  }
  return "#[closure]";
};

/** Renders a value in the edition's neutral display notation. */
export const format = (value: Value): string => formatInner(value, FORMAT_DEPTH);

/** Renders a read/check diagnostic with its source position. */
export const formatReadFailure = (error: ReadError): string =>
  `${error.diagnostic.kind} at ${error.diagnostic.span.line}:${error.diagnostic.span.column} (${error.diagnostic.construct}): ${error.diagnostic.message}`;
