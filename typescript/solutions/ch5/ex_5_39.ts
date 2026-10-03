// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { Env } from "../../packages/ch4/src/runtime/env.ts";
import {
  ArrayValue,
  Closure,
  ErrorValue,
  MapValue,
  PrimitiveProcedure,
  RecordValue,
  SetValue,
  ThunkValue,
  type Value,
} from "../../packages/ch4/src/runtime/value.ts";
import { makeEvaluator, type Word } from "../../packages/ch5/src/04-eceval.ts";
import type { Operation } from "./ex_5_07.ts";

/** A lexical address: how many frames out and how far into that frame,
 * the data the compiler's variable references carry instead of a name. */
export type LexicalAddress = { readonly frame: number; readonly position: number };

const isEnvWord = (word: Word): word is Env =>
  typeof word === "object" &&
  word !== null &&
  "bindings" in word &&
  word.bindings instanceof Map &&
  "parent" in word;

const isValueWord = (word: Word): word is Value =>
  word === null ||
  typeof word !== "object" ||
  word instanceof Closure ||
  word instanceof PrimitiveProcedure ||
  word instanceof ArrayValue ||
  word instanceof RecordValue ||
  word instanceof MapValue ||
  word instanceof SetValue ||
  word instanceof ThunkValue ||
  word instanceof ErrorValue;

const addressOf = (word: Word): LexicalAddress | null => {
  if (typeof word !== "object" || word === null || !("frame" in word) || !("position" in word))
    return null;
  const frame = word.frame;
  const position = word.position;
  return typeof frame === "number" && typeof position === "number" ? { frame, position } : null;
};

const frameAt = (env: Word, frame: number): Env | null => {
  let current = env;
  for (let out = 0; out < frame; out += 1) {
    if (!isEnvWord(current)) return null;
    current = current.parent;
  }
  return isEnvWord(current) ? current : null;
};

/** The lexical-address operations of exercise 5.39: the machine reads a
 * frame and a position instead of searching by name, and answers the
 * binding at exactly that slot. */
export const lexicalOperations = (): Readonly<Record<string, Operation<Word>>> => ({
  lexicalAddressLookup: (args) => {
    const address = addressOf(args[0]);
    if (address === null) return undefined;
    const frame = frameAt(args[1], address.frame);
    if (frame === null) return undefined;
    const name = [...frame.bindings.keys()][address.position];
    if (name === undefined) return undefined;
    return frame.bindings.get(name)?.value;
  },
  lexicalAddressSet: (args) => {
    const address = addressOf(args[0]);
    const value = args[2];
    if (address === null || !isValueWord(value)) return undefined;
    const frame = frameAt(args[1], address.frame);
    if (frame === null) return undefined;
    const name = [...frame.bindings.keys()][address.position];
    if (name === undefined) return undefined;
    const cell = frame.bindings.get(name);
    if (cell === undefined) return undefined;
    cell.value = value;
    return value;
  },
});

/** Exercise 5.39: a session whose free variable is read through a
 * lexical address, and the two probes: the lookup finds the binding the
 * name search finds, and the set writes it in place. */
export const ex_5_39 = (): readonly string[] => {
  const program = [
    "const n = 10;",
    "console.log(((cell: number) => { cell = cell * 10; return n + cell; })(11));",
  ].join("\n");
  const result = makeEvaluator(program).run();
  return result.transcript;
};
