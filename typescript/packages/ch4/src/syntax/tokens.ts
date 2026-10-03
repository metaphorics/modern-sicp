// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: sections 4.1 to 5.5

/**
 * The shared lexer. It turns source text into located tokens for the one
 * admitted grammar (host-subsets grammar section 1.1). Host-valid forms the
 * core excludes (regex literals, bigint or non-decimal numbers, suppression
 * comments) come out as `invalid` tokens carrying their construct kind, so
 * the parser reports them as `UnsupportedSyntax` with a span instead of
 * guessing. Template literals lex as `template-chunk` tokens between the
 * explicit `` ` ``, `${`, and `}` punctuators: the lexer alternates text mode
 * (inside a template) and code mode (everywhere else), and the interpolation
 * stack counts braces so an interpolation's closing `}` returns to text mode.
 */
import { type Diagnostic, diagnostic, type Span } from "./diagnostics.ts";

/** Tokens of the admitted grammar. */
export type Token =
  | { readonly kind: "ident"; readonly text: string; readonly span: Span }
  | { readonly kind: "number"; readonly text: string; readonly value: number; readonly span: Span }
  | { readonly kind: "string"; readonly text: string; readonly value: string; readonly span: Span }
  | { readonly kind: "template-chunk"; readonly value: string; readonly span: Span }
  | { readonly kind: "punct"; readonly text: string; readonly span: Span }
  | {
      readonly kind: "invalid";
      readonly construct: string;
      readonly message: string;
      readonly span: Span;
    }
  | { readonly kind: "eof"; readonly text: string; readonly span: Span };

/** The lexing result: tokens plus diagnostics found while scanning. */
export interface LexResult {
  readonly tokens: ReadonlyArray<Token>;
  readonly diagnostics: ReadonlyArray<Diagnostic>;
}

const isDigit = (c: string): boolean => c >= "0" && c <= "9";
const isIdentStart = (c: string): boolean =>
  (c >= "a" && c <= "z") ||
  (c >= "A" && c <= "Z") ||
  c === "_" ||
  c === "$" ||
  c.charCodeAt(0) > 127;
const isIdentPart = (c: string): boolean => isIdentStart(c) || isDigit(c);

/** Punctuator spellings, longest first, so scanning takes the longest match. */
const PUNCTUATORS: ReadonlyArray<string> = [
  ">>>=",
  "...",
  "===",
  "!==",
  "**=",
  "&&=",
  "||=",
  "??=",
  ">>>",
  "<<=",
  ">>=",
  "=>",
  "==",
  "!=",
  "<=",
  ">=",
  "&&",
  "||",
  "??",
  "?.",
  "++",
  "--",
  "+=",
  "-=",
  "*=",
  "/=",
  "%=",
  "**",
  "<<",
  ">>",
  "{",
  "}",
  "(",
  ")",
  "[",
  "]",
  ";",
  ",",
  ".",
  ":",
  "?",
  "=",
  "<",
  ">",
  "+",
  "-",
  "*",
  "/",
  "%",
  "!",
  "@",
  "~",
  "^",
  "|",
  "&",
  "`",
];

/** Escape sequences the ECMAScript string rules map to one character. */
const SIMPLE_ESCAPES: Readonly<Record<string, string>> = {
  n: "\n",
  r: "\r",
  t: "\t",
  b: "\b",
  f: "\f",
  v: "\v",
  "0": "\0",
};

/** Punctuators after which a `/` starts a division, not a regex. */
const REGEX_AFTER_PUNCT: Readonly<Record<string, true>> = { ")": true, "]": true, "}": true };
/** Keywords that end in expression-start position, so a following `/` would open a regex; after any other identifier it divides. */
const REGEX_AFTER_KEYWORD: Readonly<Record<string, true>> = {
  return: true,
  typeof: true,
  case: true,
  do: true,
  else: true,
  in: true,
  of: true,
  new: true,
  delete: true,
  void: true,
  throw: true,
  instanceof: true,
  yield: true,
  await: true,
};

interface Position {
  readonly offset: number;
  readonly line: number;
  readonly column: number;
}

class Lexer {
  readonly #text: string;
  readonly #tokens: Token[] = [];
  readonly #diagnostics: Diagnostic[] = [];
  #pos: Position = { offset: 0, line: 1, column: 1 };
  /** True while scanning template text (between `` ` ``/`}` and `${`/`` ` ``). */
  #textMode = false;
  /** Brace depth of each open `${` interpolation. */
  readonly #interpolations: number[] = [];

  constructor(text: string) {
    this.#text = text;
  }

  run(): LexResult {
    for (;;) {
      this.#skipTrivia();
      const start = this.#pos;
      if (start.offset >= this.#text.length) {
        this.#push({ kind: "eof", text: "", span: this.#spanFrom(start) });
        return { tokens: this.#tokens, diagnostics: this.#diagnostics };
      }
      if (this.#textMode) {
        this.#scanTemplateText(start);
        continue;
      }
      this.#scanCodeToken(start);
    }
  }

  #push(token: Token): void {
    this.#tokens.push(token);
  }

  #spanFrom(start: Position): Span {
    return { start: start.offset, end: this.#pos.offset, line: start.line, column: start.column };
  }

  #advance(count = 1): void {
    for (let i = 0; i < count; i += 1) {
      const c = this.#text[this.#pos.offset];
      if (c === undefined) {
        return;
      }
      this.#pos =
        c === "\n"
          ? { offset: this.#pos.offset + 1, line: this.#pos.line + 1, column: 1 }
          : { offset: this.#pos.offset + 1, line: this.#pos.line, column: this.#pos.column + 1 };
    }
  }

  #at(offset: number): string {
    return this.#text[offset] ?? "";
  }

  #skipTrivia(): void {
    if (this.#textMode) {
      return;
    }
    for (;;) {
      const c = this.#at(this.#pos.offset);
      if (c === " " || c === "\t" || c === "\n" || c === "\r" || c === "\f" || c === "\v") {
        this.#advance();
        continue;
      }
      if (c === "/" && this.#at(this.#pos.offset + 1) === "/") {
        this.#skipLineComment();
        continue;
      }
      if (c === "/" && this.#at(this.#pos.offset + 1) === "*") {
        this.#skipBlockComment();
        continue;
      }
      return;
    }
  }

  #skipLineComment(): void {
    const start = this.#pos;
    while (this.#pos.offset < this.#text.length && this.#at(this.#pos.offset) !== "\n") {
      this.#advance();
    }
    this.#noteSuppression(start);
  }

  #skipBlockComment(): void {
    const start = this.#pos;
    this.#advance(2);
    while (this.#pos.offset < this.#text.length) {
      if (this.#at(this.#pos.offset) === "*" && this.#at(this.#pos.offset + 1) === "/") {
        this.#advance(2);
        this.#noteSuppression(start);
        return;
      }
      this.#advance();
    }
  }

  #noteSuppression(start: Position): void {
    const body = this.#text.slice(start.offset, this.#pos.offset);
    if (body.includes("@ts-ignore") || body.includes("@ts-expect-error")) {
      this.#diagnostics.push(
        diagnostic(
          "UnsupportedSyntax",
          "suppression-comment",
          this.#spanFrom(start),
          "suppression comments are excluded",
        ),
      );
    }
  }

  // ------------------------------------------------------------------
  // Template text mode
  // ------------------------------------------------------------------

  #scanTemplateText(start: Position): void {
    const c = this.#at(start.offset);
    if (c === "`") {
      this.#advance();
      this.#textMode = false;
      this.#push({ kind: "punct", text: "`", span: this.#spanFrom(start) });
      return;
    }
    if (c === "$" && this.#at(start.offset + 1) === "{") {
      this.#advance(2);
      this.#interpolations.push(0);
      this.#textMode = false;
      this.#push({ kind: "punct", text: "${", span: this.#spanFrom(start) });
      return;
    }
    let value = "";
    while (this.#pos.offset < this.#text.length) {
      const ch = this.#at(this.#pos.offset);
      if (ch === "`" || (ch === "$" && this.#at(this.#pos.offset + 1) === "{")) {
        break;
      }
      if (ch === "\\") {
        value += this.#scanEscape();
        continue;
      }
      value += ch;
      this.#advance();
    }
    this.#push({ kind: "template-chunk", value, span: this.#spanFrom(start) });
  }

  // ------------------------------------------------------------------
  // Code mode
  // ------------------------------------------------------------------

  #scanCodeToken(start: Position): void {
    const c = this.#at(start.offset);
    if (isIdentStart(c)) {
      this.#scanIdent(start);
      return;
    }
    if (isDigit(c) || (c === "." && isDigit(this.#at(start.offset + 1)))) {
      this.#scanNumber(start);
      return;
    }
    if (c === '"') {
      this.#scanString(start);
      return;
    }
    if (c === "`") {
      this.#advance();
      this.#textMode = true;
      this.#push({ kind: "punct", text: "`", span: this.#spanFrom(start) });
      return;
    }
    if (c === "/" && this.#regexAllowed()) {
      this.#advance();
      this.#push({
        kind: "invalid",
        construct: "regex-literal",
        message: "regular-expression literals are excluded",
        span: this.#spanFrom(start),
      });
      return;
    }
    this.#scanPunct(start);
  }

  #regexAllowed(): boolean {
    const prev = this.#tokens[this.#tokens.length - 1];
    if (prev === undefined) {
      return true;
    }
    if (prev.kind === "punct") {
      return REGEX_AFTER_PUNCT[prev.text] !== true;
    }
    if (prev.kind === "number" || prev.kind === "string" || prev.kind === "template-chunk") {
      return false;
    }
    if (prev.kind === "ident") {
      return REGEX_AFTER_KEYWORD[prev.text] === true;
    }
    return true;
  }

  #scanIdent(start: Position): void {
    while (this.#pos.offset < this.#text.length && isIdentPart(this.#at(this.#pos.offset))) {
      this.#advance();
    }
    const text = this.#text.slice(start.offset, this.#pos.offset);
    const excluded = EXCLUDED_WORDS[text];
    if (excluded !== undefined) {
      this.#push({
        kind: "invalid",
        construct: excluded,
        message: `\`${text}\` is outside the admitted core`,
        span: this.#spanFrom(start),
      });
      return;
    }
    this.#push({ kind: "ident", text, span: this.#spanFrom(start) });
  }

  #scanNumber(start: Position): void {
    const c0 = this.#at(start.offset);
    const c1 = this.#at(start.offset + 1);
    if (
      c0 === "0" &&
      (c1 === "x" || c1 === "X" || c1 === "b" || c1 === "B" || c1 === "o" || c1 === "O")
    ) {
      this.#advance(2);
      while (this.#pos.offset < this.#text.length && isIdentPart(this.#at(this.#pos.offset))) {
        this.#advance();
      }
      this.#push({
        kind: "invalid",
        construct: "non-decimal-numeric-literal",
        message: "numeric literals are decimal only",
        span: this.#spanFrom(start),
      });
      return;
    }
    let seenSeparator = false;
    let seenDot = false;
    while (this.#pos.offset < this.#text.length) {
      const c = this.#at(this.#pos.offset);
      if (isDigit(c)) {
        this.#advance();
        continue;
      }
      const next = this.#at(this.#pos.offset + 1);
      const dotIsSeparator = seenDot || isIdentStart(next);
      if (c === "." && !dotIsSeparator) {
        seenDot = true;
        this.#advance();
        continue;
      }
      if (c === "_") {
        seenSeparator = true;
        this.#advance();
        continue;
      }
      const exponentAhead =
        (c === "e" || c === "E") &&
        (isDigit(next) || ("+-".includes(next) && isDigit(this.#at(this.#pos.offset + 2))));
      if (exponentAhead) {
        this.#advance();
        if ("+-".includes(this.#at(this.#pos.offset))) {
          this.#advance();
        }
        continue;
      }
      break;
    }
    const span = this.#spanFrom(start);
    if (this.#at(this.#pos.offset) === "n") {
      this.#advance();
      this.#push({
        kind: "invalid",
        construct: "bigint-literal",
        message: "bigint literals are excluded",
        span: this.#spanFrom(start),
      });
      return;
    }
    if (seenSeparator) {
      this.#push({
        kind: "invalid",
        construct: "numeric-separator",
        message: "numeric separators are excluded",
        span,
      });
      return;
    }
    const text = this.#text.slice(start.offset, this.#pos.offset);
    this.#push({ kind: "number", text, value: Number(text), span });
  }

  #scanString(start: Position): void {
    this.#advance();
    let value = "";
    for (;;) {
      if (this.#pos.offset >= this.#text.length) {
        this.#push({
          kind: "invalid",
          construct: "unterminated-string",
          message: "unterminated string literal",
          span: this.#spanFrom(start),
        });
        return;
      }
      const c = this.#at(this.#pos.offset);
      if (c === '"') {
        this.#advance();
        this.#push({
          kind: "string",
          text: this.#text.slice(start.offset, this.#pos.offset),
          value,
          span: this.#spanFrom(start),
        });
        return;
      }
      if (c === "\\") {
        value += this.#scanEscape();
        continue;
      }
      value += c;
      this.#advance();
    }
  }

  #scanEscape(): string {
    this.#advance();
    const c = this.#at(this.#pos.offset);
    const mapped = SIMPLE_ESCAPES[c];
    if (mapped !== undefined) {
      this.#advance();
      return mapped;
    }
    if (c === "x") {
      return this.#scanHexEscape(2);
    }
    if (c === "u") {
      return this.#scanUnicodeEscape();
    }
    this.#advance();
    return c;
  }

  #scanHexEscape(digits: number): string {
    this.#advance();
    let code = 0;
    for (let i = 0; i < digits; i += 1) {
      code = code * 16 + parseInt(this.#at(this.#pos.offset), 16);
      this.#advance();
    }
    return String.fromCharCode(code);
  }

  #scanUnicodeEscape(): string {
    if (this.#at(this.#pos.offset + 1) === "{") {
      const start = this.#pos;
      this.#advance(2);
      let digits = "";
      while (this.#pos.offset < this.#text.length && this.#at(this.#pos.offset) !== "}") {
        digits += this.#at(this.#pos.offset);
        this.#advance();
      }
      const closed = this.#at(this.#pos.offset) === "}";
      this.#advance();
      const code = parseInt(digits, 16);
      if (!closed || !(code >= 0 && code <= 0x10ffff)) {
        this.#diagnostics.push(
          diagnostic(
            "SyntaxError",
            "bad-unicode-escape",
            this.#spanFrom(start),
            "malformed `\\u{...}` escape",
          ),
        );
        return "";
      }
      return String.fromCodePoint(code);
    }
    return this.#scanHexEscape(4);
  }

  #scanPunct(start: Position): void {
    for (const spelling of PUNCTUATORS) {
      if (!this.#text.startsWith(spelling, start.offset)) {
        continue;
      }
      this.#advance(spelling.length);
      this.#push({ kind: "punct", text: spelling, span: this.#spanFrom(start) });
      this.#trackBraces(spelling);
      return;
    }
    this.#advance();
    this.#push({
      kind: "invalid",
      construct: "stray-character",
      message: `unexpected character \`${this.#text[start.offset] ?? ""}\``,
      span: this.#spanFrom(start),
    });
  }

  /** Counts braces inside open template interpolations. */
  #trackBraces(spelling: string): void {
    const top = this.#interpolations.length - 1;
    if (top < 0) {
      return;
    }
    if (spelling === "{") {
      this.#interpolations[top] = (this.#interpolations[top] ?? 0) + 1;
      return;
    }
    if (spelling !== "}") {
      return;
    }
    const depth = this.#interpolations[top] ?? 0;
    if (depth > 0) {
      this.#interpolations[top] = depth - 1;
      return;
    }
    this.#interpolations.pop();
    this.#textMode = true;
  }
}

/** Identifier words excluded from the core, mapped to their construct kind. */
const EXCLUDED_WORDS: Readonly<Record<string, string>> = {
  async: "async-form",
  await: "await-form",
  yield: "yield-form",
  this: "this-form",
  super: "super-form",
  var: "var-declaration",
};

/** Lexes `text` into located tokens and lexer-level diagnostics. */
export const tokenize = (text: string): LexResult => new Lexer(text).run();
