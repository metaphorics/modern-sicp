// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.49: Alyssa's generation. Her parse-word ignores the input
 * sentence and always succeeds, drawing its word ambiguously from the word
 * list, so the parser generates sentences instead of consuming them; parse
 * is then called with the empty input, whose final distinctness
 * requirement passes vacuously. The sentences descend the grammar's first
 * alternatives: every verb with the first article-noun pair, then the
 * second noun with the first verb, which is exactly the footnote's
 * complaint that generation samples the recursive grammar badly.
 */
import { Effect } from "effect";

import {
  ambEvaluator,
  runAmbText,
  setupAmbEnvironment,
} from "../../packages/ch4/src/03-nondeterministic.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format } from "../../packages/ch4/src/read.js";

/** The generating grammar: parse-word draws from the word list. */
export const generator = `
(define (require p) (if (not p) (amb)))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(define (parse-word-gen word-list)
  (list (car word-list) (an-element-of (cdr word-list))))
(define (parse-simple-noun-phrase-gen)
  (list 'simple-noun-phrase (parse-word-gen articles) (parse-word-gen nouns)))
(define (parse-sentence-gen)
  (list 'sentence (parse-simple-noun-phrase-gen) (parse-word-gen verbs)))
(parse-sentence-gen)
`;

/** The word lists of the section. */
export const words = `
(define nouns '(noun student professor cat class))
(define verbs '(verb studies lectures eats sleeps))
(define articles '(article the a))
`;

/** The first n generated sentences. */
export const sentences = (n: number): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(runAmbText(ambEvaluator, words + generator, env, n), (run) =>
      run.answers.map(format),
    ),
  );

export function ex_4_49(): string {
  const six = Effect.runSync(sentences(6));
  return (
    "Alyssa's parse-word ignores *unparsed* and draws its word from " +
    "an-element-of over the word list, always succeeding; parse is then " +
    "called with the empty input, whose final distinctness requirement " +
    "passes vacuously, and try-again walks generated sentences. The first " +
    "six all descend the grammar's first alternatives: " +
    six.join("; ") +
    ". That is exactly the footnote's complaint: the generation falls into " +
    "the recursion and samples the language badly, which is what 4.50's " +
    "ramb is for."
  );
}
