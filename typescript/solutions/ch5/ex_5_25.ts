// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Arg, Operation } from "../../packages/ch5/src/02-simulator.js";
import {
  assign,
  branch,
  type ControllerLine,
  jump,
  jumpReg,
  lbl,
  mark,
  op,
  perform,
  reg,
  restore,
  save,
  test,
} from "../../packages/ch5/src/02-simulator.js";
import {
  evaluatorController,
  isTaggedWord,
  makeListWord,
  makeTaggedWord,
  makeVariantEvaluator,
  nil,
  type OperationsSpec,
  replaceSegment,
  type State,
  type Word,
  wordItems,
  wordOperation1,
  wordOperation2,
} from "../../packages/ch5/src/04-eceval.js";

interface ThunkPayload {
  expression: Word;
  environment: Word;
  forced?: boolean;
  value?: Word;
}

const thunkPayload = (w: Word): ThunkPayload => {
  if (!isTaggedWord(w, "thunk")) throw new Error("expected a thunk word");
  return w.payload as ThunkPayload;
};
const isUnforcedThunk = (w: Word): boolean =>
  isTaggedWord(w, "thunk") && thunkPayload(w).forced !== true;

// The lazy machine changes three regions of the base controller. The
// argument loop thunks every operand without evaluating it, which is
// the laziness and needs no register saves. The variable path forces a
// thunked binding the first time the variable is read and memoizes by
// storing the forced value over the binding with set-variable-value!.
// primitive-apply forces any thunks left in argl before the primitive
// sees them, one nested evaluation per pass, memoizing into the thunk
// word in place.
const lazyArgumentLoop: ControllerLine[] = [
  save("continue"),
  save("env"),
  assign("unev", op("operands", reg("exp"))),
  save("unev"),
  assign("exp", op("operator", reg("exp"))),
  assign("continue", lbl("ev-appl-did-operator")),
  jump("eval-dispatch"),
  mark("ev-appl-did-operator"),
  restore("unev"),
  restore("env"),
  assign("argl", op("empty-arglist")),
  assign("proc", reg("val")),
  test("no-operands?", reg("unev")),
  branch("apply-dispatch"),
  save("proc"),
  mark("ev-appl-operand-loop"),
  test("no-operands?", reg("unev")),
  branch("ev-appl-thunks-done"),
  assign("exp", op("first-operand", reg("unev"))),
  // The thunk construction rides inside adjoin-arg as a nested operation
  // call; the assembler resolves nested sources, the Arg type cannot
  // spell one.
  assign("argl", op("adjoin-arg", op("make-thunk", reg("exp"), reg("env")) as Arg, reg("argl"))),
  assign("unev", op("rest-operands", reg("unev"))),
  jump("ev-appl-operand-loop"),
  mark("ev-appl-thunks-done"),
  restore("proc"),
];

const lazyVariable: ControllerLine[] = [
  assign("val", op("lookup-variable-value", reg("exp"), reg("env"))),
  test("thunk?", reg("val")),
  branch("ev-variable-force"),
  jumpReg("continue"),
  mark("ev-variable-force"),
  save("exp"),
  save("env"),
  save("continue"),
  assign("exp", op("thunk-expression", reg("val"))),
  assign("env", op("thunk-environment", reg("val"))),
  assign("continue", lbl("ev-variable-after-force")),
  jump("eval-dispatch"),
  mark("ev-variable-after-force"),
  restore("continue"),
  restore("env"),
  restore("exp"),
  perform("set-variable-value!", reg("exp"), reg("val"), reg("env")),
  jumpReg("continue"),
];

const lazyPrimitiveApply: ControllerLine[] = [
  test("thunked-args?", reg("argl")),
  branch("primitive-apply-force"),
  restore("continue"),
  assign("val", op("apply-primitive-procedure", reg("proc"), reg("argl"))),
  jumpReg("continue"),
  mark("primitive-apply-force"),
  save("proc"),
  save("argl"),
  save("unev"),
  save("continue"),
  assign("unev", op("first-unforced-thunk", reg("argl"))),
  assign("exp", op("thunk-expression", reg("unev"))),
  assign("env", op("thunk-environment", reg("unev"))),
  assign("continue", lbl("primitive-apply-forced")),
  jump("eval-dispatch"),
  mark("primitive-apply-forced"),
  restore("continue"),
  restore("unev"),
  restore("argl"),
  restore("proc"),
  perform("memoize-thunk-in-argl", reg("argl"), reg("val")),
  jump("primitive-apply"),
];

export const lazyController: readonly ControllerLine[] = replaceSegment(
  replaceSegment(
    replaceSegment(evaluatorController, "ev-application", "apply-dispatch", lazyArgumentLoop),
    "ev-variable",
    "ev-quoted",
    lazyVariable,
  ),
  "primitive-apply",
  "compound-apply",
  lazyPrimitiveApply,
);

export const makeLazyOperations = (
  _state: State,
  base: Record<string, Operation>,
): Record<string, Operation> => {
  const baseApply = (() => {
    const entry = base["apply-primitive-procedure"];
    if (!entry) throw new Error("the base table lacks apply-primitive-procedure");
    return entry;
  })();
  return {
    "make-thunk": wordOperation2("make-thunk", (expression, environment) =>
      makeTaggedWord("thunk", { expression, environment }),
    ),
    "thunk?": wordOperation1("thunk?", isUnforcedThunk),
    "thunk-expression": wordOperation1("thunk-expression", (w) => thunkPayload(w).expression),
    "thunk-environment": wordOperation1("thunk-environment", (w) => thunkPayload(w).environment),
    "thunked-args?": wordOperation1("thunked-args?", (w) => wordItems(w).some(isUnforcedThunk)),
    "first-unforced-thunk": wordOperation1("first-unforced-thunk", (w) => {
      const found = wordItems(w).find(isUnforcedThunk);
      if (found === undefined) throw new Error("the argument list has no unforced thunk");
      return found;
    }),
    "memoize-thunk-in-argl": wordOperation2("memoize-thunk-in-argl", (argl, value) => {
      // val answers the first unforced thunk; when a previous pass or
      // the variable path already cleaned argl, there is nothing to
      // memoize.
      const thunk = wordItems(argl).find(isUnforcedThunk);
      if (thunk !== undefined) {
        const payload = thunkPayload(thunk);
        payload.forced = true;
        payload.value = value;
      }
      return nil;
    }),
    // The primitive sees values, so each forced thunk unwraps before the
    // base applicability checks run.
    "apply-primitive-procedure": wordOperation2(
      "apply-primitive-procedure",
      (proc, args) =>
        baseApply([
          proc,
          makeListWord(
            wordItems(args).map((w) => {
              const payload = thunkPayload(w);
              return payload.forced === true ? (payload.value as Word) : w;
            }),
          ),
        ]) as Word,
    ),
  };
};

export const lazyOperations: OperationsSpec = makeLazyOperations;

export const runLazy = (source: string): readonly string[] =>
  makeVariantEvaluator(source, lazyController, lazyOperations).run();

// The chapter 1.5 test: strict evaluation of (p) diverges; normal
// order never touches it because the predicate selects x = 0.
const testOrderProgram = `
(define (p) (p))
(define (test-order x y) (if (= x 0) 0 y))
(test-order 0 (p))
`;

// The unused argument: strict evaluation dies on (car (quote ()));
// the lazy evaluator never forces it.
const unusedArgumentProgram = `
(define (always-42 x) 42)
(always-42 (car (quote ())))
`;

// The memoization probe: the thunked (bump) must run once, so both
// references of x see the same 1 and count stays 1.
const memoizationProgram = `
(define count 0)
(define (bump) (set! count (+ count 1)) count)
(define (use-twice x) (cons x x))
(use-twice (bump))
count
`;

const factorialProgram = `
(define (factorial n)
  (if (= n 1) 1 (* (factorial (- n 1)) n)))
(factorial 5)
`;

const valuesOf = (transcript: readonly string[]): string[] => {
  const values: string[] = [];
  for (let i = 0; i < transcript.length; i += 1) {
    if (transcript[i] === ";;; EC-Eval value:") values.push(transcript[i + 1] as string);
  }
  return values;
};

export const lazyTestOrderValue = (): string =>
  valuesOf(runLazy(testOrderProgram)).at(-1) as string;
export const lazyUnusedArgumentValue = (): string =>
  valuesOf(runLazy(unusedArgumentProgram)).at(-1) as string;
export const lazyMemoizationValues = (): readonly string[] => valuesOf(runLazy(memoizationProgram));
export const lazyFactorialValue = (): string =>
  valuesOf(runLazy(factorialProgram)).at(-1) as string;
export const ex_5_25 = lazyTestOrderValue;
