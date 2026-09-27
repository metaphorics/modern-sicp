// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.48: extending the grammar with adjectives. `parse-modifiers`
 * consumes a run of adjective words from the input: either the modifier
 * list closes with the noun word, or an adjective is consumed and the
 * recursion continues, so every alternative consumes input and the search
 * terminates. The modifiers print as a list inside the simple noun phrase.
 */
import { Effect } from "effect";

import {
  ambEvaluator,
  runAmbText,
  setupAmbEnvironment,
} from "../../packages/ch4/src/03-nondeterministic.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format } from "../../packages/ch4/src/read.js";

import { parser } from "./ex_4_45.js";

/** The extended grammar: adjectives join the noun phrase. */
export const adjectiveProgram = (input: string): string => `
(define adjectives '(adjective sleepy quick brown))
(define (parse-modifiers)
  (amb (list (parse-word nouns))
       (cons (parse-word adjectives) (parse-modifiers))))
(define (parse-simple-noun-phrase-v2)
  (list 'simple-noun-phrase (parse-word articles) (parse-modifiers)))
(define (parse-v2 text)
  (set! *unparsed* text)
  (let ((sent (list 'sentence (parse-simple-noun-phrase-v2) (parse-word verbs))))
    (require (null? *unparsed*))
    sent))
(parse-v2 '(${input}))
`;

/** Every parse of the input under the extended grammar. */
export const parses = (input: string): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(runAmbText(ambEvaluator, [parser, adjectiveProgram(input)].join("\n"), env), (run) =>
      run.answers.map(format),
    ),
  );

export function ex_4_48(): string {
  const one = Effect.runSync(parses("the sleepy cat eats"));
  const two = Effect.runSync(parses("the quick brown cat sleeps"));
  return (
    "Adjectives join the noun phrase: parse-modifiers either closes the " +
    "modifier list with the noun word or consumes an adjective word and " +
    "continues, so every alternative consumes input and the search " +
    "terminates. For 'the sleepy cat eats' the empty-modifier alternative " +
    `fails at the noun, so the adjective parse ${one.join(" ")} is the ` +
    "first answer, and 'the quick brown cat sleeps' parses with a " +
    `two-adjective modifier list: ${two.join(" ")}.`
  );
}
