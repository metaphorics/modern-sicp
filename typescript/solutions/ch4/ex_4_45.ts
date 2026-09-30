// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.45: five parses. The demonstration parses the sentence
 * "the professor lectures to the student in the class with the cat"
 * once and collects the search's answers: exactly five parses, then
 * exhaustion. The five trees differ in where "in the class" and "with
 * the cat" attach. The input threads through the parsers as a
 * parameter — bindings fork per search branch, so backtracking
 * restores the unconsumed input exactly as the section's saved state
 * does.
 */
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";
import type { Value } from "../../packages/ch4/src/runtime/value.js";

/** The ambiguous grammar over one sentence, in the search experiment. */
export const ambiguousParserSource = `
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
  const parsed = parseWord(words, ["professor", "student", "class", "cat"]);
  const word = parsed[0];
  const rest = parsed[1];
  return [["noun", word], rest];
};
const parseVerb = (words: string[]): Parsed => {
  const parsed = parseWord(words, ["lectures"]);
  const word = parsed[0];
  const rest = parsed[1];
  return [["verb", word], rest];
};
const parsePrep = (words: string[]): Parsed => {
  const parsed = parseWord(words, ["to", "in", "with"]);
  const word = parsed[0];
  const rest = parsed[1];
  return [["prep", word], rest];
};
const parseSimpleNounPhrase = (words: string[]): Parsed => {
  const parsedArticle = parseArticle(words);
  const article = parsedArticle[0];
  const afterArticle = parsedArticle[1];
  const parsedNoun = parseNoun(afterArticle);
  const noun = parsedNoun[0];
  const rest = parsedNoun[1];
  return [["simple-noun-phrase", article, noun], rest];
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
const parseNounWithPrep = (phrase: Tree, words: string[]): Parsed => {
  const parsedPrepPhrase = parsePrepPhrase(words);
  const prepPhrase = parsedPrepPhrase[0];
  const rest = parsedPrepPhrase[1];
  return parseNounPhraseRest(["noun-phrase", phrase, prepPhrase], rest);
};
const parseNounPhraseRest = (phrase: Tree, words: string[]): Parsed => {
  const stopped: Parsed = [phrase, words];
  return choose(stopped, parseNounWithPrep(phrase, words));
};
const parseNounPhrase = (words: string[]): Parsed => {
  const parsedSimple = parseSimpleNounPhrase(words);
  return parseNounPhraseRest(parsedSimple[0], parsedSimple[1]);
};
const parseVerbWithPrep = (phrase: Tree, words: string[]): Parsed => {
  const parsedPrepPhrase = parsePrepPhrase(words);
  const prepPhrase = parsedPrepPhrase[0];
  const rest = parsedPrepPhrase[1];
  return parseVerbPhraseRest(["verb-phrase", phrase, prepPhrase], rest);
};
const parseVerbPhraseRest = (phrase: Tree, words: string[]): Parsed => {
  const stopped: Parsed = [phrase, words];
  return choose(parseVerbWithPrep(phrase, words), stopped);
};
const parseVerbPhrase = (words: string[]): Parsed => {
  const parsedVerb = parseVerb(words);
  return parseVerbPhraseRest(parsedVerb[0], parsedVerb[1]);
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
parseSentence([
  "the",
  "professor",
  "lectures",
  "to",
  "the",
  "student",
  "in",
  "the",
  "class",
  "with",
  "the",
  "cat",
])[0];
`;

/** The five parses, in search order. */
export const parses = (): ReadonlyArray<Value> =>
  runAmbAnswers(ambiguousParserSource, "amb-depth-first-experiment", 1).answers;

export function ex_4_45(): string {
  return (
    "The sentence parses exactly five ways and then the search exhausts. The five trees " +
    'differ in where "in the class" and "with the cat" attach: both phrases modify the ' +
    'verb phrase deepest first; "with the cat" attaches inside "in the class"\'s noun ' +
    'phrase; "in the class" attaches inside "to the student"\'s noun phrase; both ' +
    'attach at the student\'s noun phrase with "with the cat" outside; and both attach ' +
    'inside the student\'s noun phrase with "with the cat" nested inside "in the class".'
  );
}
