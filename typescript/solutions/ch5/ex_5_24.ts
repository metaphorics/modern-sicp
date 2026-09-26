// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  assign,
  branch,
  type ControllerLine,
  c,
  jump,
  jumpReg,
  lbl,
  mark,
  op,
  reg,
  restore,
  save,
  test,
} from "../../packages/ch5/src/02-simulator.js";
import {
  appendLines,
  evaluatorController,
  insertBeforeInstruction,
  isNilWord,
  isSpecialForm,
  makeListWord,
  makeVariantEvaluator,
  type Word,
  wordAt,
  wordItems,
  wordOperation1,
} from "../../packages/ch5/src/04-eceval.js";

const isPairForm = (w: Word): boolean => typeof w === "object" && w !== null && "car" in w;

const isElseClause = (clause: Word): boolean => {
  const predicate = wordAt(clause, 0);
  return (
    !isPairForm(predicate) &&
    typeof predicate === "object" &&
    "symbol" in predicate &&
    predicate.symbol === "else"
  );
};

// cond as a basic form: a loop over the clause list tests each
// predicate through eval-dispatch; a true predicate (or an else)
// selects the clause and its actions go to ev-sequence, so the last
// action of the clause is still in tail position. A selected clause
// with no actions returns the predicate's value; a walk off the end
// with no else answers #f, what cond->if would have produced.
const evCond: ControllerLine[] = [
  mark("ev-cond"),
  assign("unev", op("cond-clauses", reg("exp"))),
  mark("ev-cond-loop"),
  test("no-clauses?", reg("unev")),
  branch("ev-cond-no-clauses"),
  assign("exp", op("first-clause", reg("unev"))),
  test("cond-else-clause?", reg("exp")),
  branch("ev-cond-else"),
  save("unev"),
  save("continue"),
  assign("continue", lbl("ev-cond-decide")),
  assign("exp", op("cond-predicate", reg("exp"))),
  jump("eval-dispatch"),
  mark("ev-cond-decide"),
  restore("continue"),
  restore("unev"),
  test("true?", reg("val")),
  branch("ev-cond-actions"),
  assign("unev", op("rest-clauses", reg("unev"))),
  jump("ev-cond-loop"),
  mark("ev-cond-else"),
  mark("ev-cond-actions"),
  assign("exp", op("first-clause", reg("unev"))),
  assign("unev", op("cond-actions", reg("exp"))),
  test("no-more-exps?", reg("unev")),
  branch("ev-cond-bodyless"),
  save("continue"),
  jump("ev-sequence"),
  mark("ev-cond-bodyless"),
  jumpReg("continue"),
  mark("ev-cond-no-clauses"),
  assign("val", c(false)),
  jumpReg("continue"),
];

const isApplicationTest = (line: ControllerLine): boolean =>
  line.tag === "test" && line.op === "application?";

export const basicCondController: readonly ControllerLine[] = appendLines(
  insertBeforeInstruction(evaluatorController, isApplicationTest, "the application dispatch test", [
    test("cond?", reg("exp")),
    branch("ev-cond"),
  ]),
  evCond,
);

export const basicCondOperations = {
  "cond?": wordOperation1("cond?", (w) => isSpecialForm("cond", w)),
  "cond-clauses": wordOperation1("cond-clauses", (w) => makeListWord(wordItems(w).slice(1))),
  "no-clauses?": wordOperation1("no-clauses?", (w) => isNilWord(w)),
  "first-clause": wordOperation1("first-clause", (w) => wordAt(w, 0)),
  "rest-clauses": wordOperation1("rest-clauses", (w) => makeListWord(wordItems(w).slice(1))),
  "cond-else-clause?": wordOperation1("cond-else-clause?", isElseClause),
  "cond-predicate": wordOperation1("cond-predicate", (w) => wordAt(w, 0)),
  "cond-actions": wordOperation1("cond-actions", (w) => makeListWord(wordItems(w).slice(1))),
};

export const runBasicCond = (source: string): readonly string[] =>
  makeVariantEvaluator(source, basicCondController, basicCondOperations).run();

const classifyProgram = `
(define (classify n)
  (cond ((= n 0) (quote zero))
        ((= n 1) (quote one))
        (else (quote many))))
(classify 0)
(classify 1)
(classify 7)
(cond ((= 1 1)))
(cond ((= 1 2)))
(cond ((= 1 2) 9) (else 10))
`;

export const basicCondTranscript = (): readonly string[] => runBasicCond(classifyProgram);
export const ex_5_24 = basicCondTranscript;
