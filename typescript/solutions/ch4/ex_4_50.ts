// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.50: ramb. The ramb special form is amb over a shuffled copy
 * of its alternatives: the variant dispatch recognizes ramb, and
 * analyze-ramb shuffles the analyzed alternatives with Fisher-Yates over
 * the tuning's seeded xorshift generator before the search descends. One
 * generator shared by two evaluators makes them ramble in step, and a
 * different seed draws a different order. Applied to Alyssa's generation,
 * ramb samples article, noun, and verb instead of descending the first
 * alternatives forever.
 */
import { Effect } from "effect";

import {
  type AmbEvaluator,
  makeAmbEvaluator,
  makeXorshift32,
  runAmbText,
  setupAmbEnvironment,
} from "../../packages/ch4/src/03-nondeterministic.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format } from "../../packages/ch4/src/read.js";

import { words } from "./ex_4_49.js";

/** The rambling generator: the word choice goes through ramb. */
export const rambGenerator = `
(define (require p) (if (not p) (amb)))
(define (an-element-of-r items)
  (require (not (null? items)))
  (ramb (car items) (an-element-of-r (cdr items))))
(define (parse-word-gen word-list)
  (list (car word-list) (an-element-of-r (cdr word-list))))
(define (parse-simple-noun-phrase-gen)
  (list 'simple-noun-phrase (parse-word-gen articles) (parse-word-gen nouns)))
(define (parse-sentence-gen)
  (list 'sentence (parse-simple-noun-phrase-gen) (parse-word-gen verbs)))
(parse-sentence-gen)
`;

/** The pinned demonstration seed. */
export const seed = 20260925;

/** One seeded ramb evaluator. */
export const rambled = (seedValue: number): AmbEvaluator =>
  makeAmbEvaluator({ ramb: true, random: makeXorshift32(seedValue) });

/** The first n sentences Alyssa's generator produces under a seeded ramb. */
export const sampledSentences = (
  seedValue: number,
  n: number,
): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(runAmbText(rambled(seedValue), words + rambGenerator, env, n), (run) =>
      run.answers.map(format),
    ),
  );

export function ex_4_50(): string {
  const pinned = Effect.runSync(sampledSentences(seed, 3));
  const other = Effect.runSync(sampledSentences(7, 3));
  return (
    "ramb is amb over a shuffled copy of its alternatives: the variant " +
    "dispatch recognizes the form, and analyze-ramb shuffles the analyzed " +
    "alternatives with Fisher-Yates over the edition's seeded xorshift " +
    "generator (fixed seed " +
    `${seed}, so the demonstration is reproducible) before the search ` +
    "descends; the search strategy changes, the undo discipline and the " +
    "try-again protocol do not. Under the seed Alyssa's generator samples " +
    `${pinned.join("; ")}, and a different seed answers ` +
    `${other[0] ?? "none"} first: the sentences vary in article, noun, and ` +
    "verb instead of descending one recursion, which is how ramb helps " +
    "Alyssa's problem."
  );
}
