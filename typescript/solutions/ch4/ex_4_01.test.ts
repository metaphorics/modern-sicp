// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import {
  evalString,
  isApplication,
  listOfValues,
  operands,
} from "../../packages/ch4/src/01-metacircular.js";
import type { Value } from "../../packages/ch4/src/core.js";
import type { List } from "../../packages/ch4/src/list.js";
import { format, read } from "../../packages/ch4/src/read.js";
import { listOfValuesLr, listOfValuesRl, recorderEnv } from "./ex_4_01.js";

/** The operand expressions of a call whose order is observable: each
 * operand appends its tag to the object-language `sequence` list. */
const observableOperands = (): List<Value> => {
  const call = read("((lambda (x y z) (list x y z)) (rec 'a) (rec 'b) (rec 'c))");
  if (!isApplication(call)) {
    throw new Error(`expected an application, got ${format(call)}`);
  }
  return operands(call);
};

describe("exercise 4.1: evaluation order of the operands", () => {
  it.effect("the left-to-right variant evaluates operands in source order", () =>
    Effect.gen(function* () {
      const env = yield* recorderEnv();
      const args = yield* listOfValuesLr(observableOperands(), env);
      const sequence = yield* evalString("sequence", env);
      expect(format(sequence)).toBe("(a b c)");
      expect(format(args)).toBe("(a b c)");
    }),
  );

  it.effect("the right-to-left variant evaluates operands in reverse source order", () =>
    Effect.gen(function* () {
      const env = yield* recorderEnv();
      const args = yield* listOfValuesRl(observableOperands(), env);
      const sequence = yield* evalString("sequence", env);
      expect(format(sequence)).toBe("(c b a)");
      expect(format(args)).toBe("(a b c)");
    }),
  );

  it.effect("the module's listOfValues agrees with the left-to-right variant", () =>
    Effect.gen(function* () {
      const env = yield* recorderEnv();
      const args = yield* listOfValues(observableOperands(), env);
      const sequence = yield* evalString("sequence", env);
      expect(format(sequence)).toBe("(a b c)");
      expect(format(args)).toBe("(a b c)");
    }),
  );
});
