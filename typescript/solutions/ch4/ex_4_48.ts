// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.48: adjectives in the grammar. `parseModifiers`
 * consumes an adjective run before the noun: either the noun word
 * closes the modifiers, or an adjective is consumed and the
 * recursion continues, so every alternative consumes input and the
 * search terminates. The modifiers print as a list inside the
 * simple noun phrase.
 */
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";
import type { Value } from "../../packages/ch4/src/runtime/value.js";

const grammar = `
type Tree = string | Tree[];
type Parsed = [Tree, string[]];
const parseWord = (words: string[], allowed: string[]): Parsed => {
  const candidate = words[0];
  const first = candidate === undefined ? "" : candidate;
  require(words.length > 0 && allowed.includes(first));
  return [first, words.slice(1)];
};
const parseArticle = (words: string[]): Parsed => {
  const parsed = parseWord(words, ["the", "a"]);
  const word = parsed[0];
  const rest = parsed[1];
  return [["article", word], rest];
};
const parseNoun = (words: string[]): Parsed => {
  const parsed = parseWord(words, ["cat", "dog"]);
  const word = parsed[0];
  const rest = parsed[1];
  return [["noun", word], rest];
};
const parseAdjective = (words: string[]): Parsed => {
  const parsed = parseWord(words, ["sleepy", "quick", "brown"]);
  const word = parsed[0];
  const rest = parsed[1];
  return [["adjective", word], rest];
};
const parseModifiers = (words: string[]): Parsed =>
  choose(parseNoun(words), parseAdjectiveThenModifiers(words));
const parseAdjectiveThenModifiers = (words: string[]): Parsed => {
  const parsedAdjective = parseAdjective(words);
  const adjective = parsedAdjective[0];
  const afterAdjective = parsedAdjective[1];
  const parsedMore = parseModifiers(afterAdjective);
  const more = parsedMore[0];
  const rest = parsedMore[1];
  return [[adjective, more], rest];
};
`;

/** The sentence parser over the extended noun phrase. */
export const adjectiveProgram = (input: string): string => `${grammar}
const parseSimpleNounPhrase = (words: string[]): Parsed => {
  const parsedArticle = parseArticle(words);
  const article = parsedArticle[0];
  const afterArticle = parsedArticle[1];
  const parsedModifiers = parseModifiers(afterArticle);
  const modifiers = parsedModifiers[0];
  const rest = parsedModifiers[1];
  return [["simple-noun-phrase", article, modifiers], rest];
};
const parseVerb = (words: string[]): Parsed => {
  const parsed = parseWord(words, ["eats", "sleeps"]);
  const word = parsed[0];
  const rest = parsed[1];
  return [["verb", word], rest];
};
const parseSentence = (words: string[]): Parsed => {
  const parsedNounPhrase = parseSimpleNounPhrase(words);
  const nounPhrase = parsedNounPhrase[0];
  const afterNoun = parsedNounPhrase[1];
  const parsedVerbPhrase = parseVerb(afterNoun);
  const verbPhrase = parsedVerbPhrase[0];
  const rest = parsedVerbPhrase[1];
  require(rest.length === 0);
  return [["sentence", nounPhrase, verbPhrase], []];
};
parseSentence(${input})[0];
`;

/** Every parse of the input words under the extended grammar. */
export const parses = (input: string): ReadonlyArray<Value> =>
  runAmbAnswers(adjectiveProgram(input), "amb-depth-first-experiment", 1).answers;

/** The rendered parses of the input words. */
export const renderedParses = (input: string): ReadonlyArray<string> =>
  parses(input).map((value) => format(value));

export function ex_4_48(): string {
  return (
    "Adjectives join the noun phrase through parseModifiers: the noun " +
    "word closes the modifier list, or an adjective is consumed and " +
    "the recursion continues, so every alternative consumes input " +
    "and the search terminates. One sleepy cat and one quick brown " +
    "dog parse exactly once each."
  );
}
