// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.21: recursion without define. The self-application trick passes
 * a procedure to itself as an argument, so the body can recurse without any
 * bound name. Part (a) evaluates the book's factorial expression and a
 * Fibonacci analog; part (b) fills in the missing operands of the mutually
 * recursive even?/odd? procedure: each recursive call passes the two
 * procedure arguments along unchanged.
 */
import { Effect } from "effect";

import { evalString, setupEnvironment } from "../../packages/ch4/src/01-metacircular.js";
import type { Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";

/** The book's expression: 10 factorial by passing the procedure to itself. */
export const factorialSource =
  "((lambda (n) ((lambda (fact) (fact fact n)) (lambda (ft k) (if (= k 1) 1 (* k (ft ft (- k 1))))))) 10)";

/** The analogous expression: fib 10 by the same self-application trick. */
export const fibonacciSource =
  "((lambda (n) ((lambda (fib) (fib fib n)) (lambda (fb k) (if (< k 2) k (+ (fb fb (- k 1)) (fb fb (- k 2))))))) 10)";

/** The completed even?/odd? f: the ?? operands become ev? od? (- n 1). */
export const evenOddSource =
  "(define (f x) ((lambda (even? odd?) (even? even? odd? x)) (lambda (ev? od? n) (if (= n 0) true (od? ev? od? (- n 1)))) (lambda (ev? od? n) (if (= n 0) false (ev? ev? od? (- n 1))))))";

const runIn = (sources: ReadonlyArray<string>): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) =>
    Effect.flatMap(
      Effect.forEach(sources, (source) => evalString(source, env)),
      (values) => {
        const last = values[values.length - 1];
        return last !== undefined
          ? Effect.succeed(last)
          : Effect.die(new Error("no forms evaluated"));
      },
    ),
  );

/** Evaluates the factorial expression with the module's evaluator. */
export const factorialValue = (): Effect.Effect<Value, EvaluationError> => runIn([factorialSource]);

/** Evaluates the Fibonacci analog. */
export const fibonacciValue = (): Effect.Effect<Value, EvaluationError> => runIn([fibonacciSource]);

/** Defines the completed f and applies it to n. */
export const evenOddValue = (n: number): Effect.Effect<Value, EvaluationError> =>
  runIn([evenOddSource, `(f ${n})`]);

export function ex_4_21(): string {
  return (
    "The trick is self-application: the inner binding names a procedure that takes " +
    "itself as its first argument, so the body can recurse by passing the procedure " +
    "along to itself, and no define or letrec is ever needed. The factorial expression " +
    "evaluates to 3628800; the Fibonacci analog evaluates to 55; and in the completed " +
    "even?/odd? procedure each missing operand list is the pair of procedures plus the " +
    "decremented count, ev? od? (- n 1), because the only way a nameless lambda can " +
    "recurse is to receive itself again as an argument."
  );
}
