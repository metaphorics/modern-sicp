// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.49: Alyssa's generation. Her parse-word ignores the
 * input sentence and always succeeds, drawing its word
 * ambiguously from the word list, so the parser generates
 * sentences instead of consuming them. Generation descends the
 * grammar's first alternatives — verbs cycle fastest under a
 * fixed article and noun — which is exactly the footnote's
 * complaint that sampling falls into the recursion. The finite
 * prefix (first six) is the observable; the sampling complaint
 * motivates 4.50's ramb.
 */
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";
import type { Value } from "../../packages/ch4/src/runtime/value.js";

/** The generating grammar: words drawn, never consumed. */
export const generatorSource = `
type Tree = string | Tree[];
type Sentence = [Tree, Tree, Tree];
const anElementOf = (items: string[]): string => {
  require(items.length > 0);
  const first = items[0];
  return choose(first === undefined ? "" : first, anElementOf(items.slice(1)));
};
const parseWordGen = (wordList: string[]): Tree => {
  const word = anElementOf(wordList.slice(1));
  const first = wordList[0];
  return [first === undefined ? "" : first, word];
};
const articles: string[] = ["article", "the", "a"];
const nouns: string[] = ["noun", "student", "professor", "cat", "class"];
const verbs: string[] = ["verb", "studies", "lectures", "eats", "sleeps"];
const parseSimpleNounPhraseGen = (): Tree => {
  const article = parseWordGen(articles);
  const noun = parseWordGen(nouns);
  return ["simple-noun-phrase", article, noun];
};
const parseSentenceGen = (): Sentence => {
  const nounPhrase = parseSimpleNounPhraseGen();
  const verb = parseWordGen(verbs);
  return ["sentence", nounPhrase, verb];
};
parseSentenceGen();
`;

/** The first n generated sentences, in search order. */
export const generatedSentences = (n: number): ReadonlyArray<Value> =>
  runAmbAnswers(generatorSource, "amb-depth-first-experiment", 1).answers.slice(0, n);

/** The first n generated sentences, rendered. */
export const renderedSentences = (n: number): ReadonlyArray<string> =>
  generatedSentences(n).map((value) => format(value));

export function ex_4_49(): string {
  const six = renderedSentences(6);
  return (
    "Alyssa's parse-word ignores the input and draws from an-element-of, " +
    "always succeeding; generation descends the first alternatives, " +
    `answering ${six.length} sentences with verbs cycling fastest under a ` +
    "fixed article and noun. The recursion sampling is the footnote's " +
    "complaint, and 4.50's ramb is its remedy."
  );
}
