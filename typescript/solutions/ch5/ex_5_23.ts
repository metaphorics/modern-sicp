// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  assign,
  branch,
  type ControllerLine,
  jump,
  mark,
  op,
  reg,
  test,
} from "../../packages/ch5/src/02-simulator.js";
import {
  appendLines,
  evaluatorController,
  insertBeforeInstruction,
  isNilWord,
  isPairWord,
  isSpecialForm,
  makeListWord,
  makeSymbolWord,
  makeVariantEvaluator,
  type Word,
  wordAt,
  wordItems,
  wordOperation1,
} from "../../packages/ch5/src/04-eceval.js";

const isNamed = (w: Word, name: string): boolean =>
  !isPairWord(w) && typeof w === "object" && "symbol" in w && w.symbol === name;

// The book's 4.1.2 cond->if: each clause becomes an if, a bodyless
// clause's consequent is its own test, and a missing else ends the
// chain in the false constant.
const sequenceToExpression = (actions: readonly Word[]): Word =>
  actions.length === 1 ? (actions[0] as Word) : makeListWord([makeSymbolWord("begin"), ...actions]);

const expandClauses = (clauses: Word): Word => {
  if (isNilWord(clauses)) return false;
  const first = wordAt(clauses, 0);
  const predicate = wordAt(first, 0);
  const actions = wordItems(first).slice(1);
  const consequent = actions.length === 0 ? predicate : sequenceToExpression(actions);
  if (isNamed(predicate, "else")) return consequent;
  return makeListWord([
    makeSymbolWord("if"),
    predicate,
    consequent,
    expandClauses(makeListWord(wordItems(clauses).slice(1))),
  ]);
};

const condToIf = (form: Word): Word => expandClauses(makeListWord(wordItems(form).slice(1)));

// The book's let->combination: a lambda over the binding names applied
// to the binding initializers in one expression.
const letToCombination = (form: Word): Word => {
  const bindings = wordItems(wordAt(form, 1));
  const names = bindings.map((binding) => wordAt(binding, 0));
  const initializers = bindings.map((binding) => wordAt(binding, 1));
  return makeListWord([
    makeListWord([makeSymbolWord("lambda"), makeListWord(names), ...wordItems(form).slice(2)]),
    ...initializers,
  ]);
};

// The dispatch grows one test per derived form, ahead of the
// application test: a cond or let is a pair, so the derived tests must
// come first, and each entry transforms exp and re-enters eval-dispatch.
const isApplicationTest = (line: ControllerLine): boolean =>
  line.tag === "test" && line.op === "application?";

const dispatchTests: ControllerLine[] = [
  test("cond?", reg("exp")),
  branch("ev-cond-derived"),
  test("let?", reg("exp")),
  branch("ev-let-derived"),
];

const transformerEntries: ControllerLine[] = [
  mark("ev-cond-derived"),
  assign("exp", op("cond->if", reg("exp"))),
  jump("eval-dispatch"),
  mark("ev-let-derived"),
  assign("exp", op("let->combination", reg("exp"))),
  jump("eval-dispatch"),
];

export const derivedExpressionController: readonly ControllerLine[] = appendLines(
  insertBeforeInstruction(
    evaluatorController,
    isApplicationTest,
    "the application dispatch test",
    dispatchTests,
  ),
  transformerEntries,
);

export const derivedExpressionOperations = {
  "cond?": wordOperation1("cond?", (w) => isSpecialForm("cond", w)),
  "let?": wordOperation1("let?", (w) => isSpecialForm("let", w)),
  "cond->if": wordOperation1("cond->if", condToIf),
  "let->combination": wordOperation1("let->combination", letToCombination),
};

export const runDerivedExpressions = (source: string): readonly string[] =>
  makeVariantEvaluator(source, derivedExpressionController, derivedExpressionOperations).run();

const classifyProgram = `
(define (classify n)
  (cond ((= n 0) (quote zero))
        ((= n 1) (quote one))
        (else (quote many))))
(classify 0)
(classify 1)
(classify 7)
(let ((a 2) (b 3)) (* a b))
(cond ((= 1 1)))
(cond ((= 1 2)))
`;

export const derivedExpressionTranscript = (): readonly string[] =>
  runDerivedExpressions(classifyProgram);
export const ex_5_23 = derivedExpressionTranscript;
