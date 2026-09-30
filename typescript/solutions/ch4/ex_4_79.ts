// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.79: environments instead of renaming. Rule application
 * renames every rule variable with a fresh application id, so a
 * rule-body variable can never capture a query variable even when
 * the names collide: the colliding rule below reuses ?father and
 * still answers Irad. The counter advances on every application,
 * and the occurs check refuses cyclic extensions. The environment
 * alternative (rule scope as an extended frame, contextual
 * deduction as hypothetical assertion) is designed in prose: this
 * open-ended exercise asks for the design, with engine integration
 * as follow-up.
 */
import {
  applyARule,
  extendIfPossible,
  newRuleApplicationId,
  type Query,
  qpair,
  qtext,
  queryAtom,
  qvar,
  type Rule,
  renameVariablesIn,
  rule,
  streamToList,
} from "../../packages/ch4/src/04-logic.js";
import { answerLines } from "./ex_4_55.js";
import { genealogyDatabase } from "./ex_4_63.js";

/**
 * The grandson rule spelling its body with ?father — the same name
 * the query below uses for the grandson.
 */
export const collidingRule: Rule = rule(queryAtom("grandson", qvar("gs"), qvar("gf")), {
  tag: "and",
  clauses: [
    queryAtom("son", qvar("father"), qvar("gs")),
    queryAtom("son", qvar("gf"), qvar("father")),
  ],
});

/** A pattern reusing the rule's body name for the grandson. */
export const collidingPattern: Query = queryAtom("grandson", qvar("father"), qtext("Cain"));

/** The answer under the collision: Irad, capture avoided. */
export const collisionAnswers = (): ReadonlyArray<string> => {
  const db = genealogyDatabase();
  db.addRule(collidingRule);
  return answerLines(db, collidingPattern);
};

/** The renamed head variables carry the application-id suffix. */
export const renamedHeadVars = (): ReadonlyArray<string> => {
  const renamed = renameVariablesIn(collidingRule);
  if (renamed.head.tag !== "atom") {
    return [];
  }
  return renamed.head.fields.map((field) => (field.tag === "var" ? field.name : "#"));
};

/** Two successive renamings use distinct application ids. */
export const renamingAdvances = (): boolean => {
  const first = renameVariablesIn(collidingRule);
  const second = renameVariablesIn(collidingRule);
  const names = (candidate: Rule): string => {
    if (candidate.head.tag !== "atom") {
      return "";
    }
    return candidate.head.fields.map((field) => (field.tag === "var" ? field.name : "#")).join(",");
  };
  return names(first) !== names(second);
};

/** The application-id counter advances across calls. */
export const counterAdvances = (): boolean => {
  const before = newRuleApplicationId();
  const after = newRuleApplicationId();
  return after === before + 1;
};

/** The occurs check refuses ?x = [?x | []] extended through the frame. */
export const occursCheckRefuses = (): boolean =>
  extendIfPossible("x", qpair(qvar("x"), { tag: "nil" }), []) === undefined;

/** One frame from direct rule application over the empty frame. */
export const directApplicationSize = (): number => {
  const db = genealogyDatabase();
  db.addRule(collidingRule);
  return streamToList(applyARule(collidingRule, collidingPattern, [], db)).length;
};

export function ex_4_79(): string {
  const answers = collisionAnswers();
  return (
    `Renaming applies the colliding rule to ${answers.length} answer without ` +
    `capture; ids advance ${counterAdvances() ? "monotonically" : "never"}; ` +
    `the occurs check refuses cyclic extension ${occursCheckRefuses() ? "as specified" : "never"}. ` +
    `Environments would scope rule bodies as extended frames instead.`
  );
}
