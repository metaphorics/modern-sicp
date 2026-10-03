// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: sections 4.1 to 5.5

/**
 * The subset checker and admission gate (host-subsets grammar sections 1, 2,
 * and 8). `checkProgram` imposes the subset's stricter rules — boolean-only
 * truthiness positions, compatible arithmetic operands, no evident
 * use-before-initialization, no `const` rebinding, no duplicate declarations,
 * no forbidden host primitives — and reads operand types from the pinned
 * TypeScript checker through the type oracle. `admitSource` runs the full
 * driver sequence before any guest effect: parse all source, reject
 * unsupported syntax, run subset checks, then the pinned `tsc` diagnostics;
 * a failure at any stage returns diagnostics and performs no program I/O or
 * mutation. The two error layers stay distinct: subset diagnostics carry
 * their category, host type failures preserve their TS code and span.
 */
import type { Block, Decl, Expr, Param, Program, Stmt, TypeNode } from "./ast.ts";
import { type Diagnostic, diagnostic, type Span } from "./diagnostics.ts";
import { type ExperimentMode, parseExperiment, parseProgram, ReadError } from "./parse.ts";
import {
  type HostDiagnostic,
  openTypeOracle,
  type TypeOracle,
  type TypeProbe,
} from "./typeOracle.ts";

export type { ExperimentMode, HostDiagnostic, TypeOracle };

/** The result of admitting one source unit. */
export type Admission =
  | { readonly ok: true; readonly program: Program }
  | {
      readonly ok: false;
      readonly diagnostics: ReadonlyArray<Diagnostic>;
      readonly hostDiagnostics: ReadonlyArray<HostDiagnostic>;
    };

type BindingKind = "const" | "let" | "function" | "param" | "catch" | "loop";

interface Frame {
  readonly names: Map<string, BindingKind>;
  readonly pending: Set<string>;
  readonly annotations: Map<string, string>;
  readonly parent: Frame | null;
}

const frameWith = (parent: Frame | null): Frame => ({
  names: new Map(),
  pending: new Set(),
  annotations: new Map(),
  parent,
});

const FORBIDDEN_CALLEES: Readonly<Record<string, string>> = {
  eval: "eval",
  Function: "Function-constructor",
  require: "require",
  fetch: "fetch",
  process: "process",
  globalThis: "globalThis",
  Date: "Date",
};

const FORBIDDEN_MEMBERS: Readonly<Record<string, Readonly<Record<string, string>>>> = {
  Math: { random: "Math.random" },
  Date: { now: "Date.now" },
  performance: { now: "performance.now" },
};

const NUMERIC_OPS: Readonly<Record<string, true>> = {
  "-": true,
  "*": true,
  "/": true,
  "%": true,
  "<": true,
  "<=": true,
  ">": true,
  ">=": true,
};

class Checker {
  readonly #diagnostics: Diagnostic[] = [];
  readonly #oracle: TypeOracle | undefined;
  readonly #readonlyFields = new Map<string, Set<string>>();

  constructor(oracle: TypeOracle | undefined) {
    this.#oracle = oracle;
  }

  run(program: Program): ReadonlyArray<Diagnostic> {
    this.#collectTypeFacts(program);
    let stillLeading = true;
    for (const item of program) {
      if (item.tag === "import") {
        if (!stillLeading) {
          this.#report(
            "UnsupportedSyntax",
            "leading-imports-only",
            item.span,
            "imports must lead the unit (the grammar is `Import* DeclarationOrStatement*`)",
          );
        }
      } else {
        stillLeading = false;
      }
    }
    this.#runScope(program, null);
    return this.#diagnostics;
  }

  #report(kind: Diagnostic["kind"], construct: string, span: Span, message: string): void {
    this.#diagnostics.push(diagnostic(kind, construct, span, message));
  }

  // ------------------------------------------------------------------
  // Type facts (locally declared interfaces and object type literals)
  // ------------------------------------------------------------------

  #collectTypeFacts(program: Program): void {
    for (const item of program) {
      this.#collectDeclFacts(item);
    }
  }

  #collectDeclFacts(item: Decl | Stmt): void {
    if (item.tag === "interface-decl") {
      this.#readonlyFields.set(item.name, readonlyNamesOf(item.fields));
      return;
    }
    if (item.tag !== "type-decl") {
      return;
    }
    const readonly = new Set<string>();
    for (const member of unionMembers(item.aliased)) {
      for (const field of member.fields) {
        if (field.readonly) {
          readonly.add(field.name);
        }
      }
    }
    this.#readonlyFields.set(item.name, readonly);
  }

  // ------------------------------------------------------------------
  // Scopes and statements
  // ------------------------------------------------------------------

  #runScope(items: ReadonlyArray<Decl | Stmt>, parent: Frame | null): void {
    const frame = frameWith(parent);
    for (const item of items) {
      this.#predeclare(item, frame);
    }
    for (const item of items) {
      this.#runItem(item, frame);
    }
  }

  #predeclare(item: Decl | Stmt, frame: Frame): void {
    if (item.tag === "var-decl") {
      this.#declare(frame, item.name, item.kind, item.span);
      this.#noteAnnotation(frame, item.name, item.declaredType);
      frame.pending.add(item.name);
      return;
    }
    if (item.tag === "function-decl") {
      this.#declare(frame, item.name, "function", item.span);
    }
  }

  #declare(frame: Frame, name: string, kind: BindingKind, span: Span): void {
    if (frame.names.has(name)) {
      this.#report(
        "DuplicateDeclaration",
        name,
        span,
        `\`${name}\` is already declared in this scope`,
      );
      return;
    }
    frame.names.set(name, kind);
  }

  #noteAnnotation(frame: Frame, name: string, declaredType: TypeNode | null): void {
    if (declaredType !== null && declaredType.tag === "type-reference") {
      frame.annotations.set(name, declaredType.name);
    }
  }

  #runItem(item: Decl | Stmt, frame: Frame): void {
    switch (item.tag) {
      case "var-decl": {
        this.#expr(item.init, frame);
        this.#typeNode(item.declaredType);
        frame.pending.delete(item.name);
        return;
      }
      case "function-decl": {
        this.#typeNode(item.returnType);
        this.#runFunction(item.params, item.body, frame);
        return;
      }
      case "type-decl": {
        this.#typeNode(item.aliased);
        return;
      }
      case "interface-decl": {
        for (const field of item.fields) {
          this.#typeNode(field.type);
        }
        return;
      }
      case "import": {
        if (item.from.startsWith("node:")) {
          this.#report(
            "ForbiddenHostPrimitive",
            "node-module-import",
            item.span,
            "node modules are boundary-only",
          );
        }
        return;
      }
      default:
        this.#runStatement(item, frame);
    }
  }

  #runFunction(params: ReadonlyArray<Param>, body: Block, parent: Frame): void {
    const frame = frameWith(parent);
    for (const entry of params) {
      this.#typeNode(entry.type);
      this.#declare(frame, entry.name, "param", entry.span);
      this.#noteAnnotation(frame, entry.name, entry.type);
    }
    for (const item of body.body) {
      this.#predeclare(item, frame);
    }
    for (const item of body.body) {
      this.#runItem(item, frame);
    }
  }

  #runStatement(stmt: Stmt, frame: Frame): void {
    switch (stmt.tag) {
      case "block":
        this.#runScope(stmt.body, frame);
        return;
      case "if": {
        this.#condition(stmt.test, frame);
        this.#runStatement(stmt.consequent, frame);
        if (stmt.alternative !== null) {
          this.#runStatement(stmt.alternative, frame);
        }
        return;
      }
      case "while": {
        this.#condition(stmt.test, frame);
        this.#runStatement(stmt.body, frame);
        return;
      }
      case "for-of": {
        this.#expr(stmt.iterable, frame);
        const loop = frameWith(frame);
        loop.names.set(stmt.name, "loop");
        this.#runStatement(stmt.body, loop);
        return;
      }
      case "switch": {
        this.#expr(stmt.discriminant, frame);
        const items: Array<Decl | Stmt> = [];
        for (const clause of stmt.cases) {
          this.#expr(clause.test, frame);
          items.push(...clause.body);
        }
        if (stmt.defaultBody !== null) {
          items.push(...stmt.defaultBody);
        }
        this.#runScope(items, frame);
        return;
      }
      case "return":
        if (stmt.argument !== null) {
          this.#expr(stmt.argument, frame);
        }
        return;
      case "throw":
        this.#expr(stmt.argument, frame);
        return;
      case "try": {
        this.#runScope(stmt.block.body, frame);
        if (stmt.handler !== null) {
          const catchFrame = frameWith(frame);
          if (stmt.handler.param !== null) {
            catchFrame.names.set(stmt.handler.param, "catch");
          }
          for (const item of stmt.handler.body.body) {
            this.#predeclare(item, catchFrame);
          }
          for (const item of stmt.handler.body.body) {
            this.#runItem(item, catchFrame);
          }
        }
        if (stmt.finalizer !== null) {
          this.#runScope(stmt.finalizer.body, frame);
        }
        return;
      }
      case "expr-stmt":
        this.#expr(stmt.expr, frame);
        return;
      case "break":
      case "continue":
        return;
    }
  }

  // ------------------------------------------------------------------
  // Expressions
  // ------------------------------------------------------------------

  #condition(test: Expr, frame: Frame): void {
    this.#expr(test, frame);
    const probe = this.#probe(test.span);
    if (probe === undefined || probe.kind === "boolean") {
      return;
    }
    if (probe.kind === "any") {
      this.#report("UnsupportedType", "any-type", test.span, "condition operand has type `any`");
      return;
    }
    if (probe.kind === "error" || probe.kind === "unresolved") {
      return;
    }
    this.#report(
      "ExpectedBoolean",
      "condition",
      test.span,
      `condition operand is \`${probe.text}\`, not boolean`,
    );
  }

  #numericOperand(operand: Expr, operator: string, frame: Frame): void {
    this.#expr(operand, frame);
    const probe = this.#probe(operand.span);
    if (probe === undefined || probe.kind === "number") {
      return;
    }
    if (probe.kind === "any") {
      this.#report(
        "UnsupportedType",
        "any-type",
        operand.span,
        "arithmetic operand has type `any`",
      );
      return;
    }
    if (probe.kind === "error" || probe.kind === "unresolved") {
      return;
    }
    this.#report(
      "ExpectedNumber",
      operator,
      operand.span,
      `\`${operator}\` operand is \`${probe.text}\`, not a number`,
    );
  }

  #plusOperands(left: Expr, right: Expr, frame: Frame): void {
    this.#expr(left, frame);
    this.#expr(right, frame);
    const leftProbe = this.#probe(left.span);
    const rightProbe = this.#probe(right.span);
    if (leftProbe === undefined || rightProbe === undefined) {
      return;
    }
    const unresolved =
      leftProbe.kind === "error" ||
      rightProbe.kind === "error" ||
      leftProbe.kind === "unresolved" ||
      rightProbe.kind === "unresolved";
    if (unresolved) {
      return;
    }
    if (leftProbe.kind === "any" || rightProbe.kind === "any") {
      this.#report("UnsupportedType", "any-type", left.span, "arithmetic operand has type `any`");
      return;
    }
    const bothNumbers = leftProbe.kind === "number" && rightProbe.kind === "number";
    const bothStrings = leftProbe.kind === "string" && rightProbe.kind === "string";
    if (bothNumbers || bothStrings) {
      return;
    }
    const mixed =
      (leftProbe.kind === "number" || leftProbe.kind === "string") &&
      (rightProbe.kind === "number" || rightProbe.kind === "string");
    const construct = mixed ? "mixed-plus-operands" : "plus-operands";
    const kind: Diagnostic["kind"] = mixed ? "UnsupportedType" : "ExpectedNumber";
    this.#report(
      kind,
      construct,
      left.span,
      `\`+\` operands are \`${leftProbe.text}\` and \`${rightProbe.text}\``,
    );
  }

  #probe(span: Span): TypeProbe | undefined {
    return this.#oracle?.probe(span);
  }

  #expr(expr: Expr, frame: Frame): void {
    switch (expr.tag) {
      case "number":
      case "string":
      case "boolean":
      case "null":
      case "undefined":
        return;
      case "template": {
        for (const inner of expr.exprs) {
          this.#expr(inner, frame);
        }
        return;
      }
      case "variable": {
        if (frame.pending.has(expr.name)) {
          this.#report(
            "UseBeforeInitialization",
            expr.name,
            expr.span,
            `\`${expr.name}\` is read before its initialization`,
          );
        }
        return;
      }
      case "array": {
        for (const element of expr.elements) {
          this.#expr(element.expr, frame);
        }
        return;
      }
      case "object": {
        for (const field of expr.fields) {
          this.#expr(field.value, frame);
        }
        return;
      }
      case "unary": {
        if (expr.op === "!" || expr.op === "+" || expr.op === "-") {
          if (expr.op === "!") {
            this.#condition(expr.operand, frame);
          } else {
            this.#numericOperand(expr.operand, expr.op, frame);
          }
          return;
        }
        this.#expr(expr.operand, frame);
        return;
      }
      case "binary": {
        if (expr.op === "+") {
          this.#plusOperands(expr.left, expr.right, frame);
          return;
        }
        if (NUMERIC_OPS[expr.op] === true) {
          this.#numericOperand(expr.left, expr.op, frame);
          this.#numericOperand(expr.right, expr.op, frame);
          return;
        }
        this.#expr(expr.left, frame);
        this.#expr(expr.right, frame);
        return;
      }
      case "logical": {
        this.#condition(expr.left, frame);
        this.#condition(expr.right, frame);
        return;
      }
      case "conditional": {
        this.#condition(expr.test, frame);
        this.#expr(expr.consequent, frame);
        this.#expr(expr.alternative, frame);
        return;
      }
      case "assign": {
        this.#assignTarget(expr.target, frame);
        this.#expr(expr.value, frame);
        return;
      }
      case "permanent-assign": {
        this.#assignTarget(expr.target, frame);
        this.#expr(expr.value, frame);
        return;
      }
      case "if-fail": {
        this.#expr(expr.expression, frame);
        this.#expr(expr.fallback, frame);
        return;
      }
      case "arrow": {
        this.#runFunction(expr.params, expr.body, frame);
        return;
      }
      case "call": {
        this.#checkCall(expr.callee, expr.args.length, expr.span, frame);
        if (expr.callee.tag === "member") {
          this.#expr(expr.callee.object, frame);
        } else {
          this.#expr(expr.callee, frame);
        }
        for (const arg of expr.args) {
          this.#expr(arg.expr, frame);
        }
        return;
      }
      case "member": {
        this.#checkMember(expr.object, expr.name, expr.span, frame);
        this.#expr(expr.object, frame);
        return;
      }
      case "index": {
        this.#expr(expr.object, frame);
        this.#expr(expr.index, frame);
        return;
      }
      case "new-error": {
        for (const arg of expr.args) {
          this.#expr(arg, frame);
        }
        return;
      }
      case "new-map":
      case "new-set": {
        for (const arg of expr.args) {
          this.#expr(arg, frame);
        }
        return;
      }
      case "delay":
      case "force": {
        this.#expr(expr.expr, frame);
        return;
      }
      case "require": {
        this.#condition(expr.condition, frame);
        return;
      }
      case "choose":
      case "ramb": {
        for (const alternative of expr.alternatives) {
          this.#expr(alternative, frame);
        }
        return;
      }
    }
  }

  #assignTarget(target: Expr, frame: Frame): void {
    if (target.tag === "variable") {
      const kind = lookupKind(frame, target.name);
      if (kind === "const") {
        this.#report(
          "ReassignConst",
          target.name,
          target.span,
          `\`${target.name}\` is a const binding`,
        );
      }
      return;
    }
    if (target.tag === "member") {
      this.#checkMemberWrite(target.object, target.name, target.span, frame);
      this.#expr(target.object, frame);
      return;
    }
    if (target.tag === "index") {
      this.#expr(target.object, frame);
      this.#expr(target.index, frame);
      return;
    }
    this.#report(
      "UnsupportedSyntax",
      "assignment-target",
      target.span,
      "the assignment target must be a name or property",
    );
  }

  #checkMemberWrite(object: Expr, name: string, span: Span, frame: Frame): void {
    if (object.tag !== "variable") {
      return;
    }
    const typeName = lookupAnnotation(frame, object.name);
    const readonly = typeName === undefined ? undefined : this.#readonlyFields.get(typeName);
    if (readonly?.has(name) === true) {
      this.#report("ReadOnlyField", name, span, `\`${name}\` is a readonly field`);
    }
  }

  #checkCall(callee: Expr, argCount: number, span: Span, frame: Frame): void {
    if (callee.tag === "variable") {
      const forbidden = FORBIDDEN_CALLEES[callee.name];
      if (forbidden !== undefined && lookupKind(frame, callee.name) === undefined) {
        this.#report(
          "ForbiddenHostPrimitive",
          forbidden,
          span,
          `\`${callee.name}\` is not a guest primitive`,
        );
      }
      return;
    }
    if (callee.tag !== "member") {
      return;
    }
    if (
      callee.object.tag === "variable" &&
      callee.object.name === "console" &&
      lookupKind(frame, "console") === undefined
    ) {
      if (callee.name !== "log" || argCount !== 1) {
        this.#report(
          "ForbiddenHostPrimitive",
          "console",
          span,
          "only one-argument `console.log` is admitted",
        );
      }
      return;
    }
    this.#checkMember(callee.object, callee.name, span, frame);
  }

  #checkMember(object: Expr, name: string, span: Span, frame: Frame): void {
    if (object.tag !== "variable" || lookupKind(frame, object.name) !== undefined) {
      return;
    }
    const forbiddenFor = FORBIDDEN_MEMBERS[object.name];
    const forbidden = forbiddenFor?.[name];
    if (forbidden !== undefined) {
      this.#report(
        "ForbiddenHostPrimitive",
        forbidden,
        span,
        `\`${object.name}.${name}\` is not a guest primitive`,
      );
      return;
    }
    if (object.name === "console") {
      this.#report(
        "ForbiddenHostPrimitive",
        "console",
        span,
        "only one-argument `console.log` is admitted",
      );
    }
  }

  #typeNode(node: TypeNode | null): void {
    if (node === null) {
      return;
    }
    switch (node.tag) {
      case "primitive-type":
        return;
      case "literal-type":
        return;
      case "type-reference": {
        if (node.name === "any") {
          this.#report("UnsupportedType", "any-type", node.span, "explicit `any` is excluded");
        }
        for (const arg of node.args) {
          this.#typeNode(arg);
        }
        return;
      }
      case "array-type":
        this.#typeNode(node.element);
        return;
      case "tuple-type": {
        for (const element of node.elements) {
          this.#typeNode(element);
        }
        return;
      }
      case "object-type": {
        for (const field of node.fields) {
          this.#typeNode(field.type);
        }
        return;
      }
      case "function-type": {
        for (const param of node.params) {
          this.#typeNode(param.kind === "rest" ? param.type : param.type);
        }
        this.#typeNode(node.result);
        return;
      }
      case "union-type": {
        for (const member of node.members) {
          this.#typeNode(member);
        }
        return;
      }
    }
  }
}

const lookupKind = (frame: Frame, name: string): BindingKind | undefined => {
  let current: Frame | null = frame;
  while (current !== null) {
    const kind = current.names.get(name);
    if (kind !== undefined) {
      return kind;
    }
    current = current.parent;
  }
  return undefined;
};

const lookupAnnotation = (frame: Frame, name: string): string | undefined => {
  let current: Frame | null = frame;
  while (current !== null) {
    if (current.names.has(name)) {
      return current.annotations.get(name);
    }
    current = current.parent;
  }
  return undefined;
};

const readonlyNamesOf = (
  fields: ReadonlyArray<{ readonly name: string; readonly readonly: boolean }>,
): Set<string> => {
  const names = new Set<string>();
  for (const field of fields) {
    if (field.readonly) {
      names.add(field.name);
    }
  }
  return names;
};

const unionMembers = (
  type: TypeNode,
): ReadonlyArray<{
  readonly fields: ReadonlyArray<{ readonly name: string; readonly readonly: boolean }>;
}> =>
  type.tag === "union-type"
    ? type.members.flatMap((member) => (member.tag === "object-type" ? [member] : []))
    : type.tag === "object-type"
      ? [type]
      : [];

/** Runs the subset's syntactic rules (and type rules when `oracle` is given). */
export const checkProgram = (program: Program, oracle?: TypeOracle): ReadonlyArray<Diagnostic> =>
  new Checker(oracle).run(program);

const parseInMode = (text: string, mode: ExperimentMode): Program =>
  mode === "core" ? parseProgram(text) : parseExperiment(text, mode);

/** Boundary-only type declarations for the named experiment extensions. */
const experimentDeclarations = (mode: ExperimentMode): string => {
  if (mode === "lazy-memoized-experiment" || mode === "lazy-recompute-experiment") {
    return "declare function delay<T>(value: T): T;\ndeclare function force<T>(value: T): T;";
  }
  if (mode === "amb-depth-first-experiment" || mode === "amb-ramb-experiment") {
    return "declare function choose<T>(...alternatives: T[]): T;\ndeclare function ramb<T>(...alternatives: T[]): T;\ndeclare function require(condition: boolean): void;\ndeclare function permanentAssign<T>(target: T, value: NoInfer<T>): void;\ndeclare function ifFail<T, U>(expression: T, fallback: U): T | U;";
  }
  return "";
};

/** Admits one source unit: parse, subset checks, pinned host type gate. */
export const admitSource = (text: string, mode: ExperimentMode = "core"): Admission => {
  let program: Program;
  try {
    program = parseInMode(text, mode);
  } catch (error) {
    if (error instanceof ReadError) {
      return { ok: false, diagnostics: [error.diagnostic], hostDiagnostics: [] };
    }
    throw error;
  }
  const oracle = openTypeOracle(text, experimentDeclarations(mode));
  const diagnostics = checkProgram(program, oracle);
  const hostDiagnostics = oracle.hostDiagnostics();
  if (diagnostics.length > 0 || hostDiagnostics.length > 0) {
    return { ok: false, diagnostics, hostDiagnostics };
  }
  return { ok: true, program };
};
