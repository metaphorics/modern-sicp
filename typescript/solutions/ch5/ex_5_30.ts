// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  assign,
  branch,
  type ControllerLine,
  jump,
  jumpReg,
  mark,
  op,
  reg,
  restore,
  test,
} from "../../packages/ch5/src/02-simulator.js";
import {
  EvaluatorFault,
  evaluatorController,
  isTaggedWord,
  makeConditionWord,
  makeVariantEvaluator,
  type OperationsSpec,
  replaceSegment,
  type State,
  type Word,
  wordName,
  wordOperation1,
  wordOperation2,
} from "../../packages/ch5/src/04-eceval.js";

// (a) The lookup operation answers a distinguished condition code for
// an unbound variable, a word no user value can spell, and ev-variable
// tests for it before using val.
const checkingVariable: ControllerLine[] = [
  assign("val", op("lookup-variable-value", reg("exp"), reg("env"))),
  test("condition?", reg("val")),
  branch("signal-error"),
  jumpReg("continue"),
];

// (b) The primitive application answers a condition code when an
// applicability check fails, and primitive-apply tests for it, cleans
// the stack the way unknown-procedure-type does, and goes to
// signal-error.
const checkingPrimitiveApply: ControllerLine[] = [
  assign("val", op("apply-primitive-procedure", reg("proc"), reg("argl"))),
  test("condition?", reg("val")),
  branch("primitive-apply-condition"),
  restore("continue"),
  jumpReg("continue"),
  mark("primitive-apply-condition"),
  restore("continue"),
  jump("signal-error"),
];

export const checkingController: readonly ControllerLine[] = replaceSegment(
  replaceSegment(evaluatorController, "ev-variable", "ev-quoted", checkingVariable),
  "primitive-apply",
  "compound-apply",
  checkingPrimitiveApply,
);

const lookupOrCondition = (state: State, name: Word, env: Word): Word => {
  const variable = wordName(name);
  if (!isTaggedWord(env, "environment")) throw new EvaluatorFault("expected an environment word");
  let index = env.payload as number;
  while (true) {
    const frame = state.frames[index];
    if (!frame) break;
    const found = frame.bindings.get(variable);
    if (found !== undefined) return found;
    if (frame.parent === null) break;
    index = frame.parent;
  }
  return makeConditionWord("error", `unbound variable: ${variable}`);
};

export const checkingOperations: OperationsSpec = (state, base) => ({
  "condition?": wordOperation1("condition?", (w) => isTaggedWord(w, "condition")),
  "lookup-variable-value": wordOperation2("lookup-variable-value", (v, e) =>
    lookupOrCondition(state, v, e),
  ),
  // Each primitive keeps its applicability checks; a failed check
  // surfaces here as the condition code primitive-apply tests.
  "apply-primitive-procedure": wordOperation2("apply-primitive-procedure", (proc, args) => {
    const baseApply = base["apply-primitive-procedure"];
    if (!baseApply) throw new EvaluatorFault("the base table lacks apply-primitive-procedure");
    try {
      return baseApply([proc, args]) as Word;
    } catch (error) {
      if (error instanceof EvaluatorFault) return makeConditionWord("error", error.message);
      throw error;
    }
  }),
});

export const runCheckingEvaluator = (source: string): readonly string[] =>
  makeVariantEvaluator(source, checkingController, checkingOperations).run();

export const checkingEvaluatorError = (source: string): string => {
  const transcript = runCheckingEvaluator(source);
  const printed = transcript[transcript.length - 2];
  if (printed === undefined) throw new EvaluatorFault(`no error printed for ${source}`);
  return printed;
};

export const ex_5_30 = checkingEvaluatorError;

// The exercise's worked failures, each pinned by its printed detail.
export const checkingFailureRows = (): ReadonlyArray<{
  readonly program: string;
  readonly printed: string;
}> => [
  { program: "(/ 1 0)", printed: checkingEvaluatorError("(/ 1 0)") },
  { program: "(car 5)", printed: checkingEvaluatorError("(car 5)") },
  { program: "no-such-variable", printed: checkingEvaluatorError("no-such-variable") },
  { program: "(cons 1)", printed: checkingEvaluatorError("(cons 1)") },
  { program: "(5 6)", printed: checkingEvaluatorError("(5 6)") },
];
