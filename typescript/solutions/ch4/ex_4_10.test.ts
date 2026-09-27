// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { setupEnvironment } from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { nil } from "../../packages/ch4/src/list.js";
import { format } from "../../packages/ch4/src/read.js";
import { evalStringIn, makeBackwardsTable, makeSchemeTable, type SyntaxTable } from "./ex_4_10.js";

const lastOf = (values: ReadonlyArray<Value>): Value => {
  const last = values[values.length - 1];
  return last === undefined ? nil : last;
};

const evalProgramsIn = (
  syntax: SyntaxTable,
  sources: ReadonlyArray<string>,
  env: Env,
): Effect.Effect<Value, EvaluationError> =>
  Effect.map(
    Effect.forEach(sources, (source) => evalStringIn(syntax, source, env)),
    lastOf,
  );

const failureOf = (run: Effect.Effect<Value, EvaluationError>): Effect.Effect<EvaluationError> =>
  Effect.flatMap(Effect.result(run), (outcome) =>
    outcome._tag === "Failure"
      ? Effect.succeed(outcome.failure)
      : Effect.die(new Error("expected a failure")),
  );

const unboundName = (error: EvaluationError): string =>
  error._tag === "UnboundVariable" ? error.name : "<not an UnboundVariable>";

describe("exercise 4.10: syntax table", () => {
  it.effect("the same square program runs under both syntaxes", () =>
    Effect.gen(function* () {
      const schemeEnv = yield* setupEnvironment();
      const scheme = yield* evalProgramsIn(
        makeSchemeTable(),
        ["(define square (lambda (x) (* x x)))", "(square 7)"],
        schemeEnv,
      );
      expect(format(scheme)).toBe("49");

      const backwardsEnv = yield* setupEnvironment();
      const backwards = yield* evalProgramsIn(
        makeBackwardsTable(),
        ["(enifed square (adbmal (x) (* x x)))", "(square 7)"],
        backwardsEnv,
      );
      expect(format(backwards)).toBe("49");
    }),
  );

  it.effect("if works in both spellings", () =>
    Effect.gen(function* () {
      const schemeEnv = yield* setupEnvironment();
      const scheme = yield* evalProgramsIn(makeSchemeTable(), ["(if (> 3 2) 1 0)"], schemeEnv);
      expect(format(scheme)).toBe("1");

      const backwardsEnv = yield* setupEnvironment();
      const backwards = yield* evalProgramsIn(
        makeBackwardsTable(),
        ["(fi (> 3 2) 1 0)"],
        backwardsEnv,
      );
      expect(format(backwards)).toBe("1");
    }),
  );

  it.effect("a tag from the other table is an unbound variable", () =>
    Effect.gen(function* () {
      const schemeEnv = yield* setupEnvironment();
      const schemeFailure = yield* failureOf(
        evalStringIn(makeSchemeTable(), "(fi 1 2)", schemeEnv),
      );
      expect(schemeFailure._tag).toBe("UnboundVariable");
      expect(unboundName(schemeFailure)).toBe("fi");

      const backwardsEnv = yield* setupEnvironment();
      const backwardsFailure = yield* failureOf(
        evalStringIn(makeBackwardsTable(), "(if 1 2)", backwardsEnv),
      );
      expect(backwardsFailure._tag).toBe("UnboundVariable");
      expect(unboundName(backwardsFailure)).toBe("if");
    }),
  );
});
