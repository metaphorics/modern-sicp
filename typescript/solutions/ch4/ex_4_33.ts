// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.33: quote produces lazy lists. With the 4.2.3 procedural
 * cons, car, and cdr defined, a quoted list is still an ordinary pair of
 * the shared runtime, and the procedural car applies its argument, so
 * Ben's (car '(a b c)) fails with "not a procedure: (a b c)". The fix
 * reroutes quote handling: a quotation of a non-empty proper list lifts
 * into the cons chain of its quoted elements, so the list the driver
 * hands out is the same lazy structure the program builds by hand; atoms,
 * the empty list, and dotted tails stay ordinary data.
 */
import { Effect } from "effect";

import {
  lazyDriverWith,
  lazyEvaluator,
  makeLazyEvaluator,
} from "../../packages/ch4/src/02-lazy.js";
import type { EvaluationError, NotAProcedure } from "../../packages/ch4/src/errors.js";

export const proceduralPairs = [
  "(define (cons x y) (lambda (m) (m x y)))",
  "(define (car z) (z (lambda (p q) p)))",
  "(define (cdr z) (z (lambda (p q) q)))",
];

export const listRefDefinition =
  "(define (list-ref items n) (if (= n 0) (car items) (list-ref (cdr items) (- n 1))))";

/** Ben's failing expression under the section evaluator. */
export const benSession = [...proceduralPairs, listRefDefinition, "(car '(a b c))"];

/** The lifted session: the same expressions, true lazy lists. */
export const liftedSession = [
  ...proceduralPairs,
  listRefDefinition,
  "(car '(a b c))",
  "(car (cdr '(a b c)))",
  "(list-ref '(a b c d) 3)",
];

/** The lazy evaluator with quoted proper lists lifted into lazy pairs. */
export const liftedEvaluator = makeLazyEvaluator({ liftQuotedLists: true });

/** Ben's failure under the plain evaluator, and the lifted answers. */
export const answers = (): Effect.Effect<
  { readonly plain: string; readonly lifted: ReadonlyArray<string> },
  EvaluationError
> =>
  Effect.flatMap(
    Effect.flatMap(Effect.result(lazyDriverWith(lazyEvaluator, benSession)), (outcome) =>
      outcome._tag === "Failure" && outcome.failure._tag === "NotAProcedure"
        ? Effect.succeed(outcome.failure)
        : Effect.die(new Error("expected the procedural car to fail")),
    ),
    (plain: NotAProcedure) =>
      Effect.map(lazyDriverWith(liftedEvaluator, liftedSession), (transcript) => ({
        plain: `not a procedure: ${plain.value}`,
        lifted: transcript.filter((_, i) => i % 4 === 3).slice(4),
      })),
  );

export function ex_4_33(): string {
  const observed = Effect.runSync(answers());
  const [car, cadr, listRef] = observed.lifted;
  return (
    "Ben's error is real: after the 4.2.3 definitions shadow the pair " +
    "primitives, '(a b c) is still an ordinary pair, and the procedural " +
    `car applies its argument, which fails with "${observed.plain}". The ` +
    "fix reroutes quote handling: a quotation of a non-empty proper list " +
    "lifts into the cons chain of its quoted elements, so quoted lists are " +
    `true lazy pairs. Lifted, (car '(a b c)) answers ${car}, (car (cdr ` +
    `'(a b c))) answers ${cadr} through the lazy spine, and the ` +
    `object-language list-ref walks the quoted list to ${listRef}. The ` +
    "elements are quoted data wrapped by the procedural cons, so they are " +
    "forced only when used, the same rule the program's own lists follow."
  );
}
