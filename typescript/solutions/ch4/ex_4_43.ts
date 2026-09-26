// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.43: the yacht puzzle. The program assigns each daughter a
 * father, keeps the two facts the story states (the Melissa is named after
 * Sir Barnacle's daughter, and Mary Ann's father is Mr. Moore when we are
 * told her surname), requires the five fathers distinct with the four
 * fixed yacht namings, and holds the last sentence: Gabrielle's father
 * owns the yacht that is named after Dr. Parker's daughter. Told that Mary
 * Ann is a Moore, Lorna's father is Colonel Downing and the solution is
 * unique; without the surname, Dr. Parker also works.
 */
import { Effect } from "effect";

import {
  ambEvaluator,
  runAmbText,
  setupAmbEnvironment,
} from "../../packages/ch4/src/03-nondeterministic.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format } from "../../packages/ch4/src/read.js";

import { library } from "./ex_4_35.js";

/** The puzzle program; the `told` flag stands for the varied sentence.
 * The Melissa fact is built in rather than chosen, as the efficiency
 * note in the statement asks: Sir Barnacle's daughter is Melissa. */
export const yachtProgram = `
(define (distinct? items)
  (cond ((null? items) #t)
        ((null? (cdr items)) #t)
        ((member (car items) (cdr items)) #f)
        (else (distinct? (cdr items)))))
(define (member x xs)
  (cond ((null? xs) #f)
        ((equal? x (car xs)) xs)
        (else (member x (cdr xs)))))
(define (lookup key alist)
  (cond ((null? alist) #f)
        ((eq? key (car (car alist))) (car (cdr (car alist))))
        (else (lookup key (cdr alist)))))
(define (yacht-solve told)
  (let ((melissa 'barnacle))
    (let ((mary-ann (an-element-of '(moore downing hall barnacle parker)))
          (gabrielle (an-element-of '(moore downing hall barnacle parker)))
          (lorna (an-element-of '(moore downing hall barnacle parker)))
          (rosalind (an-element-of '(moore downing hall barnacle parker)))
          (parker-yacht (an-element-of '(mary-ann gabrielle lorna rosalind melissa))))
      (require (distinct? (list mary-ann gabrielle lorna rosalind melissa)))
      (if told (require (eq? mary-ann 'moore)) 'ok)
      (require (not (member parker-yacht '(lorna melissa rosalind gabrielle))))
      (require (not (eq? lorna 'moore)))
      (require (not (eq? rosalind 'hall)))
      (require (not (eq? gabrielle 'barnacle)))
      (let ((parker-daughter
             (cond ((eq? mary-ann 'parker) 'mary-ann)
                   ((eq? gabrielle 'parker) 'gabrielle)
                   ((eq? lorna 'parker) 'lorna)
                   ((eq? rosalind 'parker) 'rosalind)
                   (else 'melissa))))
        (let ((yachts (list (list 'moore 'lorna) (list 'downing 'melissa)
                            (list 'hall 'rosalind) (list 'barnacle 'gabrielle)
                            (list 'parker parker-yacht))))
          (require (eq? (lookup gabrielle yachts) parker-daughter))
          (list 'lornas-father lorna))))))
`;

/** Every solution for one value of the told flag. */
export const solutions = (told: boolean): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(
      runAmbText(ambEvaluator, [library, yachtProgram, `(yacht-solve ${told})`].join("\n"), env),
      (run) => run.answers.map(format),
    ),
  );

export function ex_4_43(): string {
  const told = Effect.runSync(solutions(true));
  const untold = Effect.runSync(solutions(false));
  return (
    "The program chooses each daughter's father, fixes the story's two " +
    "facts (the Melissa is named after Sir Barnacle's daughter, and Mary " +
    "Ann's father is Moore when we are told her surname), requires the " +
    "fathers distinct against the four fixed yacht namings, and holds the " +
    "last sentence: Gabrielle's father owns the yacht named after Dr. " +
    "Parker's daughter. Told that Mary Ann is a Moore, the puzzle has " +
    `${told.length} solution and Lorna's father is Colonel Downing ` +
    `(${told.join(" ")}). Without the surname there are ` +
    `${untold.length}: Lorna is Colonel Downing's daughter, or Dr. ` +
    "Parker's. The demonstration pins both answer sets and the exhaustion " +
    "after each."
  );
}
