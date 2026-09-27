// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.1: the book asks for both evaluation orders because Lisp
 * leaves the order of operand evaluation unspecified. This host fixes it:
 * JavaScript evaluates call arguments and Array operations left to right,
 * so the edition's `listOfValues` is the book's no-defer left-to-right
 * version, written with cons. Both book-faithful variants below are plain
 * explicit recursion; they build the same argument list and differ only in
 * which operand's effects land first.
 */
import { Effect } from "effect";

import {
  evalString,
  evaluate,
  firstOperand,
  noOperands,
  restOperands,
  setupEnvironment,
} from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { cons, type List, nil } from "../../packages/ch4/src/list.js";

/** The book's left-to-right list-of-values: evaluate this operand, then
 * the rest, then cons. */
export const listOfValuesLr = (
  exps: List<Value>,
  env: Env,
): Effect.Effect<List<Value>, EvaluationError> =>
  noOperands(exps)
    ? Effect.succeed(nil)
    : Effect.flatMap(evaluate(firstOperand(exps), env), (first) =>
        Effect.map(listOfValuesLr(restOperands(exps), env), (rest) => cons(first, rest)),
      );

/** The book's right-to-left variant: evaluate the rest first, then this
 * operand, then cons. */
export const listOfValuesRl = (
  exps: List<Value>,
  env: Env,
): Effect.Effect<List<Value>, EvaluationError> =>
  noOperands(exps)
    ? Effect.succeed(nil)
    : Effect.flatMap(listOfValuesRl(restOperands(exps), env), (rest) =>
        Effect.map(evaluate(firstOperand(exps), env), (first) => cons(first, rest)),
      );

/** A global environment in which `sequence` is an object-language list and
 * `(rec tag)` appends the tag to it and returns the tag, so the order in
 * which operands are evaluated is observable from the evaluated program. */
export const recorderEnv = (): Effect.Effect<Env, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) =>
    Effect.flatMap(evalString("(define sequence '())", env), () =>
      Effect.map(
        evalString("(define (rec tag) (set! sequence (append sequence (list tag))) tag)", env),
        () => env,
      ),
    ),
  );

export function ex_4_01(): string {
  return (
    "Lisp leaves the order of operand evaluation unspecified, which is why the book asks " +
    "for both orders; this edition runs on JavaScript, which evaluates call arguments and " +
    "Array operations left to right, so the host already fixes the order. The edition's " +
    "listOfValues is the book's no-defer left-to-right version written with cons, and it " +
    "agrees operand for operand with the book-faithful listOfValuesLr; listOfValuesRl " +
    "evaluates the same operands and builds the same argument list, but its effects land " +
    "right to left."
  );
}
