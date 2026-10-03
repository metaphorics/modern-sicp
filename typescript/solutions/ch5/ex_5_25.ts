// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Block, Expr, Stmt } from "../../packages/ch4/src/syntax/ast.ts";
import type { Span } from "../../packages/ch4/src/syntax/diagnostics.ts";
import {
  assign,
  branch,
  constant,
  evaluatorController as evaluatorControllerOf525,
  gotoLabel,
  type MachineStatement,
  makeEvaluator,
  type Operation,
  op,
  perform,
  register,
  replaceSegment,
  restore,
  save,
  test,
  type Word,
} from "../../packages/ch5/src/04-eceval.ts";
import { mark } from "./ex_5_07.ts";

type EvaluatorStatement = MachineStatement<Word>;

const span: Span = { start: 0, end: 0, line: 1, column: 1 };

const EXPRESSION_TAGS: ReadonlySet<string> = new Set([
  "number",
  "string",
  "boolean",
  "null",
  "undefined",
  "template",
  "variable",
  "array",
  "object",
  "unary",
  "binary",
  "logical",
  "conditional",
  "assign",
  "arrow",
  "call",
  "member",
  "index",
  "new-error",
  "new-map",
  "new-set",
]);

/** Narrow one machine word to the expression nodes of the shared AST. */
const isExprWord = (word: Word): word is Expr =>
  typeof word === "object" && word !== null && "tag" in word && EXPRESSION_TAGS.has(word.tag);

/** Exercise 5.25: the normal-order evaluator. The operand loop builds
 * thunks instead of evaluating operands, and a variable read forces the
 * thunk and remembers the value, so an argument is computed at most
 * once however often it is used. The thunk is a marked zero-parameter
 * procedure over the operand expression, so it travels the machine's
 * own apply path; the marks and the memo live in the experiment's
 * operations, never in the evaluator kernel. */
export const makeNormalOrderOperations = (): {
  operations: Readonly<Record<string, Operation<Word>>>;
  forced: () => number;
} => {
  const thunks = new WeakSet<object>();
  const memo = new WeakMap<object, Word>();
  const pendingForces: Array<{ readonly thunk: object; readonly continuation: Word }> = [];
  let forceCount = 0;
  return {
    forced: () => forceCount,
    operations: {
      thunkArrow: (args) => {
        const expression = isExprWord(args[0]) ? args[0] : null;
        const returnStmt: Stmt = { tag: "return", argument: expression, span };
        const body: Block = { body: [returnStmt], span };
        return { tag: "arrow", params: [], body, span };
      },
      markThunk: (args) => {
        const value = args[0];
        if (typeof value === "object" && value !== null) {
          thunks.add(value);
        }
        return value;
      },
      isThunk: (args) => {
        const value = args[0];
        return typeof value === "object" && value !== null && thunks.has(value);
      },
      hasForced: (args) => {
        const value = args[0];
        return typeof value === "object" && value !== null && memo.has(value);
      },
      forcedValue: (args) => {
        const value = args[0];
        return typeof value === "object" && value !== null ? memo.get(value) : undefined;
      },
      beginForce: (args) => {
        const thunk = args[0];
        if (typeof thunk !== "object" || thunk === null) return undefined;
        pendingForces.push({ thunk, continuation: args[1] });
        forceCount += 1;
        return undefined;
      },
      cachePending: (args) => {
        const pending = pendingForces[pendingForces.length - 1];
        if (pending === undefined) return undefined;
        const value = args[0];
        memo.set(pending.thunk, value);
        return value;
      },
      restoreForceContinuation: () => {
        const pending = pendingForces.pop();
        if (pending === undefined)
          throw new Error("normal-order force returned without a pending thunk");
        return pending.continuation;
      },
    },
  };
};

/** The normal-order operand loop: each operand becomes a thunk over the
 * operand expression and the current environment; nothing is evaluated
 * before the call. */
const thunkOperandLoop = (): readonly EvaluatorStatement[] => [
  mark("ev-operand-loop"),
  test("noOperands", register("unev")),
  branch("continue-dispatch"),
  assign("val", op("thunkArrow", op("firstOperand", register("unev")))),
  assign(
    "val",
    op(
      "makeProcedure",
      op("lambdaParams", register("val")),
      op("lambdaBody", register("val")),
      register("env"),
    ),
  ),
  perform("markThunk", register("val")),
  assign("argl", op("adjoinArg", register("argl"), register("unev"), register("val"))),
  assign("unev", op("restOperands", register("unev"))),
  gotoLabel("ev-operand-loop"),
];

/** The force path spliced into variable lookup: a thunk is applied once
 * and its value cached on the thunk itself. */
const forceSegment = (): readonly EvaluatorStatement[] => [
  mark("ef-force-thunk"),
  test("hasForced", register("val")),
  branch("ef-forced-cache"),
  save("item"),
  save("proc"),
  save("argl"),
  perform("beginForce", register("val"), register("continue")),
  assign("proc", register("val")),
  assign("argl", op("emptyArgList")),
  assign("continue", constant({ tag: "symbol", name: "ef-forced" })),
  save("continue"),
  gotoLabel("apply-dispatch"),
  mark("ef-forced-cache"),
  assign("val", op("forcedValue", register("val"))),
  gotoLabel("continue-dispatch"),
  mark("ef-forced"),
  perform("cachePending", register("val")),
  assign("continue", op("restoreForceContinuation")),
  gotoLabel("continue-dispatch"),
];

/** The normal-order controller: the operand loop is replaced by the
 * thunking loop, and the variable path grows the force test. */
const normalOrderStrictOutputLoop = (): readonly EvaluatorStatement[] => [
  mark("normal-order-output-loop"),
  test("noOperands", register("unev")),
  branch("continue-dispatch"),
  save("unev"),
  save("continue"),
  assign("expr", op("firstOperand", register("unev"))),
  assign("continue", constant({ tag: "symbol", name: "normal-order-output-next" })),
  gotoLabel("eval-form"),
  mark("normal-order-output-next"),
  restore("continue"),
  restore("unev"),
  test("isTransfer", register("transfer")),
  branch("continue-dispatch"),
  test("isErrorValue", register("val")),
  branch("raise-error"),
  assign("argl", op("adjoinArg", register("argl"), register("unev"), register("val"))),
  assign("unev", op("restOperands", register("unev"))),
  gotoLabel("normal-order-output-loop"),
];

const normalOrderBaseController: readonly EvaluatorStatement[] = replaceSegment(
  replaceSegment(
    replaceSegment(evaluatorControllerOf525, "ev-operand-loop", "ev-return", thunkOperandLoop()),
    "ef-variable",
    "ef-arrow",
    [
      mark("ef-variable"),
      assign(
        "val",
        op("lookupVariableValue", op("variableName", register("expr")), register("env")),
      ),
      test("isErrorValue", register("val")),
      branch("raise-error"),
      test("isThunk", register("val")),
      branch("ef-force-thunk"),
      gotoLabel("continue-dispatch"),
    ],
  ),
  "ev-output",
  "ev-output-done",
  [
    mark("ev-output"),
    save("item"),
    save("argl"),
    save("continue"),
    assign("item", register("expr")),
    assign("argl", op("emptyArgList")),
    assign("unev", op("argExprs", register("item"))),
    assign("continue", constant({ tag: "symbol", name: "ev-output-done" })),
    gotoLabel("normal-order-output-loop"),
  ],
);

export const normalOrderController: readonly EvaluatorStatement[] = replaceSegment(
  normalOrderBaseController,
  "done",
  "done",
  [...normalOrderStrictOutputLoop(), ...forceSegment()],
);

const runNormal = (program: string): readonly string[] => {
  const { operations } = makeNormalOrderOperations();
  const result = makeEvaluator(program, operations, normalOrderController).run();
  return result.transcript;
};

/** The chapter 1.5 test: strict evaluation of `p()` diverges; normal
 * order never touches it because the predicate selects x = 0. */
export const ex_5_25 = (): {
  readonly normal: readonly string[];
  readonly strictFault: string | null;
} => {
  const testOrder = [
    "function p(): number { return p(); }",
    "function testOrder(x: number, y: number) { return x === 0 ? 0 : y; }",
    "console.log(testOrder(0, p()));",
  ].join("\n");
  const unused = [
    "function always42(x: number) { return 42; }",
    "console.log(always42(nope()));",
  ].join("\n");
  const memoization = [
    "let count = 0;",
    "function bump() { count = count + 1; return count; }",
    "function useTwice(x: number) { return x + x; }",
    "console.log(useTwice(bump()));",
    "console.log(count);",
  ].join("\n");
  const factorial = [
    "function factorial(n: number): number { return n === 1 ? 1 : factorial(n - 1) * n; }",
    "console.log(factorial(5));",
  ].join("\n");
  const strict = makeEvaluator(testOrder).run();
  return {
    normal: [
      ...runNormal(testOrder),
      ...runNormal(unused),
      ...runNormal(memoization),
      ...runNormal(factorial),
    ],
    strictFault: strict.outcome.tag === "ok" ? null : JSON.stringify(strict.outcome.error),
  };
};
