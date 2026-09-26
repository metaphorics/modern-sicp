// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import {
  isApplication,
  setupEnvironment,
  taggedList,
} from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { nil } from "../../packages/ch4/src/list.js";
import { format, read } from "../../packages/ch4/src/read.js";
import {
  evalStringWithLoops,
  forToCombination,
  isFor,
  isWhile,
  whileToCombination,
} from "./ex_4_09.js";

const lastOf = (values: ReadonlyArray<Value>): Value => {
  const last = values[values.length - 1];
  return last === undefined ? nil : last;
};

const evalPrograms = (
  sources: ReadonlyArray<string>,
  env: Env,
): Effect.Effect<Value, EvaluationError> =>
  Effect.map(
    Effect.forEach(sources, (source) => evalStringWithLoops(source, env)),
    lastOf,
  );

describe("exercise 4.9: iteration constructs", () => {
  it.effect("a for loop sums like manual recursion", () =>
    Effect.gen(function* () {
      const loopEnv = yield* setupEnvironment();
      const loopTotal = yield* evalPrograms(
        ["(define total 0)", "(for (n 1 5) (set! total (+ total n)))", "total"],
        loopEnv,
      );
      expect(format(loopTotal)).toBe("15");

      const recursiveEnv = yield* setupEnvironment();
      const recursive = yield* evalPrograms(
        ["(define (sum-to n) (if (= n 0) 0 (+ n (sum-to (- n 1)))))", "(sum-to 5)"],
        recursiveEnv,
      );
      expect(format(recursive)).toBe("15");
    }),
  );

  it.effect("a while loop sums, including nested in a procedure body", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      const top = yield* evalPrograms(
        [
          "(define total 0)",
          "(define i 1)",
          "(while (< i 5) (set! total (+ total i)) (set! i (+ i 1)))",
          "total",
        ],
        env,
      );
      expect(format(top)).toBe("10");

      const nestedEnv = yield* setupEnvironment();
      const nested = yield* evalPrograms(
        [
          "(define total 0)",
          "(define (tally limit) (define i 1) (while (< i limit) (set! total (+ total i)) (set! i (+ i 1))) total)",
          "(tally 5)",
        ],
        nestedEnv,
      );
      expect(format(nested)).toBe("10");
    }),
  );

  it.effect("a while with a false predicate runs its body zero times", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      const runs = yield* evalPrograms(
        ["(define ran 0)", "(while false (set! ran (+ ran 1)))", "ran"],
        env,
      );
      expect(format(runs)).toBe("0");
      const value = yield* evalStringWithLoops("(while false 1)", env);
      expect(format(value)).toBe("()");
    }),
  );

  it.effect("for bounds are inclusive", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      const total = yield* evalPrograms(
        ["(define total 0)", "(for (n 3 3) (set! total (+ total n)))", "total"],
        env,
      );
      expect(format(total)).toBe("3");
    }),
  );

  it.effect("both constructs expand into applications of local loop lambdas", () =>
    Effect.sync(() => {
      const whileForm = read("(while (< i 5) (set! i (+ i 1)))");
      if (!isWhile(whileForm)) {
        throw new Error("while form not recognized");
      }
      expect(isApplication(whileToCombination(whileForm))).toBe(true);

      const forForm = read("(for (n 1 5) (set! t (+ t n)))");
      if (!isFor(forForm)) {
        throw new Error("for form not recognized");
      }
      const forExpansion = forToCombination(forForm);
      if (!isApplication(forExpansion)) {
        throw new Error("for expansion is not a combination");
      }
      if (!taggedList("lambda", forExpansion.head)) {
        throw new Error("expansion operator is not a lambda");
      }
    }),
  );
});
