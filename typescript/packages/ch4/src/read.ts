// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.1

/**
 * Surface syntax for the evaluated language: the edition's stand-in for the
 * `read` procedure the book's driver loop takes from the underlying Lisp.
 * The host has no Scheme reader, so this module turns text into the same
 * cons-list data the evaluator interprets, and renders values back to text
 * the way the book's `display` does. A program is data: `read` of the
 * book's factorial text produces exactly the list the evaluator consumes.
 */

import type { SymbolValue, Value } from "./core.js";
import { cons, type List, nil } from "./list.js";

/** Raised when text cannot be read; the driver loop converts it to a
 * `RuntimeError` on the evaluator's error channel. */
export class ReadError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "ReadError";
  }
}

const symbolOf = (name: string): SymbolValue => ({ _tag: "Symbol", name });

const isDelimiter = (c: string): boolean =>
  c === " " ||
  c === "\t" ||
  c === "\n" ||
  c === "\r" ||
  c === "(" ||
  c === ")" ||
  c === '"' ||
  c === ";" ||
  c === "'";

/** Tokenizes into parens, quotes, string literals, and atoms. */
function* tokenize(text: string): Generator<string> {
  const chars = [...text];
  const charAt = (k: number): string => chars[k] ?? "";
  let i = 0;
  while (i < chars.length) {
    const c = charAt(i);
    if (c === ";") {
      while (i < chars.length && charAt(i) !== "\n") {
        i += 1;
      }
    } else if (c === " " || c === "\t" || c === "\n" || c === "\r") {
      i += 1;
    } else if (c === "(" || c === ")" || c === "'") {
      yield c;
      i += 1;
    } else if (c === '"') {
      let j = i + 1;
      let literal = "";
      while (j < chars.length && charAt(j) !== '"') {
        if (charAt(j) === "\\" && j + 1 < chars.length) {
          literal += charAt(j + 1) === "n" ? "\n" : charAt(j + 1);
          j += 2;
        } else {
          literal += charAt(j);
          j += 1;
        }
      }
      if (j >= chars.length) {
        throw new ReadError("unterminated string literal");
      }
      yield `"${literal}"`;
      i = j + 1;
    } else {
      let j = i;
      while (j < chars.length && !isDelimiter(charAt(j))) {
        j += 1;
      }
      yield text.slice(i, j);
      i = j;
    }
  }
}

const atomToValue = (atom: string): Value => {
  if (atom === "#t") {
    return { _tag: "Boolean", b: true };
  }
  if (atom === "#f") {
    return { _tag: "Boolean", b: false };
  }
  if (/^[+-]?(\d+\.?\d*|\.\d+)$/.test(atom)) {
    return { _tag: "Number", n: Number(atom) };
  }
  if (atom.startsWith('"')) {
    return { _tag: "String", s: atom.slice(1, -1) };
  }
  return symbolOf(atom);
};

class TokenStream {
  readonly #tokens: string[];
  #pos = 0;

  constructor(text: string) {
    this.#tokens = [...tokenize(text)];
  }

  peek(): string | undefined {
    return this.#tokens[this.#pos];
  }

  next(): string | undefined {
    const token = this.#tokens[this.#pos];
    this.#pos += 1;
    return token;
  }

  atEnd(): boolean {
    return this.#pos >= this.#tokens.length;
  }
}

/** Reads one datum; `.` in tail position builds an improper pair. */
const readDatum = (stream: TokenStream): Value => {
  const token = stream.next();
  if (token === undefined) {
    throw new ReadError("unexpected end of input");
  }
  if (token === ")") {
    throw new ReadError("unexpected )");
  }
  if (token === "'") {
    return cons(symbolOf("quote"), cons(readDatum(stream), nil));
  }
  if (token === "(") {
    return readListTail(stream);
  }
  return atomToValue(token);
};

const readListTail = (stream: TokenStream): List<Value> => {
  const token = stream.peek();
  if (token === undefined) {
    throw new ReadError("unexpected end of input inside list");
  }
  if (token === ")") {
    stream.next();
    return nil;
  }
  if (token === ".") {
    stream.next();
    const tail = readDatum(stream);
    if (stream.next() !== ")") {
      throw new ReadError("expected ) after dotted tail");
    }
    if (tail._tag !== "Cons" && tail._tag !== "Nil") {
      throw new ReadError("dotted tail must be a list");
    }
    return tail;
  }
  return cons(readDatum(stream), readListTail(stream));
};

const startsForm = (token: string | undefined): boolean => token !== undefined && token !== ")";

/** Reads exactly one form from `text`; any trailing token is an error. */
export const read = (text: string): Value => {
  const stream = new TokenStream(text);
  const datum = readDatum(stream);
  if (!stream.atEnd()) {
    throw new ReadError("trailing input after the form");
  }
  return datum;
};

/** Reads every form in `text`, left to right: a whole program as data. */
export const readAll = (text: string): ReadonlyArray<Value> => {
  const stream = new TokenStream(text);
  const forms: Value[] = [];
  while (startsForm(stream.peek())) {
    forms.push(readDatum(stream));
  }
  if (!stream.atEnd()) {
    throw new ReadError("unexpected ) outside any list");
  }
  return forms;
};

const formatList = (items: List<Value>): string => {
  const parts: string[] = [];
  let rest: List<Value> = items;
  while (rest._tag === "Cons") {
    parts.push(format(rest.head));
    rest = rest.tail;
  }
  if (rest._tag === "Nil") {
    return `(${parts.join(" ")})`;
  }
  return `(${parts.join(" ")} . ${format(rest)})`;
};

/** Renders a value the way the book's `display` prints it: lists in
 * parentheses, `#t` and `#f` for booleans, strings without quotes. */
export const format = (value: Value): string => {
  switch (value._tag) {
    case "Number":
      return String(value.n);
    case "Boolean":
      return value.b ? "#t" : "#f";
    case "Symbol":
      return value.name;
    case "String":
      return value.s;
    case "Unspecified":
      return "#<unspecified>";
    case "Nil":
      return "()";
    case "Cons":
      return formatList(value);
    case "Primitive":
      return `#[primitive ${value.name}]`;
    case "Compound":
      return "#[compound-procedure]";
    case "Execution":
      return "#[execution-procedure]";
  }
};
