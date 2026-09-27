// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.47: Louis Reasoner's parse-verb-phrase. In the written order
 * it parses: the amb's alternatives are analyzed once but evaluated one at
 * a time only when the search tries them, so the first alternative
 * consumes a verb and the ordinary parses come out. But once a parse has
 * been answered the next try-again descends parse-verb-phrase endlessly on
 * the exhausted input: the recursive alternative is entered before the
 * noun phrase can fail, and since the recursion performs no choice that
 * consumes, it never returns. Interchanging the two expressions is worse:
 * the recursion precedes any consumption, and the first parse completes
 * only because each recursion level eventually falls through the whole
 * depth budget to a verb. The demonstration bounds the recursion with a
 * depth parameter, turning the divergence into termination the test can
 * pin.
 */
import { Effect } from "effect";

import {
  ambEvaluator,
  runAmbText,
  setupAmbEnvironment,
} from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";

import { parser } from "./ex_4_45.js";

/** Louis's definition beside a depth-capped copy and an interchanged one. */
export const louisProgram = `
(define (parse-verb-phrase-louis)
  (amb (parse-word verbs)
       (list 'verb-phrase
             (parse-verb-phrase-louis)
             (parse-prepositional-phrase))))
(define (parse-verb-phrase-capped depth)
  (amb (parse-word verbs)
       (if (< depth 25)
           (list 'verb-phrase
                 (parse-verb-phrase-capped (+ depth 1))
                 (parse-prepositional-phrase))
           (amb))))
(define (parse-verb-phrase-swapped depth)
  (amb (if (< depth 25)
           (list 'verb-phrase
                 (parse-verb-phrase-swapped (+ depth 1))
                 (parse-prepositional-phrase))
           (amb))
       (parse-word verbs)))
(define (parse-with parser input)
  (set! *unparsed* input)
  (let ((sent (list 'sentence (parse-noun-phrase) (parser))))
    (require (null? *unparsed*))
    sent))
`;

const runParse = (
  call: string,
  limit: number,
): { answers: ReadonlyArray<string>; exhausted: boolean } =>
  Effect.runSync(
    Effect.flatMap(setupAmbEnvironment(), (env) =>
      Effect.map(
        Effect.result(
          runAmbText(ambEvaluator, [parser, louisProgram, call].join("\n"), env, limit),
        ),
        (outcome) =>
          outcome._tag === "Failure"
            ? { answers: [], exhausted: true }
            : {
                answers: outcome.success.answers.map(format),
                exhausted: outcome.success.exhausted,
              },
      ),
    ),
  );

/** Louis's first parse of the small sentence; his try-again is never
 * taken here because it is the divergence the exercise predicts. */
export const louisFirst = (): string =>
  // limit 1: Louis's uncapped try-again is the divergence the exercise
  // predicts and must never be taken.
  runParse(`(parse-with (lambda () (parse-verb-phrase-louis)) '(the cat eats))`, 1).answers[0] ??
  "none";

/** The capped copy answers the same first parse; its try-again runs dry
 * because the recursion never consumes. */
export const cappedResult = (): { answers: ReadonlyArray<string>; exhausted: boolean } =>
  runParse(`(parse-with (lambda () (parse-verb-phrase-capped 0)) '(the cat eats))`, 2);

/** The interchanged order still answers the ordinary first parse. */
export const swappedFirst = (): string =>
  runParse(`(parse-with (lambda () (parse-verb-phrase-swapped 0)) '(the cat eats))`, 1)
    .answers[0] ?? "none";

export function ex_4_47(): string {
  const first = louisFirst();
  const capped = cappedResult();
  const swapped = swappedFirst();
  return (
    "Louis's version parses: the amb's alternatives are analyzed once and " +
    "evaluated only when the search tries them, so the first alternative " +
    `consumes a verb and the ordinary first parse ${first} comes out. But ` +
    "the second alternative recurses before anything fails or consumes, so " +
    "once a parse has been answered the next try-again descends " +
    "parse-verb-phrase endlessly on the exhausted input: the depth-capped " +
    "copy answers the same first parse " +
    `${capped.answers[0] ?? "none"} and then runs dry after ` +
    `${capped.exhausted ? "one" : "no"} answer, which is the divergence ` +
    "turned into exhaustion. Interchanging the two amb expressions makes " +
    "the recursion precede any consumption: even the capped copy burns its " +
    "whole depth budget before the verb, and the first parse " +
    `${swapped} completes only through the second alternative.`
  );
}
