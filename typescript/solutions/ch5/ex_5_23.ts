// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { CaseClause, Decl, Expr, Stmt } from "../../packages/ch4/src/syntax/ast.ts";
import type { Span } from "../../packages/ch4/src/syntax/diagnostics.ts";
import {
  assign,
  branch,
  evaluatorController,
  gotoLabel,
  insertBeforeInstruction,
  type MachineStatement,
  makeEvaluator,
  type Operation,
  op,
  register,
  replaceSegment,
  test,
  type Word,
} from "../../packages/ch5/src/04-eceval.ts";
import { mark } from "./ex_5_07.ts";

type EvaluatorStatement = MachineStatement<Word>;

const span: Span = { start: 0, end: 0, line: 1, column: 1 };

/** The transformer of exercise 5.23: one machine operation builds the
 * derived `if` chain out of the switch's clauses and the dispatch
 * re-enters eval on the transformed form, so the rest of the controller
 * never knows the form existed. Each clause runs one at a time and ends
 * the switch, the derived-form discipline of the book's cond; the basic
 * form's fall-through and break bookkeeping is what exercise 5.24 adds
 * in the controller. A clause that would fall through is left to the
 * basic form or rejected there, never silently mistranslated. */
export const makeSwitchTransformer = (): {
  operations: Readonly<Record<string, Operation<Word>>>;
  fired: () => number;
} => {
  let count = 0;
  return {
    operations: {
      transformSwitch: (args) => {
        const node = switchNode(args[0]);
        return node !== null && clausesFit(node);
      },
      switchToIf: (args) => {
        const node = switchNode(args[0]);
        if (node === null) return undefined;
        count += 1;
        return switchToIfChain(node);
      },
    },
    fired: () => count,
  };
};

interface SwitchForm {
  readonly tag: "switch";
  readonly discriminant: Expr;
  readonly cases: ReadonlyArray<CaseClause>;
  readonly defaultBody: ReadonlyArray<Decl | Stmt> | null;
  readonly span: Span;
}

const isSwitchForm = (word: Word): word is SwitchForm =>
  typeof word === "object" && word !== null && "tag" in word && word.tag === "switch";

const switchNode = (word: Word): SwitchForm | null => (isSwitchForm(word) ? word : null);

const endsTheSwitch = (body: ReadonlyArray<Decl | Stmt>): boolean => {
  const last = body[body.length - 1];
  if (last === undefined) return true;
  return last.tag === "return" || last.tag === "throw";
};

const clausesFit = (node: SwitchForm): boolean =>
  node.cases.every((clause) => endsTheSwitch(clause.body)) &&
  (node.defaultBody === null || endsTheSwitch(node.defaultBody));

const switchToIfChain = (node: SwitchForm): Stmt => {
  const build = (index: number): Stmt | null => {
    const clause = node.cases[index];
    if (clause === undefined) {
      const fallback = node.defaultBody;
      return fallback === null || fallback.length === 0
        ? null
        : { tag: "block", body: [...fallback], span };
    }
    return {
      tag: "if",
      test: { tag: "binary", op: "===", left: node.discriminant, right: clause.test, span },
      consequent: { tag: "block", body: [...clause.body], span },
      alternative: build(index + 1),
      span,
    };
  };
  const chain = build(0);
  return chain === null ? { tag: "block", body: [], span } : chain;
};

/** The derived-entry controller: the derived test goes before the basic
 * switch pair, exactly the splice point the exercise is about, and the
 * derived segment re-enters eval-form on the transformed form. A switch
 * the transformer declines falls through to the basic form. */
export const derivedSwitchController: readonly EvaluatorStatement[] = replaceSegment(
  insertBeforeInstruction(
    evaluatorController,
    (line) => line.tag === "test" && line.operation === "isSwitchStmt",
    "the derived switch test goes first",
    [test("transformSwitch", register("expr")), branch("ef-switch-derived")],
  ),
  "done",
  "done",
  [
    mark("ef-switch-derived"),
    assign("expr", op("switchToIf", register("expr"))),
    gotoLabel("eval-form"),
  ],
);

/** Exercise 5.23: the transformer answers the switch sessions with the
 * same values the basic form answers, and the counter shows one
 * transformation per switch. */
export const ex_5_23 = (): readonly string[] => {
  const { operations, fired } = makeSwitchTransformer();
  const program = [
    "function classify(n: number) {",
    "  switch (n) {",
    '    case 0: return "zero";',
    '    case 1: return "one";',
    '    default: return "many";',
    "  }",
    "}",
    "console.log(classify(0));",
    "console.log(classify(1));",
    "console.log(classify(7));",
  ].join("\n");
  const variant = makeEvaluator(program, operations, derivedSwitchController);
  const result = variant.run();
  if (result.outcome.tag !== "ok") {
    throw new Error(`the transformed session faulted: ${JSON.stringify(result.outcome.error)}`);
  }
  const base = makeEvaluator(program).run();
  const baseLines = base.transcript;
  for (let i = 0; i < Math.max(result.transcript.length, baseLines.length); i += 1) {
    if (result.transcript[i] !== baseLines[i]) {
      throw new Error(
        `the transformed session diverged at line ${i}: ${result.transcript[i] ?? "<missing>"} != ${baseLines[i] ?? "<missing>"}`,
      );
    }
  }
  return [...result.transcript, `transformations: ${fired()}`];
};
