// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.50: ramb. The ramb form chooses over a seeded
 * deterministic shuffle of its alternatives instead of their
 * source order, as a separately named experiment. Applied to
 * Alyssa's generator, the word draws sample across articles,
 * nouns, and verbs instead of descending the first alternatives.
 * The seed makes the sampling reproducible: one seed, one order,
 * every run. Addition 4.50a is that seeding discipline itself —
 * the edition's shuffle is a linear-congruential generator over
 * the run seed, never ambient entropy.
 */
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";
import type { Value } from "../../packages/ch4/src/runtime/value.js";

/** The rambling generator: word choice goes through ramb. */
export const rambGeneratorSource = `
type Tree = string | Tree[];
type Sentence = [Tree, Tree, Tree];
const anElementOfR = (items: string[]): string => {
  require(items.length > 0);
  const first = items[0];
  return ramb(first === undefined ? "" : first, anElementOfR(items.slice(1)));
};
const parseWordGenR = (wordList: string[]): Tree => {
  const word = anElementOfR(wordList.slice(1));
  const first = wordList[0];
  return [first === undefined ? "" : first, word];
};
const articles: string[] = ["article", "the", "a"];
const nouns: string[] = ["noun", "student", "professor", "cat", "class"];
const verbs: string[] = ["verb", "studies", "lectures", "eats", "sleeps"];
const parseSimpleNounPhraseGenR = (): Tree => {
  const article = parseWordGenR(articles);
  const noun = parseWordGenR(nouns);
  return ["simple-noun-phrase", article, noun];
};
const parseSentenceGenR = (): Sentence => {
  const nounPhrase = parseSimpleNounPhraseGenR();
  const verb = parseWordGenR(verbs);
  return ["sentence", nounPhrase, verb];
};
parseSentenceGenR();
`;

/** The first n rambling sentences under one seed, in search order. */
export const sampledSentences = (seed: number, n: number): ReadonlyArray<Value> =>
  runAmbAnswers(rambGeneratorSource, "amb-ramb-experiment", seed).answers.slice(0, n);

/** The first n rambling sentences under one seed, rendered. */
export const renderedSamples = (seed: number, n: number): ReadonlyArray<string> =>
  sampledSentences(seed, n).map((value) => format(value));

/** The pinned demonstration seed. */
export const seed = 20260925;

export function ex_4_50(): string {
  const sampled = renderedSamples(seed, 3);
  return (
    "ramb shuffles each alternative list under the run seed before the " +
    "search descends, so the generator samples articles, nouns, and " +
    `verbs instead of the first alternatives (${sampled.length} sentences ` +
    "shown). One seed, one order, every run: reproducibility is the " +
    "addition the seed carries."
  );
}
