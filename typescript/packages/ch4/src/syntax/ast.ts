// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: sections 4.1 to 5.5

/**
 * The shared typed syntax: one tagged discriminated union for every admitted
 * source production (host-subsets grammar sections 1.1 and 3). Each node
 * records its source span. This AST is the common boundary every evaluator,
 * analyzer, machine, and compiler reads; no chapter keeps a second
 * representation. The representation of a guest evaluator's own syntax is an
 * ordinary recursive data type like this one: data, never host executable
 * code.
 */
import { noSpan, type Span } from "./diagnostics.ts";

// ---------------------------------------------------------------------
// Types (declaration-level annotations)
// ---------------------------------------------------------------------

/** A primitive keyword type. */
export type PrimitiveTypeName =
  | "number"
  | "string"
  | "boolean"
  | "null"
  | "undefined"
  | "void"
  | "unknown";

/** A property/field declaration inside an object type or interface. */
export interface FieldNode {
  readonly name: string;
  readonly optional: boolean;
  readonly readonly: boolean;
  readonly type: TypeNode;
  readonly span: Span;
}

/** A parameter in a parameter list. */
export type Param =
  | {
      readonly kind: "param";
      readonly name: string;
      readonly optional: boolean;
      readonly type: TypeNode | null;
      readonly span: Span;
    }
  | { readonly kind: "rest"; readonly name: string; readonly type: TypeNode; readonly span: Span };

/** A generic type parameter with an optional bound. */
export interface TypeParamNode {
  readonly name: string;
  readonly bound: TypeNode | null;
  readonly span: Span;
}

/** The admitted type grammar. */
export type TypeNode =
  | { readonly tag: "primitive-type"; readonly name: PrimitiveTypeName; readonly span: Span }
  | { readonly tag: "literal-type"; readonly value: number | string | boolean; readonly span: Span }
  | {
      readonly tag: "type-reference";
      readonly name: string;
      readonly args: ReadonlyArray<TypeNode>;
      readonly span: Span;
    }
  | { readonly tag: "array-type"; readonly element: TypeNode; readonly span: Span }
  | { readonly tag: "tuple-type"; readonly elements: ReadonlyArray<TypeNode>; readonly span: Span }
  | { readonly tag: "object-type"; readonly fields: ReadonlyArray<FieldNode>; readonly span: Span }
  | {
      readonly tag: "function-type";
      readonly params: ReadonlyArray<Param>;
      readonly result: TypeNode;
      readonly span: Span;
    }
  | { readonly tag: "union-type"; readonly members: ReadonlyArray<TypeNode>; readonly span: Span };

// ---------------------------------------------------------------------
// Expressions
// ---------------------------------------------------------------------

/** One element of an array literal or call argument list. */
export type Arg =
  | { readonly kind: "item"; readonly expr: Expr }
  | { readonly kind: "spread"; readonly expr: Expr };

/** One object-literal field. */
export interface ObjectField {
  readonly key: string;
  readonly value: Expr;
  readonly span: Span;
}

/** The admitted expression grammar plus the named experiment extensions. */
export type Expr =
  | { readonly tag: "number"; readonly value: number; readonly span: Span }
  | { readonly tag: "string"; readonly value: string; readonly span: Span }
  | { readonly tag: "boolean"; readonly value: boolean; readonly span: Span }
  | { readonly tag: "null"; readonly span: Span }
  | { readonly tag: "undefined"; readonly span: Span }
  | {
      readonly tag: "template";
      readonly chunks: ReadonlyArray<string>;
      readonly exprs: ReadonlyArray<Expr>;
      readonly span: Span;
    }
  | { readonly tag: "variable"; readonly name: string; readonly span: Span }
  | { readonly tag: "array"; readonly elements: ReadonlyArray<Arg>; readonly span: Span }
  | { readonly tag: "object"; readonly fields: ReadonlyArray<ObjectField>; readonly span: Span }
  | {
      readonly tag: "unary";
      readonly op: "!" | "+" | "-" | "typeof";
      readonly operand: Expr;
      readonly span: Span;
    }
  | {
      readonly tag: "binary";
      readonly op: "+" | "-" | "*" | "/" | "%" | "<" | "<=" | ">" | ">=" | "===" | "!==";
      readonly left: Expr;
      readonly right: Expr;
      readonly span: Span;
    }
  | {
      readonly tag: "logical";
      readonly op: "&&" | "||";
      readonly left: Expr;
      readonly right: Expr;
      readonly span: Span;
    }
  | {
      readonly tag: "conditional";
      readonly test: Expr;
      readonly consequent: Expr;
      readonly alternative: Expr;
      readonly span: Span;
    }
  | { readonly tag: "assign"; readonly target: Expr; readonly value: Expr; readonly span: Span }
  | {
      readonly tag: "permanent-assign";
      readonly target: Expr;
      readonly value: Expr;
      readonly span: Span;
    }
  | {
      readonly tag: "if-fail";
      readonly expression: Expr;
      readonly fallback: Expr;
      readonly span: Span;
    }
  | {
      readonly tag: "arrow";
      readonly params: ReadonlyArray<Param>;
      readonly body: Block;
      readonly span: Span;
    }
  | {
      readonly tag: "call";
      readonly callee: Expr;
      readonly args: ReadonlyArray<Arg>;
      readonly span: Span;
    }
  | { readonly tag: "member"; readonly object: Expr; readonly name: string; readonly span: Span }
  | { readonly tag: "index"; readonly object: Expr; readonly index: Expr; readonly span: Span }
  | { readonly tag: "new-error"; readonly args: ReadonlyArray<Expr>; readonly span: Span }
  | {
      readonly tag: "new-map";
      readonly args: ReadonlyArray<Expr>;
      readonly typeArgs: ReadonlyArray<TypeNode>;
      readonly span: Span;
    }
  | {
      readonly tag: "new-set";
      readonly args: ReadonlyArray<Expr>;
      readonly typeArgs: ReadonlyArray<TypeNode>;
      readonly span: Span;
    }
  | { readonly tag: "delay"; readonly expr: Expr; readonly span: Span }
  | { readonly tag: "force"; readonly expr: Expr; readonly span: Span }
  | { readonly tag: "choose"; readonly alternatives: ReadonlyArray<Expr>; readonly span: Span }
  | { readonly tag: "ramb"; readonly alternatives: ReadonlyArray<Expr>; readonly span: Span }
  | { readonly tag: "require"; readonly condition: Expr; readonly span: Span };

// ---------------------------------------------------------------------
// Statements and declarations
// ---------------------------------------------------------------------

/** A braced block of declarations and statements. */
export interface Block {
  readonly body: ReadonlyArray<Decl | Stmt>;
  readonly span: Span;
}

/** One switch case clause. */
export interface CaseClause {
  readonly test: Expr;
  readonly body: ReadonlyArray<Decl | Stmt>;
  readonly span: Span;
}

/** The admitted statement grammar. */
export type Stmt =
  | { readonly tag: "block"; readonly body: ReadonlyArray<Decl | Stmt>; readonly span: Span }
  | {
      readonly tag: "if";
      readonly test: Expr;
      readonly consequent: Stmt;
      readonly alternative: Stmt | null;
      readonly span: Span;
    }
  | { readonly tag: "while"; readonly test: Expr; readonly body: Stmt; readonly span: Span }
  | {
      readonly tag: "for-of";
      readonly name: string;
      readonly iterable: Expr;
      readonly body: Stmt;
      readonly span: Span;
    }
  | {
      readonly tag: "switch";
      readonly discriminant: Expr;
      readonly cases: ReadonlyArray<CaseClause>;
      readonly defaultBody: ReadonlyArray<Decl | Stmt> | null;
      readonly span: Span;
    }
  | { readonly tag: "return"; readonly argument: Expr | null; readonly span: Span }
  | { readonly tag: "break"; readonly span: Span }
  | { readonly tag: "continue"; readonly span: Span }
  | { readonly tag: "throw"; readonly argument: Expr; readonly span: Span }
  | {
      readonly tag: "try";
      readonly block: Block;
      readonly handler: { readonly param: string | null; readonly body: Block } | null;
      readonly finalizer: Block | null;
      readonly span: Span;
    }
  | { readonly tag: "expr-stmt"; readonly expr: Expr; readonly span: Span };

/** One imported binding of an import declaration. */
export interface ImportName {
  readonly imported: string;
  readonly local: string;
  readonly isType: boolean;
}

/** The admitted declaration grammar. */
export type Decl =
  | {
      readonly tag: "import";
      readonly names: ReadonlyArray<ImportName>;
      readonly from: string;
      readonly span: Span;
    }
  | {
      readonly tag: "type-decl";
      readonly name: string;
      readonly typeParams: ReadonlyArray<TypeParamNode>;
      readonly aliased: TypeNode;
      readonly exported: boolean;
      readonly span: Span;
    }
  | {
      readonly tag: "interface-decl";
      readonly name: string;
      readonly typeParams: ReadonlyArray<TypeParamNode>;
      readonly extends: ReadonlyArray<TypeNode>;
      readonly fields: ReadonlyArray<FieldNode>;
      readonly exported: boolean;
      readonly span: Span;
    }
  | {
      readonly tag: "var-decl";
      readonly kind: "const" | "let";
      readonly name: string;
      readonly declaredType: TypeNode | null;
      readonly init: Expr;
      readonly exported: boolean;
      readonly span: Span;
    }
  | {
      readonly tag: "function-decl";
      readonly name: string;
      readonly typeParams: ReadonlyArray<TypeParamNode>;
      readonly params: ReadonlyArray<Param>;
      readonly returnType: TypeNode | null;
      readonly body: Block;
      readonly exported: boolean;
      readonly span: Span;
    };

/** A whole source unit: zero or more declarations and statements in order. */
export type Program = ReadonlyArray<Decl | Stmt>;

// ---------------------------------------------------------------------
// Synthesizing constructors (for exercises and domain data)
// ---------------------------------------------------------------------

/** Numeric literal. */
export const num = (value: number, span: Span = noSpan): Expr => ({ tag: "number", value, span });
/** String literal. */
export const str = (value: string, span: Span = noSpan): Expr => ({ tag: "string", value, span });
/** Boolean literal. */
export const bool = (value: boolean, span: Span = noSpan): Expr => ({
  tag: "boolean",
  value,
  span,
});
/** Variable read. */
export const ident = (name: string, span: Span = noSpan): Expr => ({ tag: "variable", name, span });
/** Unary operation. */
export const un = (op: "!" | "+" | "-" | "typeof", operand: Expr, span: Span = noSpan): Expr => ({
  tag: "unary",
  op,
  operand,
  span,
});
/** Binary operation. */
export const bin = (
  op: "+" | "-" | "*" | "/" | "%" | "<" | "<=" | ">" | ">=" | "===" | "!==",
  left: Expr,
  right: Expr,
  span: Span = noSpan,
): Expr => ({ tag: "binary", op, left, right, span });
/** Short-circuiting boolean operation. */
export const logic = (op: "&&" | "||", left: Expr, right: Expr, span: Span = noSpan): Expr => ({
  tag: "logical",
  op,
  left,
  right,
  span,
});
/** Conditional expression. */
export const cond = (
  test: Expr,
  consequent: Expr,
  alternative: Expr,
  span: Span = noSpan,
): Expr => ({
  tag: "conditional",
  test,
  consequent,
  alternative,
  span,
});
/** Assignment. */
export const assign = (target: Expr, value: Expr, span: Span = noSpan): Expr => ({
  tag: "assign",
  target,
  value,
  span,
});
/** Call. */
export const call = (callee: Expr, args: ReadonlyArray<Expr>, span: Span = noSpan): Expr => ({
  tag: "call",
  callee,
  args: args.map((expr) => ({ kind: "item" as const, expr })),
  span,
});
/** Property read. */
export const member = (object: Expr, name: string, span: Span = noSpan): Expr => ({
  tag: "member",
  object,
  name,
  span,
});
/** Computed index read. */
export const at = (object: Expr, index: Expr, span: Span = noSpan): Expr => ({
  tag: "index",
  object,
  index,
  span,
});
/** Array literal. */
export const arrayLit = (elements: ReadonlyArray<Expr>, span: Span = noSpan): Expr => ({
  tag: "array",
  elements: elements.map((expr) => ({ kind: "item" as const, expr })),
  span,
});
/** Object literal with statically named fields. */
export const objectLit = (
  fields: ReadonlyArray<readonly [string, Expr]>,
  span: Span = noSpan,
): Expr => ({
  tag: "object",
  fields: fields.map(([key, value]) => ({ key, value, span: value.span })),
  span,
});
/** A required parameter. */
export const param = (name: string, type: TypeNode | null = null, span: Span = noSpan): Param => ({
  kind: "param",
  name,
  optional: false,
  type,
  span,
});
/** A block. */
export const block = (body: ReadonlyArray<Decl | Stmt>, span: Span = noSpan): Block => ({
  body,
  span,
});
/** Arrow function. */
export const lam = (
  params: ReadonlyArray<Param>,
  body: ReadonlyArray<Decl | Stmt>,
  span: Span = noSpan,
): Expr => ({
  tag: "arrow",
  params,
  body: block(body),
  span,
});
/** `if` statement. */
export const ifStmt = (
  test: Expr,
  consequent: Stmt,
  alternative: Stmt | null = null,
  span: Span = noSpan,
): Stmt => ({
  tag: "if",
  test,
  consequent,
  alternative,
  span,
});
/** `while` statement. */
export const whileStmt = (test: Expr, body: Stmt, span: Span = noSpan): Stmt => ({
  tag: "while",
  test,
  body,
  span,
});
/** `return` statement. */
export const returnStmt = (argument: Expr | null = null, span: Span = noSpan): Stmt => ({
  tag: "return",
  argument,
  span,
});
/** Expression statement. */
export const exprStmt = (expr: Expr, span: Span = noSpan): Stmt => ({
  tag: "expr-stmt",
  expr,
  span,
});
/** `const`/`let` declaration. */
export const varDecl = (
  kind: "const" | "let",
  name: string,
  init: Expr,
  declaredType: TypeNode | null = null,
  span: Span = noSpan,
): Decl => ({ tag: "var-decl", kind, name, declaredType, init, exported: false, span });
/** Function declaration. */
export const functionDecl = (
  name: string,
  params: ReadonlyArray<Param>,
  body: ReadonlyArray<Decl | Stmt>,
  returnType: TypeNode | null = null,
  span: Span = noSpan,
): Decl => ({
  tag: "function-decl",
  name,
  typeParams: [],
  params,
  returnType,
  body: block(body),
  exported: false,
  span,
});
