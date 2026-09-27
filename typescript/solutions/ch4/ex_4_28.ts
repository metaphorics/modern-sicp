// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.28: forcing the operator. The demonstration is ((id +) 2 3):
 * id's body answers a thunk whose expression is the variable +, so the
 * application clause must run actual-value on the operator before apply
 * can dispatch, and the forced call answers 5. The negation proves the
 * point: a variant evaluator whose application clause evaluates the
 * operator without forcing hands the thunk itself to apply, which fails
 * with "not a procedure: #[thunk]".
 */
import { Effect } from "effect";

import {
  isApplication,
  isAssignment,
  isBegin,
  isCond,
  isDefinition,
  isIf,
  isLambda,
  isQuoted,
  operands,
  operator,
} from "../../packages/ch4/src/01-metacircular.js";
import {
  type LazyEvaluator,
  lazyDriverWith,
  lazyEvaluator,
  makeLazyEvaluator,
} from "../../packages/ch4/src/02-lazy.js";
import type { Evaluate, Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";

/** The book's session: id delivering a primitive into operator position. */
export const session = ["(define (id x) x)", "((id +) 2 3)"];

/** Whether the expression is one of the base grammar's special forms,
 * which the base dispatch must see before any application clause does. */
const isSpecialForm = (exp: Value): boolean =>
  isQuoted(exp) ||
  isAssignment(exp) ||
  isDefinition(exp) ||
  isLambda(exp) ||
  isIf(exp) ||
  isBegin(exp) ||
  isCond(exp);

/** The variant: the application clause evaluates the operator with eval
 * instead of actual-value, so a thunk can reach apply. Everything else is
 * the section's evaluator, special forms included. */
export const unforcedOperatorEvaluator: LazyEvaluator = (() => {
  const base = makeLazyEvaluator();
  const evaluateVariant: Evaluate = (exp, env) => {
    if (isSpecialForm(exp) || !isApplication(exp)) {
      return base.evaluate(exp, env);
    }
    return Effect.flatMap(base.evaluate(operator(exp), env), (procedure) =>
      base.applyProcedure(procedure, operands(exp), env),
    );
  };
  return { ...base, evaluate: evaluateVariant };
})();

/** The forced run's last value, and the variant run's not-a-procedure
 * failure. */
export const answers = (): Effect.Effect<
  { readonly forced: string; readonly unforced: string },
  EvaluationError
> =>
  Effect.flatMap(lazyDriverWith(lazyEvaluator, session), (transcript) =>
    Effect.map(Effect.result(lazyDriverWith(unforcedOperatorEvaluator, session)), (outcome) => {
      const forced = transcript[transcript.length - 1] ?? "";
      if (outcome._tag === "Success") {
        return { forced, unforced: "no failure" };
      }
      const failure = outcome.failure;
      return {
        forced,
        unforced:
          failure._tag === "NotAProcedure" ? `not a procedure: ${failure.value}` : failure._tag,
      };
    }),
  );

export function ex_4_28(): string {
  const observed = Effect.runSync(answers());
  return (
    "The example is ((id +) 2 3) with id the identity: applying the outer " +
    "combination needs a procedure in the operator position, and id's body " +
    "answers a thunk of the variable +, so the application clause must run " +
    "actual-value on the operator before apply dispatches. Forced, the " +
    `expression answers ${observed.forced}. The need shows by negation: a ` +
    "variant whose application clause calls eval on the operator hands the " +
    `thunk to apply, which fails with "${observed.unforced}".`
  );
}
