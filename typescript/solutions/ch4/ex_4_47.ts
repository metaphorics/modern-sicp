// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.47: Louis's parse-verb-phrase. His consuming-first
 * ordering produces the ordinary parse, then recursion on exhausted
 * input never returns. Interchanging the alternatives recurses before
 * consuming the verb, so it cannot produce even the first parse. The
 * source programs are genuinely unbounded; SearchLimits observes a
 * finite prefix without adding a guest-language recursion cap.
 */
import {
  runAmbAnswers,
  type SearchLimits,
  type SearchRun,
} from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";

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
  const parsed = parseWord(words, ["the"]);
  const word = parsed[0];
  const rest = parsed[1];
  return [["article", word], rest];
};
const parseNoun = (words: string[]): Parsed => {
  const parsed = parseWord(words, ["cat"]);
  const word = parsed[0];
  const rest = parsed[1];
  return [["noun", word], rest];
};
const parseVerb = (words: string[]): Parsed => {
  const parsed = parseWord(words, ["eats"]);
  const word = parsed[0];
  const rest = parsed[1];
  return [["verb", word], rest];
};
const parsePrep = (words: string[]): Parsed => {
  const parsed = parseWord(words, ["with"]);
  const word = parsed[0];
  const rest = parsed[1];
  return [["prep", word], rest];
};
const parseNounPhrase = (words: string[]): Parsed => {
  const parsedArticle = parseArticle(words);
  const article = parsedArticle[0];
  const afterArticle = parsedArticle[1];
  const parsedNoun = parseNoun(afterArticle);
  const noun = parsedNoun[0];
  const rest = parsedNoun[1];
  return [["noun-phrase", article, noun], rest];
};
const parsePrepPhrase = (words: string[]): Parsed => {
  const parsedPrep = parsePrep(words);
  const prep = parsedPrep[0];
  const afterPrep = parsedPrep[1];
  const parsedNounPhrase = parseNounPhrase(afterPrep);
  const nounPhrase = parsedNounPhrase[0];
  const rest = parsedNounPhrase[1];
  return [["prep-phrase", prep, nounPhrase], rest];
};
`;

/** Louis's unbounded version: consuming alternative before recursion. */
export const louisSource = `${grammar}
const parseVerbPhrase = (words: string[]): Parsed =>
  choose(parseVerb(words), parseVerbWithPrep(words));
const parseVerbWithPrep = (words: string[]): Parsed => {
  const parsedVerbPhrase = parseVerbPhrase(words);
  const verbPhrase = parsedVerbPhrase[0];
  const afterVerb = parsedVerbPhrase[1];
  const parsedPrepPhrase = parsePrepPhrase(afterVerb);
  const prepPhrase = parsedPrepPhrase[0];
  const rest = parsedPrepPhrase[1];
  return [["verb-phrase", verbPhrase, prepPhrase], rest];
};
const parseSentence = (words: string[]): Parsed => {
  const parsedNounPhrase = parseNounPhrase(words);
  const nounPhrase = parsedNounPhrase[0];
  const afterNoun = parsedNounPhrase[1];
  const parsedVerbPhrase = parseVerbPhrase(afterNoun);
  const verbPhrase = parsedVerbPhrase[0];
  const rest = parsedVerbPhrase[1];
  require(rest.length === 0);
  return [["sentence", nounPhrase, verbPhrase], []];
};
parseSentence(["the", "cat", "eats"])[0];
`;

/** The unbounded interchanged order: recursion before consumption. */
export const interchangedSource = `${grammar}
const parseVerbPhrase = (words: string[]): Parsed =>
  choose(parseVerbWithPrep(words), parseVerb(words));
const parseVerbWithPrep = (words: string[]): Parsed => {
  const parsedVerbPhrase = parseVerbPhrase(words);
  const verbPhrase = parsedVerbPhrase[0];
  const afterVerb = parsedVerbPhrase[1];
  const parsedPrepPhrase = parsePrepPhrase(afterVerb);
  const prepPhrase = parsedPrepPhrase[0];
  const rest = parsedPrepPhrase[1];
  return [["verb-phrase", verbPhrase, prepPhrase], rest];
};
const parseSentence = (words: string[]): Parsed => {
  const parsedNounPhrase = parseNounPhrase(words);
  const nounPhrase = parsedNounPhrase[0];
  const afterNoun = parsedNounPhrase[1];
  const parsedVerbPhrase = parseVerbPhrase(afterNoun);
  const verbPhrase = parsedVerbPhrase[0];
  const rest = parsedVerbPhrase[1];
  require(rest.length === 0);
  return [["sentence", nounPhrase, verbPhrase], []];
};
parseSentence(["the", "cat", "eats"])[0];
`;

/** A bounded observation of an unbounded search, without guest truncation. */
export const searchPrefix = (source: string, limits: SearchLimits): SearchRun =>
  runAmbAnswers(source, "amb-depth-first-experiment", 1, limits);

/** A finite Louis prefix plus bounded retries into its unbounded continuation. */
export const louisPrefix = (maxAnswers: number, maxSteps: number): SearchRun =>
  searchPrefix(louisSource, { maxAnswers, maxSteps });

export function ex_4_47(): string {
  return (
    "Louis's consuming-first parser returns the ordinary parse, then " +
    "recurses forever on exhausted input when resumed. Interchanging " +
    "the alternatives recurses before consuming a verb and cannot " +
    "produce the first parse. The observation limit cuts off each " +
    "genuinely unbounded search at a finite prefix."
  );
}
