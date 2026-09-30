// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: sections 4.1 to 5.5

/**
 * The shared recursive-descent parser for the one admitted grammar
 * (host-subsets grammar section 1.1). It lowers accepted source to the shared
 * typed AST of `ast.ts`, rejects host-valid but excluded constructs as
 * `UnsupportedSyntax` with a span and a construct kind (never lowering them
 * into a supported node), and is the single parser behind `read`, `readAll`,
 * and `readProgram`. The named experiment modes admit their own extension
 * nodes; the core rejects those names.
 */
import type {
  Arg,
  Block,
  CaseClause,
  Decl,
  Expr,
  FieldNode,
  ImportName,
  ObjectField,
  Param,
  PrimitiveTypeName,
  Program,
  Stmt,
  TypeNode,
  TypeParamNode,
} from "./ast.ts";
import { type Diagnostic, diagnostic, type Span, spanOf } from "./diagnostics.ts";
import { type Token, tokenize } from "./tokens.ts";

/** Thrown when text cannot be read; carries the first diagnostic. */
export class ReadError extends Error {
  readonly diagnostic: Diagnostic;

  constructor(d: Diagnostic) {
    super(`${d.kind} (${d.construct}): ${d.message}`);
    this.name = "ReadError";
    this.diagnostic = d;
  }
}

/** The named execution modes of grammar section 6. */
export type ExperimentMode =
  | "core"
  | "lazy-memoized-experiment"
  | "lazy-recompute-experiment"
  | "amb-depth-first-experiment"
  | "amb-ramb-experiment";

/** Which named experiments admit each reserved extension name. */
const RESERVED_NAMES: Readonly<Record<string, ReadonlyArray<ExperimentMode>>> = {
  delay: ["lazy-memoized-experiment", "lazy-recompute-experiment"],
  force: ["lazy-memoized-experiment", "lazy-recompute-experiment"],
  choose: ["amb-depth-first-experiment", "amb-ramb-experiment"],
  require: ["amb-depth-first-experiment", "amb-ramb-experiment"],
  ramb: ["amb-ramb-experiment"],
  permanentAssign: ["amb-depth-first-experiment", "amb-ramb-experiment"],
  ifFail: ["amb-depth-first-experiment", "amb-ramb-experiment"],
};

const PRIMITIVE_TYPES: Readonly<Record<string, PrimitiveTypeName>> = {
  number: "number",
  string: "string",
  boolean: "boolean",
  null: "null",
  undefined: "undefined",
  void: "void",
  unknown: "unknown",
};

/** Binary and logical operator spellings mapped to their exact op names. */
type BinaryOp = "+" | "-" | "*" | "/" | "%" | "<" | "<=" | ">" | ">=" | "===" | "!==";
type LogicalOp = "&&" | "||";
const OPERATOR_TEXTS: Readonly<Record<string, BinaryOp | LogicalOp>> = {
  "||": "||",
  "&&": "&&",
  "===": "===",
  "!==": "!==",
  "<": "<",
  "<=": "<=",
  ">": ">",
  ">=": ">=",
  "+": "+",
  "-": "-",
  "*": "*",
  "/": "/",
  "%": "%",
};

/** Prefix operator spellings mapped to their exact op names. */
const PREFIX_OPERATORS: Readonly<Record<string, "!" | "+" | "-">> = {
  "!": "!",
  "+": "+",
  "-": "-",
};

/** Excluded operator spellings mapped to their construct kind. */
const EXCLUDED_OPERATORS: Readonly<Record<string, string>> = {
  "==": "loose-equality",
  "!=": "loose-equality",
  "++": "increment-or-decrement",
  "--": "increment-or-decrement",
  "+=": "compound-assignment",
  "-=": "compound-assignment",
  "*=": "compound-assignment",
  "/=": "compound-assignment",
  "%=": "compound-assignment",
  "**=": "compound-assignment",
  "&&=": "compound-assignment",
  "||=": "compound-assignment",
  "??=": "compound-assignment",
  "**": "exponent-operator",
  "<<": "bitwise-operator",
  ">>": "bitwise-operator",
  ">>>": "bitwise-operator",
  "&": "bitwise-operator",
  "|": "bitwise-operator",
  "^": "bitwise-operator",
  "~": "bitwise-operator",
};

const EXCLUDED_KEYWORD_OPERATORS: Readonly<Record<string, string>> = {
  delete: "delete-operator",
  in: "in-operator",
  instanceof: "instanceof-operator",
};

class Parser {
  readonly #tokens: ReadonlyArray<Token>;
  readonly #mode: ExperimentMode;
  #index = 0;

  constructor(text: string, mode: ExperimentMode) {
    const lexed = tokenize(text);
    const first = lexed.diagnostics[0];
    if (first !== undefined) {
      throw new ReadError(first);
    }
    this.#tokens = lexed.tokens;
    this.#mode = mode;
    this.#rejectInvalidTokens();
  }

  // ------------------------------------------------------------------
  // Token cursor
  // ------------------------------------------------------------------

  #peek(offset = 0): Token {
    return (
      this.#tokens[Math.min(this.#index + offset, this.#tokens.length - 1)] ??
      this.#tokens[this.#tokens.length - 1] ?? {
        kind: "eof",
        text: "",
        span: { start: 0, end: 0, line: 1, column: 1 },
      }
    );
  }

  #next(): Token {
    const token = this.#peek();
    if (token.kind !== "eof") {
      this.#index += 1;
    }
    return token;
  }

  #isText(text: string, offset = 0): boolean {
    const token = this.#peek(offset);
    return (token.kind === "punct" || token.kind === "ident") && token.text === text;
  }

  #eat(text: string): boolean {
    if (!this.#isText(text)) {
      return false;
    }
    this.#index += 1;
    return true;
  }

  #expect(text: string, what: string): Token {
    if (!this.#isText(text)) {
      return this.#fail(
        "SyntaxError",
        `expected-${text}`,
        this.#peek().span,
        `expected \`${text}\` ${what}`,
      );
    }
    return this.#next();
  }

  #fail(kind: Diagnostic["kind"], construct: string, span: Span, message: string): never {
    throw new ReadError(diagnostic(kind, construct, span, message));
  }

  #unsupported(construct: string, span: Span, message: string): never {
    this.#fail("UnsupportedSyntax", construct, span, message);
  }

  // ------------------------------------------------------------------
  // Excluded-construct recognition
  // ------------------------------------------------------------------

  #rejectInvalidTokens(): void {
    for (const token of this.#tokens) {
      if (token.kind === "invalid") {
        this.#unsupported(token.construct, token.span, token.message);
      }
    }
  }

  #rejectReservedName(token: Token, name: string): never {
    const owners = RESERVED_NAMES[name];
    const owner = owners?.[0];
    this.#unsupported(
      owner ?? "reserved-name",
      token.span,
      `\`${name}\` belongs to the named ${owner ?? "experiment"} mode; core source excludes it`,
    );
  }

  #rejectWord(word: string, construct: string): void {
    if (this.#isText(word)) {
      this.#unsupported(construct, this.#peek().span, `\`${word}\` is outside the admitted core`);
    }
  }

  // ------------------------------------------------------------------
  // Declarations and statements
  // ------------------------------------------------------------------

  parseProgram(): Program {
    const forms: Array<Decl | Stmt> = [];
    while (this.#peek().kind !== "eof") {
      forms.push(this.#parseDeclarationOrStatement());
    }
    return forms;
  }

  parseOneExpression(): Expr {
    const expr = this.#parseExpression();
    const rest = this.#peek();
    if (rest.kind !== "eof") {
      this.#fail(
        "TrailingInput",
        "trailing-input",
        rest.span,
        "unexpected input after the expression",
      );
    }
    return expr;
  }

  #parseDeclarationOrStatement(): Decl | Stmt {
    this.#rejectWord("class", "class-declaration");
    this.#rejectWord("enum", "enum-declaration");
    this.#rejectWord("namespace", "namespace-declaration");
    this.#rejectWord("module", "module-declaration");
    this.#rejectWord("declare", "declare-declaration");
    this.#rejectWord("abstract", "abstract-declaration");
    this.#rejectWord("do", "do-statement");
    this.#rejectWord("with", "with-statement");
    this.#rejectWord("static", "static-modifier");
    if (this.#isText("export")) {
      return this.#parseExport();
    }
    return this.#parseStatementOrDecl();
  }

  #parseExport(): Decl {
    const start = this.#next().span;
    if (this.#isText("default")) {
      this.#unsupported("export-default", this.#peek().span, "default exports are excluded");
    }
    const inner = this.#parseStatementOrDecl();
    const span = spanOf(start, inner.span);
    switch (inner.tag) {
      case "type-decl":
        return { ...inner, exported: true, span };
      case "interface-decl":
        return { ...inner, exported: true, span };
      case "var-decl":
        return { ...inner, exported: true, span };
      case "function-decl":
        return { ...inner, exported: true, span };
      default:
        return this.#unsupported("export-statement", span, "only declarations may be exported");
    }
  }

  #parseStatementOrDecl(): Decl | Stmt {
    if (this.#isText("import")) {
      return this.#parseImport();
    }
    if (
      this.#isText("type") &&
      this.#peek(1).kind === "ident" &&
      (this.#isText("=", 2) || this.#isText("<", 2))
    ) {
      return this.#parseTypeDecl();
    }
    if (this.#isText("interface") && this.#peek(1).kind === "ident") {
      return this.#parseInterfaceDecl();
    }
    if (this.#isText("const") || this.#isText("let")) {
      return this.#parseVarDecl();
    }
    if (this.#isText("function")) {
      return this.#parseFunctionDecl();
    }
    return this.#parseStatement();
  }

  #parseImport(): Decl {
    const start = this.#next().span;
    // The grammar admits one leading `type` modifier: it makes the whole
    // declaration type-only, so every bound name is erased at run time.
    const typeOnly = this.#eat("type");
    this.#expect("{", "after `import`");
    const names: ImportName[] = [];
    while (!this.#isText("}")) {
      const isType = typeOnly || this.#eat("type");
      const importedToken = this.#expectIdent("for the imported name");
      const local = this.#eat("as") ? this.#expectIdent("after `as`").text : importedToken.text;
      names.push({ imported: importedToken.text, local, isType });
      if (!this.#eat(",")) {
        break;
      }
    }
    this.#expect("}", "to close the import list");
    this.#expect("from", "after the import list");
    const source = this.#next();
    if (source.kind !== "string") {
      this.#fail("SyntaxError", "expected-module-string", source.span, "expected a module string");
    }
    const end = this.#expect(";", "to end the import").span;
    return { tag: "import", names, from: source.value, span: spanOf(start, end) };
  }

  #expectIdent(what: string): Extract<Token, { readonly kind: "ident" }> {
    const token = this.#next();
    if (token.kind !== "ident") {
      return this.#fail(
        "SyntaxError",
        "expected-identifier",
        token.span,
        `expected an identifier ${what}`,
      );
    }
    return token;
  }

  #parseTypeDecl(): Decl {
    const start = this.#next().span;
    const name = this.#expectIdent("for the type name").text;
    const typeParams = this.#parseTypeParams();
    this.#expect("=", "before the aliased type");
    const aliased = this.#parseType();
    const end = this.#expect(";", "to end the type declaration").span;
    return {
      tag: "type-decl",
      name,
      typeParams,
      aliased,
      exported: false,
      span: spanOf(start, end),
    };
  }

  #parseInterfaceDecl(): Decl {
    const start = this.#next().span;
    const name = this.#expectIdent("for the interface name").text;
    const typeParams = this.#parseTypeParams();
    const extendsList: TypeNode[] = [];
    if (this.#eat("extends")) {
      for (;;) {
        extendsList.push(this.#parseType());
        if (!this.#eat(",")) {
          break;
        }
      }
    }
    const body = this.#parseObjectFields();
    return {
      tag: "interface-decl",
      name,
      typeParams,
      extends: extendsList,
      fields: body.fields,
      exported: false,
      span: spanOf(start, body.span),
    };
  }

  #parseObjectFields(): { fields: ReadonlyArray<FieldNode>; span: Span } {
    const open = this.#expect("{", "to open the field list").span;
    const fields: FieldNode[] = [];
    while (!this.#isText("}")) {
      const readonly = this.#eat("readonly");
      const nameToken = this.#expectIdent("for the field name");
      const optional = this.#eat("?");
      this.#expect(":", "before the field type");
      const type = this.#parseType();
      fields.push({ name: nameToken.text, optional, readonly, type, span: nameToken.span });
      if (!this.#eat(";") && !this.#eat(",")) {
        break;
      }
    }
    const close = this.#expect("}", "to close the field list").span;
    return { fields, span: spanOf(open, close) };
  }

  #parseTypeParams(): ReadonlyArray<TypeParamNode> {
    if (!this.#isText("<")) {
      return [];
    }
    this.#next();
    const params: TypeParamNode[] = [];
    while (!this.#isText(">")) {
      const nameToken = this.#expectIdent("for the type parameter");
      const bound = this.#eat("extends") ? this.#parseType() : null;
      params.push({ name: nameToken.text, bound, span: nameToken.span });
      if (!this.#eat(",")) {
        break;
      }
    }
    this.#expect(">", "to close the type parameters");
    if (params.length > 2) {
      this.#unsupported(
        "excess-type-params",
        params[2]?.span ?? this.#peek().span,
        "at most two type parameters are admitted",
      );
    }
    return params;
  }

  #parseVarDecl(): Decl {
    const kind: "const" | "let" = this.#isText("const") ? "const" : "let";
    const kindToken = this.#next();
    const nameToken = this.#expectIdent("for the binding name");
    this.#rejectWord("var", "var-declaration");
    const declaredType = this.#eat(":") ? this.#parseType() : null;
    this.#expect("=", "before the initializer");
    const init = this.#parseExpression();
    const end = this.#expect(";", "to end the declaration").span;
    return {
      tag: "var-decl",
      kind,
      name: nameToken.text,
      declaredType,
      init,
      exported: false,
      span: spanOf(kindToken.span, end),
    };
  }

  #parseFunctionDecl(): Decl {
    const start = this.#next().span;
    if (this.#isText("*")) {
      this.#unsupported("generator", this.#peek().span, "generator functions are excluded");
    }
    const name = this.#expectIdent("for the function name").text;
    const typeParams = this.#parseTypeParams();
    const params = this.#parseParameters();
    const returnType = this.#eat(":") ? this.#parseType() : null;
    const body = this.#parseBlock();
    return {
      tag: "function-decl",
      name,
      typeParams,
      params,
      returnType,
      body,
      exported: false,
      span: spanOf(start, body.span),
    };
  }

  #parseParameters(): ReadonlyArray<Param> {
    this.#expect("(", "to open the parameter list");
    const params: Param[] = [];
    while (!this.#isText(")")) {
      if (this.#isText("readonly")) {
        this.#unsupported(
          "readonly-parameter",
          this.#peek().span,
          "readonly parameters are excluded",
        );
      }
      const isRest = this.#eat("...");
      const nameToken = this.#expectIdent("for the parameter name");
      const optional = !isRest && this.#eat("?");
      this.#expect(":", "before the parameter type");
      const type = this.#parseType();
      if (isRest) {
        params.push({ kind: "rest", name: nameToken.text, type, span: nameToken.span });
      } else {
        params.push({ kind: "param", name: nameToken.text, optional, type, span: nameToken.span });
      }
      if (!this.#eat(",")) {
        break;
      }
    }
    this.#expect(")", "to close the parameter list");
    return params;
  }

  #parseStatement(): Stmt {
    if (this.#isText("{")) {
      const body = this.#parseBlock();
      return { tag: "block", body: body.body, span: body.span };
    }
    if (this.#isText("if")) {
      return this.#parseIf();
    }
    if (this.#isText("while")) {
      return this.#parseWhile();
    }
    if (this.#isText("for")) {
      return this.#parseForOf();
    }
    if (this.#isText("switch")) {
      return this.#parseSwitch();
    }
    if (this.#isText("return")) {
      const start = this.#next().span;
      const argument = this.#isText(";") ? null : this.#parseExpression();
      const end = this.#expect(";", "to end the return").span;
      return { tag: "return", argument, span: spanOf(start, end) };
    }
    if (this.#isText("break")) {
      const start = this.#next().span;
      const end = this.#expect(";", "to end the break").span;
      return { tag: "break", span: spanOf(start, end) };
    }
    if (this.#isText("continue")) {
      const start = this.#next().span;
      const end = this.#expect(";", "to end the continue").span;
      return { tag: "continue", span: spanOf(start, end) };
    }
    if (this.#isText("throw")) {
      const start = this.#next().span;
      const argument = this.#parseExpression();
      const end = this.#expect(";", "to end the throw").span;
      return { tag: "throw", argument, span: spanOf(start, end) };
    }
    if (this.#isText("try")) {
      return this.#parseTry();
    }
    const next = this.#peek(1);
    if (this.#peek().kind === "ident" && next.kind === "punct" && next.text === ":") {
      this.#unsupported("labeled-statement", this.#peek().span, "labeled statements are excluded");
    }
    const expr = this.#parseExpression();
    const end = this.#expect(";", "to end the expression statement").span;
    return { tag: "expr-stmt", expr, span: spanOf(expr.span, end) };
  }

  #parseBlock(): Block {
    const start = this.#expect("{", "to open the block").span;
    const body: Array<Decl | Stmt> = [];
    while (!this.#isText("}")) {
      if (this.#peek().kind === "eof") {
        this.#fail("SyntaxError", "unterminated-block", this.#peek().span, "unterminated block");
      }
      body.push(this.#parseDeclarationOrStatement());
    }
    const end = this.#next().span;
    return { body, span: spanOf(start, end) };
  }

  #parseIf(): Stmt {
    const start = this.#next().span;
    this.#expect("(", "after `if`");
    const test = this.#parseExpression();
    this.#expect(")", "after the condition");
    const consequent = this.#parseStatement();
    let alternative: Stmt | null = null;
    if (this.#eat("else")) {
      alternative = this.#parseStatement();
    }
    return {
      tag: "if",
      test,
      consequent,
      alternative,
      span: spanOf(start, (alternative ?? consequent).span),
    };
  }

  #parseWhile(): Stmt {
    const start = this.#next().span;
    this.#expect("(", "after `while`");
    const test = this.#parseExpression();
    this.#expect(")", "after the condition");
    const body = this.#parseStatement();
    return { tag: "while", test, body, span: spanOf(start, body.span) };
  }

  #parseForOf(): Stmt {
    const start = this.#next().span;
    this.#expect("(", "after `for`");
    if (!this.#isText("const")) {
      this.#unsupported(
        "for-counter-or-in-loop",
        this.#peek().span,
        "only `for (const x of ...)` loops are admitted",
      );
    }
    this.#next();
    const nameToken = this.#expectIdent("for the loop binding");
    if (!this.#eat("of")) {
      this.#unsupported(
        "for-counter-or-in-loop",
        this.#peek().span,
        "only `for (const x of ...)` loops are admitted",
      );
    }
    const iterable = this.#parseExpression();
    this.#expect(")", "after the iterable");
    const body = this.#parseStatement();
    return { tag: "for-of", name: nameToken.text, iterable, body, span: spanOf(start, body.span) };
  }

  #parseSwitch(): Stmt {
    const start = this.#next().span;
    this.#expect("(", "after `switch`");
    const discriminant = this.#parseExpression();
    this.#expect(")", "after the discriminant");
    this.#expect("{", "to open the switch body");
    const cases: CaseClause[] = [];
    let defaultBody: ReadonlyArray<Decl | Stmt> | null = null;
    while (!this.#isText("}")) {
      if (this.#eat("case")) {
        const test = this.#parseExpression();
        this.#expect(":", "after the case label");
        const body = this.#parseCaseBody();
        cases.push({ test, body, span: test.span });
        continue;
      }
      if (this.#eat("default")) {
        this.#expect(":", "after `default`");
        defaultBody = this.#parseCaseBody();
        continue;
      }
      this.#fail(
        "SyntaxError",
        "expected-case-clause",
        this.#peek().span,
        "expected `case`, `default`, or `}`",
      );
    }
    const end = this.#next().span;
    return { tag: "switch", discriminant, cases, defaultBody, span: spanOf(start, end) };
  }

  #parseCaseBody(): ReadonlyArray<Decl | Stmt> {
    const body: Array<Decl | Stmt> = [];
    while (!this.#isText("case") && !this.#isText("default") && !this.#isText("}")) {
      if (this.#peek().kind === "eof") {
        this.#fail("SyntaxError", "unterminated-switch", this.#peek().span, "unterminated switch");
      }
      body.push(this.#parseDeclarationOrStatement());
    }
    return body;
  }

  #parseTry(): Stmt {
    const start = this.#next().span;
    const block = this.#parseBlock();
    let handler: { param: string | null; body: Block } | null = null;
    let finalizer: Block | null = null;
    if (this.#eat("catch")) {
      let param: string | null = null;
      if (this.#eat("(")) {
        param = this.#expectIdent("for the catch binding").text;
        this.#expect(")", "after the catch binding");
      }
      handler = { param, body: this.#parseBlock() };
    }
    if (this.#eat("finally")) {
      finalizer = this.#parseBlock();
    }
    if (handler === null && finalizer === null) {
      this.#fail(
        "SyntaxError",
        "expected-catch-or-finally",
        this.#peek().span,
        "expected `catch` or `finally`",
      );
    }
    return {
      tag: "try",
      block,
      handler,
      finalizer,
      span: spanOf(start, (finalizer ?? handler?.body ?? block).span),
    };
  }

  // ------------------------------------------------------------------
  // Types
  // ------------------------------------------------------------------

  #parseType(): TypeNode {
    this.#eat("|");
    const first = this.#parsePostfixType();
    let last = first;
    const members: TypeNode[] = [first];
    while (this.#eat("|")) {
      last = this.#parsePostfixType();
      members.push(last);
    }
    if (members.length === 1) {
      return first;
    }
    return { tag: "union-type", members, span: spanOf(first.span, last.span) };
  }

  #parsePostfixType(): TypeNode {
    let inner = this.#parsePrimaryType();
    while (this.#isText("[")) {
      const open = this.#next();
      if (!this.#isText("]")) {
        this.#unsupported(
          "indexed-access-type",
          spanOf(inner.span, open.span),
          "indexed access types are excluded",
        );
      }
      const close = this.#next();
      inner = { tag: "array-type", element: inner, span: spanOf(inner.span, close.span) };
    }
    return inner;
  }

  #parsePrimaryType(): TypeNode {
    const token = this.#peek();
    if (token.kind === "number") {
      this.#next();
      return { tag: "literal-type", value: token.value, span: token.span };
    }
    if (token.kind === "string") {
      this.#next();
      return { tag: "literal-type", value: token.value, span: token.span };
    }
    if (token.kind === "ident" && (token.text === "true" || token.text === "false")) {
      this.#next();
      return { tag: "literal-type", value: token.text === "true", span: token.span };
    }
    const primitiveName = token.kind === "ident" ? PRIMITIVE_TYPES[token.text] : undefined;
    if (primitiveName !== undefined) {
      this.#next();
      return { tag: "primitive-type", name: primitiveName, span: token.span };
    }
    if (this.#isText("[")) {
      return this.#parseTupleType();
    }
    if (this.#isText("{")) {
      const body = this.#parseObjectFields();
      return { tag: "object-type", fields: body.fields, span: body.span };
    }
    if (this.#isText("(")) {
      return this.#parseParenOrFunctionType();
    }
    if (token.kind === "ident") {
      return this.#parseTypeReference();
    }
    this.#fail("SyntaxError", "expected-type", token.span, "expected a type");
  }

  #parseTypeReference(): TypeNode {
    const nameToken = this.#expectIdent("for the type name");
    let name = nameToken.text;
    let end = nameToken.span;
    if (this.#eat(".")) {
      const memberToken = this.#expectIdent("after the qualified name");
      name = `${name}.${memberToken.text}`;
      end = memberToken.span;
    }
    const args: TypeNode[] = [];
    if (this.#eat("<")) {
      while (!this.#isText(">")) {
        args.push(this.#parseType());
        if (!this.#eat(",")) {
          break;
        }
      }
      end = this.#expect(">", "to close the type arguments").span;
    }
    return { tag: "type-reference", name, args, span: spanOf(nameToken.span, end) };
  }

  #parseTupleType(): TypeNode {
    const start = this.#expect("[", "to open the tuple type").span;
    const elements: TypeNode[] = [];
    while (!this.#isText("]")) {
      elements.push(this.#parseType());
      if (!this.#eat(",")) {
        break;
      }
    }
    const end = this.#expect("]", "to close the tuple type").span;
    return { tag: "tuple-type", elements, span: spanOf(start, end) };
  }

  #parseParenOrFunctionType(): TypeNode {
    const start = this.#peek().span;
    if (this.#isFunctionTypeAhead()) {
      const params = this.#parseParameters();
      this.#expect("=>", "before the function type result");
      const result = this.#parseType();
      return { tag: "function-type", params, result, span: spanOf(start, result.span) };
    }
    this.#next();
    const inner = this.#parseType();
    this.#expect(")", "to close the parenthesized type");
    return inner;
  }

  #isFunctionTypeAhead(): boolean {
    // A function type's parameter list is `()`, a rest parameter, or a
    // parameter name followed by `:` or `?` (grammar: `Identifier ["?"] ":"
    // Type`); anything else after `(` starts a parenthesized type.
    const opener = this.#tokens[this.#index + 1];
    if (opener?.kind === "punct" && opener.text === "...") {
      return true;
    }
    if (opener?.kind === "ident") {
      return this.#isText(":", 2) || this.#isText("?", 2);
    }
    if (opener?.kind !== "punct" || opener.text !== ")") {
      return false;
    }
    let depth = 0;
    for (let i = this.#index; i < this.#tokens.length; i += 1) {
      const token = this.#tokens[i];
      if (token === undefined) {
        return false;
      }
      if (token.kind === "punct" && "([{".includes(token.text)) {
        depth += 1;
        continue;
      }
      if (token.kind === "punct" && ")]}".includes(token.text)) {
        depth -= 1;
        if (depth === 0) {
          return this.#isText("=>", i + 1 - this.#index);
        }
      }
    }
    return false;
  }

  // ------------------------------------------------------------------
  // Expressions
  // ------------------------------------------------------------------

  #parseExpression(): Expr {
    return this.#parseAssignment();
  }

  #parseAssignment(): Expr {
    const start = this.#parseConditional();
    if (!this.#eat("=")) {
      return start;
    }
    const value = this.#parseAssignment();
    if (start.tag !== "variable" && start.tag !== "member" && start.tag !== "index") {
      this.#fail(
        "SyntaxError",
        "invalid-assignment-target",
        start.span,
        "the assignment target must be a name or property",
      );
    }
    return { tag: "assign", target: start, value, span: spanOf(start.span, value.span) };
  }

  #parseConditional(): Expr {
    const test = this.#parseBinary(0);
    if (!this.#eat("?")) {
      return test;
    }
    const consequent = this.#parseExpression();
    this.#expect(":", "in the conditional expression");
    const alternative = this.#parseExpression();
    return {
      tag: "conditional",
      test,
      consequent,
      alternative,
      span: spanOf(test.span, alternative.span),
    };
  }

  #parseBinary(level: number): Expr {
    const levels = BINARY_LEVELS;
    if (level >= levels.length) {
      return this.#parseUnary();
    }
    let left = this.#parseBinary(level + 1);
    for (;;) {
      const token = this.#peek();
      const op = token.kind === "punct" ? OPERATOR_TEXTS[token.text] : undefined;
      const isOperator =
        token.kind === "punct" && op !== undefined && (levels[level] ?? []).includes(token.text);
      if (!isOperator || op === undefined) {
        this.#rejectExcludedOperator();
        return left;
      }
      this.#next();
      const right = this.#parseBinary(level + 1);
      const span = spanOf(left.span, right.span);
      left =
        op === "&&" || op === "||"
          ? { tag: "logical", op, left, right, span }
          : { tag: "binary", op, left, right, span };
    }
  }

  #rejectExcludedOperator(): void {
    const token = this.#peek();
    if (token.kind === "punct") {
      const construct = EXCLUDED_OPERATORS[token.text];
      if (construct !== undefined) {
        this.#unsupported(
          construct,
          token.span,
          `\`${token.text}\` is outside the admitted operator set`,
        );
      }
      return;
    }
    if (token.kind === "ident") {
      const construct = EXCLUDED_KEYWORD_OPERATORS[token.text];
      if (construct !== undefined) {
        this.#unsupported(
          construct,
          token.span,
          `\`${token.text}\` is outside the admitted operator set`,
        );
      }
    }
  }

  #parseUnary(): Expr {
    const token = this.#peek();
    const prefixOp = token.kind === "punct" ? PREFIX_OPERATORS[token.text] : undefined;
    const isTypeof = token.kind === "ident" && token.text === "typeof";
    if (prefixOp === undefined && !isTypeof) {
      return this.#parsePostfix();
    }
    this.#next();
    const operand = this.#parseUnary();
    const op: "!" | "+" | "-" | "typeof" = prefixOp ?? "typeof";
    return { tag: "unary", op, operand, span: spanOf(token.span, operand.span) };
  }

  #parsePostfix(): Expr {
    let expr = this.#parsePrimary();
    for (;;) {
      if (this.#isText(".")) {
        this.#next();
        const name = this.#expectIdent("after `.`");
        expr = { tag: "member", object: expr, name: name.text, span: spanOf(expr.span, name.span) };
        continue;
      }
      if (this.#isText("[")) {
        this.#next();
        const index = this.#parseExpression();
        const end = this.#expect("]", "to close the index").span;
        expr = { tag: "index", object: expr, index, span: spanOf(expr.span, end) };
        continue;
      }
      if (this.#isText("(")) {
        const { args, end } = this.#parseArguments();
        expr = { tag: "call", callee: expr, args, span: spanOf(expr.span, end) };
        continue;
      }
      if (this.#isText("`")) {
        this.#unsupported("tagged-template", this.#peek().span, "tagged templates are excluded");
      }
      if (this.#isText("as") || this.#isText("satisfies")) {
        this.#unsupported(
          "type-assertion",
          this.#peek().span,
          "type assertions and `satisfies` are excluded",
        );
      }
      if (this.#isText("!")) {
        this.#unsupported(
          "non-null-assertion",
          this.#peek().span,
          "non-null assertions are excluded",
        );
      }
      if (this.#isText("?.") || this.#isText("??")) {
        this.#unsupported(
          "optional-chaining",
          this.#peek().span,
          "optional chaining and nullish coalescing are excluded",
        );
      }
      return expr;
    }
  }

  #parseArguments(): { args: ReadonlyArray<Arg>; end: Span } {
    this.#expect("(", "to open the argument list");
    const args: Arg[] = [];
    while (!this.#isText(")")) {
      if (this.#eat("...")) {
        args.push({ kind: "spread", expr: this.#parseExpression() });
      } else {
        args.push({ kind: "item", expr: this.#parseExpression() });
      }
      if (!this.#eat(",")) {
        break;
      }
    }
    const end = this.#expect(")", "to close the argument list").span;
    return { args, end };
  }

  #parsePrimary(): Expr {
    const token = this.#peek();
    if (token.kind === "number") {
      this.#next();
      return { tag: "number", value: token.value, span: token.span };
    }
    if (token.kind === "string") {
      this.#next();
      return { tag: "string", value: token.value, span: token.span };
    }
    if (token.kind === "template-chunk" || this.#isText("`")) {
      return this.#parseTemplate();
    }
    if (this.#isText("(")) {
      return this.#parseParenOrArrow();
    }
    if (this.#isText("[")) {
      return this.#parseArrayLiteral();
    }
    if (this.#isText("{")) {
      return this.#parseObjectLiteral();
    }
    if (this.#isText("new")) {
      return this.#parseNew();
    }
    if (this.#isText("<")) {
      this.#unsupported("type-assertion", token.span, "type assertions are excluded");
    }
    if (this.#isText("/")) {
      this.#unsupported("regex-literal", token.span, "regular-expression literals are excluded");
    }
    if (token.kind === "punct" && EXCLUDED_OPERATORS[token.text] !== undefined) {
      this.#unsupported(
        EXCLUDED_OPERATORS[token.text] ?? "excluded-operator",
        token.span,
        "excluded operator",
      );
    }
    if (token.kind === "ident" && EXCLUDED_KEYWORD_OPERATORS[token.text] !== undefined) {
      this.#unsupported(
        EXCLUDED_KEYWORD_OPERATORS[token.text] ?? "excluded-operator",
        token.span,
        "excluded operator",
      );
    }
    if (token.kind === "ident") {
      return this.#parseNameOrArrow();
    }
    this.#fail("SyntaxError", "expected-expression", token.span, "expected an expression");
  }

  #parseNameOrArrow(): Expr {
    const token = this.#next();
    if (token.kind !== "ident") {
      return this.#fail("SyntaxError", "expected-identifier", token.span, "expected an identifier");
    }
    const name = token.text;
    const owner = RESERVED_NAMES[name];
    if (owner !== undefined) {
      if (this.#isText("(")) {
        return this.#parseExperimentForm(name, owner, token.span);
      }
      this.#rejectReservedName(token, name);
    }
    if (this.#isText("=>")) {
      this.#next();
      const body = this.#parseArrowBody();
      const param: Param = { kind: "param", name, optional: false, type: null, span: token.span };
      return { tag: "arrow", params: [param], body, span: spanOf(token.span, body.span) };
    }
    if (name === "true" || name === "false") {
      return { tag: "boolean", value: name === "true", span: token.span };
    }
    if (name === "null") {
      return { tag: "null", span: token.span };
    }
    if (name === "undefined") {
      return { tag: "undefined", span: token.span };
    }
    return { tag: "variable", name, span: token.span };
  }

  #parseExperimentForm(name: string, owners: ReadonlyArray<ExperimentMode>, start: Span): Expr {
    if (!owners.includes(this.#mode)) {
      this.#unsupported(
        owners[0] ?? "reserved-name",
        spanOf(start, this.#peek().span),
        `\`${name}\` requires a named experiment mode`,
      );
    }
    const { args, end } = this.#parseArguments();
    const span = spanOf(start, end);
    const spread = args.find((arg) => arg.kind === "spread");
    if (spread !== undefined) {
      this.#unsupported(
        "spread-argument",
        spread.expr.span,
        "extension forms take plain arguments",
      );
    }
    const exprs = args.map((arg) => arg.expr);
    if (name === "delay" || name === "force") {
      const only = exprs[0];
      if (only === undefined || exprs.length !== 1) {
        this.#fail("SyntaxError", "wrong-arity", span, `\`${name}\` takes exactly one argument`);
      }
      return { tag: name, expr: only, span };
    }
    if (name === "require") {
      const only = exprs[0];
      if (only === undefined || exprs.length !== 1) {
        this.#fail("SyntaxError", "wrong-arity", span, "`require` takes exactly one argument");
      }
      return { tag: "require", condition: only, span };
    }
    if (name === "permanentAssign") {
      const target = exprs[0];
      const value = exprs[1];
      if (target === undefined || value === undefined || exprs.length !== 2) {
        this.#fail(
          "SyntaxError",
          "wrong-arity",
          span,
          "`permanentAssign` takes exactly two arguments",
        );
      }
      if (target.tag !== "variable" && target.tag !== "member" && target.tag !== "index") {
        this.#fail(
          "SyntaxError",
          "invalid-assignment-target",
          target.span,
          "the assignment target must be a name or property",
        );
      }
      return { tag: "permanent-assign", target, value, span };
    }
    if (name === "ifFail") {
      const expression = exprs[0];
      const fallback = exprs[1];
      if (expression === undefined || fallback === undefined || exprs.length !== 2) {
        this.#fail("SyntaxError", "wrong-arity", span, "`ifFail` takes exactly two arguments");
      }
      return { tag: "if-fail", expression, fallback, span };
    }
    return { tag: name === "choose" ? "choose" : "ramb", alternatives: exprs, span };
  }

  #parseTemplate(): Expr {
    const start = this.#expect("`", "to open the template").span;
    const chunks: string[] = [];
    const exprs: Expr[] = [];
    for (;;) {
      if (this.#peek().kind === "template-chunk") {
        const chunk = this.#next();
        chunks.push(chunk.kind === "template-chunk" ? chunk.value : "");
      } else {
        chunks.push("");
      }
      if (this.#eat("${")) {
        exprs.push(this.#parseExpression());
        this.#expect("}", "to close the template interpolation");
        continue;
      }
      const end = this.#expect("`", "to close the template").span;
      return { tag: "template", chunks, exprs, span: spanOf(start, end) };
    }
  }

  #parseParenOrArrow(): Expr {
    const start = this.#peek().span;
    if (this.#isArrowAhead()) {
      return this.#parseParenArrow(start);
    }
    this.#next();
    const inner = this.#parseExpression();
    this.#expect(")", "to close the parenthesized expression");
    return inner;
  }

  #isArrowAhead(): boolean {
    let depth = 0;
    for (let i = this.#index; i < this.#tokens.length; i += 1) {
      const token = this.#tokens[i];
      if (token === undefined) {
        return false;
      }
      if (token.kind === "punct" && "([{".includes(token.text)) {
        depth += 1;
        continue;
      }
      if (token.kind === "punct" && ")]}".includes(token.text)) {
        depth -= 1;
        if (depth !== 0) {
          continue;
        }
        if (this.#isText("=>", i + 1 - this.#index)) {
          return true;
        }
        if (this.#isText(":", i + 1 - this.#index)) {
          return this.#typeThenArrow(i + 2);
        }
        return false;
      }
    }
    return false;
  }

  /** Whether a type starting at absolute token index `from` ends in `=>`. */
  #typeThenArrow(from: number): boolean {
    let depth = 0;
    for (let i = from; i < this.#tokens.length; i += 1) {
      const token = this.#tokens[i];
      if (token === undefined) {
        return false;
      }
      if (token.kind === "punct" && "([{<".includes(token.text)) {
        depth += 1;
        continue;
      }
      if (token.kind === "punct" && ")]}>".includes(token.text)) {
        depth -= 1;
        continue;
      }
      if (depth === 0 && token.kind === "punct" && token.text === "=>") {
        return true;
      }
      if (depth < 0) {
        return false;
      }
    }
    return false;
  }

  #parseParenArrow(start: Span): Expr {
    this.#expect("(", "to open the arrow parameters");
    const params: Param[] = [];
    while (!this.#isText(")")) {
      if (this.#eat("...")) {
        const nameToken = this.#expectIdent("for the rest parameter");
        this.#expect(":", "before the rest parameter type");
        const type = this.#parseType();
        params.push({ kind: "rest", name: nameToken.text, type, span: nameToken.span });
        break;
      }
      const nameToken = this.#expectIdent("for the parameter name");
      const optional = this.#eat("?");
      const type = this.#eat(":") ? this.#parseType() : null;
      params.push({ kind: "param", name: nameToken.text, optional, type, span: nameToken.span });
      if (!this.#eat(",")) {
        break;
      }
    }
    this.#expect(")", "to close the arrow parameters");
    if (this.#eat(":")) {
      this.#parseType();
    }
    this.#expect("=>", "after the arrow parameters");
    const body = this.#parseArrowBody();
    return { tag: "arrow", params, body, span: spanOf(start, body.span) };
  }

  #parseArrowBody(): Block {
    if (this.#isText("{")) {
      return this.#parseBlock();
    }
    const expr = this.#parseExpression();
    return { body: [{ tag: "return", argument: expr, span: expr.span }], span: expr.span };
  }

  #parseArrayLiteral(): Expr {
    const start = this.#expect("[", "to open the array literal").span;
    const elements: Array<{ kind: "item" | "spread"; expr: Expr }> = [];
    while (!this.#isText("]")) {
      if (this.#eat("...")) {
        elements.push({ kind: "spread", expr: this.#parseExpression() });
      } else {
        elements.push({ kind: "item", expr: this.#parseExpression() });
      }
      if (!this.#eat(",")) {
        break;
      }
    }
    const end = this.#expect("]", "to close the array literal").span;
    return { tag: "array", elements, span: spanOf(start, end) };
  }

  #parseObjectLiteral(): Expr {
    const start = this.#expect("{", "to open the object literal").span;
    const fields: ObjectField[] = [];
    while (!this.#isText("}")) {
      const keyToken = this.#peek();
      if (keyToken.kind === "punct" && keyToken.text === "[") {
        this.#unsupported("computed-key", keyToken.span, "computed keys are excluded");
      }
      if (keyToken.kind === "ident" && (keyToken.text === "get" || keyToken.text === "set")) {
        this.#unsupported("accessor", keyToken.span, "accessors are excluded");
      }
      const key = this.#expectObjectKey();
      if (this.#isText("(")) {
        this.#unsupported(
          "method-definition",
          this.#peek().span,
          "method definitions are excluded",
        );
      }
      if (this.#eat(":")) {
        const value = this.#parseExpression();
        fields.push({ key, value, span: spanOf(keyToken.span, value.span) });
      } else {
        fields.push({
          key,
          value: { tag: "variable", name: key, span: keyToken.span },
          span: keyToken.span,
        });
      }
      if (!this.#eat(",")) {
        break;
      }
    }
    const end = this.#expect("}", "to close the object literal").span;
    return { tag: "object", fields, span: spanOf(start, end) };
  }

  #expectObjectKey(): string {
    const token = this.#next();
    if (token.kind === "ident") {
      return token.text;
    }
    if (token.kind === "string") {
      return token.value;
    }
    this.#fail("SyntaxError", "expected-field-name", token.span, "expected a field name");
  }

  #parseNew(): Expr {
    const start = this.#next().span;
    const target = this.#next();
    if (target.kind !== "ident") {
      this.#unsupported(
        "unsupported-new-target",
        target.span,
        "`new` admits only `Error`, `Map`, and `Set`",
      );
    }
    const typeArgs = this.#eat("<") ? this.#parseTypeArgs() : [];
    const { args, end } = this.#parseArguments();
    const spread = args.find((arg) => arg.kind === "spread");
    if (spread !== undefined) {
      this.#unsupported("spread-argument", spread.expr.span, "spread arguments are excluded here");
    }
    const exprs = args.map((arg) => arg.expr);
    const span = spanOf(start, end);
    if (target.text === "Error") {
      return { tag: "new-error", args: exprs, span };
    }
    if (target.text === "Map") {
      return { tag: "new-map", args: exprs, typeArgs, span };
    }
    if (target.text === "Set") {
      return { tag: "new-set", args: exprs, typeArgs, span };
    }
    this.#unsupported(
      "unsupported-new-target",
      target.span,
      "`new` admits only `Error`, `Map`, and `Set`",
    );
  }

  #parseTypeArgs(): ReadonlyArray<TypeNode> {
    const args: TypeNode[] = [];
    while (!this.#isText(">")) {
      args.push(this.#parseType());
      if (!this.#eat(",")) {
        break;
      }
    }
    this.#expect(">", "to close the type arguments");
    return args;
  }
}

/** Binary operator precedence, loosest first. */
const BINARY_LEVELS: ReadonlyArray<ReadonlyArray<string>> = [
  ["||"],
  ["&&"],
  ["===", "!=="],
  ["<", "<=", ">", ">="],
  ["+", "-"],
  ["*", "/", "%"],
];

/** Parses `text` as one expression; trailing input is rejected. */
export const parseExpression = (text: string): Expr =>
  new Parser(text, "core").parseOneExpression();

/** Parses `text` as a program (zero or more forms in source order). */
export const parseProgram = (text: string): Program => new Parser(text, "core").parseProgram();

/** Parses `text` in one named execution mode, admitting that mode's extension nodes. */
export const parseExperiment = (text: string, mode: ExperimentMode): Program =>
  new Parser(text, mode).parseProgram();
