// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.45: five parses. With the section's grammar the sentence
 * "The professor lectures to the student in the class with the cat" parses
 * in exactly five ways under the depth-first search, differing in where
 * "in the class" and "with the cat" attach.
 */
import { Effect } from "effect";

import {
  ambEvaluator,
  runAmbText,
  setupAmbEnvironment,
} from "../../packages/ch4/src/03-nondeterministic.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format } from "../../packages/ch4/src/read.js";

/** The section's parser. */
export const parser = `
(define nouns '(noun student professor cat class))
(define verbs '(verb studies lectures eats sleeps))
(define articles '(article the a))
(define prepositions '(prep for to in by with))
(define *unparsed* '())
(define (require p) (if (not p) (amb)))
(define (memq x xs)
  (cond ((null? xs) #f)
        ((eq? x (car xs)) xs)
        (else (memq x (cdr xs)))))
(define (parse-word word-list)
  (require (not (null? *unparsed*)))
  (require (memq (car *unparsed*) (cdr word-list)))
  (let ((found-word (car *unparsed*)))
    (set! *unparsed* (cdr *unparsed*))
    (list (car word-list) found-word)))
(define (parse-simple-noun-phrase)
  (list 'simple-noun-phrase (parse-word articles) (parse-word nouns)))
(define (parse-noun-phrase)
  (define (maybe-extend noun-phrase)
    (amb noun-phrase
         (maybe-extend (list 'noun-phrase
                             noun-phrase
                             (parse-prepositional-phrase)))))
  (maybe-extend (parse-simple-noun-phrase)))
(define (parse-prepositional-phrase)
  (list 'prep-phrase (parse-word prepositions) (parse-noun-phrase)))
(define (parse-verb-phrase)
  (define (maybe-extend verb-phrase)
    (amb verb-phrase
         (maybe-extend (list 'verb-phrase
                             verb-phrase
                             (parse-prepositional-phrase)))))
  (maybe-extend (parse-word verbs)))
(define (parse-sentence)
  (list 'sentence (parse-noun-phrase) (parse-verb-phrase)))
(define (parse input)
  (set! *unparsed* input)
  (let ((sent (parse-sentence)))
    (require (null? *unparsed*))
    sent))
`;

/** The ambiguous sentence of the exercise. */
export const sentence = "(the professor lectures to the student in the class with the cat)";

/** Every parse of the sentence, in search order. */
export const parses = (): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(runAmbText(ambEvaluator, [parser, `(parse '${sentence})`].join("\n"), env), (run) =>
      run.answers.map(format),
    ),
  );

export function ex_4_45(): string {
  const found = Effect.runSync(parses());
  if (found.length !== 5) {
    throw new Error(`expected exactly five parses, got ${found.length}`);
  }
  return (
    "The evaluator produces exactly five parses and then reports " +
    "exhaustion. They differ in where the two prepositional phrases " +
    "attach. Parse 1: both attach to the verb phrase, so the professor " +
    "lectures to the student in the class while holding the cat. Parse 2: " +
    "with the cat attaches inside in the class's noun phrase, so the " +
    "class has the cat. Parse 3: in the class attaches inside to the " +
    "student's noun phrase, so the student is in the class. Parse 4: both " +
    "attach at the student's noun phrase, with the cat outside in the " +
    "class: the student in the class has the cat. Parse 5: both attach " +
    "inside the student's noun phrase, with the cat nested inside in the " +
    "class. The five trees are pinned in the test in search order."
  );
}
