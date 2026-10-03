// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { listOfValues, lookupVariableValue } from "../../packages/ch4/src/01-metacircular.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { isArrayValue, type Value } from "../../packages/ch4/src/runtime/value.js";
import { listOfValuesLr, listOfValuesRl, recorderEnv, recorderOperands } from "./ex_4_01.js";

const arrayItems = (outcome: Outcome): ReadonlyArray<Value> => {
  expect(outcome.tag).toBe("ok");
  if (outcome.tag !== "ok" || !isArrayValue(outcome.value)) {
    throw new Error("expected an array outcome");
  }
  return outcome.value.items;
};

describe("exercise 4.1: evaluation order of the operands", () => {
  it("the left-to-right variant logs operands in source order", () => {
    const { session, env } = recorderEnv();
    const args = listOfValuesLr(recorderOperands(), env, session);
    expect(session.transcript).toEqual(["a", "b", "c"]);
    expect(arrayItems(args)).toEqual(["a", "b", "c"]);
  });

  it("the right-to-left variant logs the reversed order but builds the same argument list", () => {
    const { session, env } = recorderEnv();
    const args = listOfValuesRl(recorderOperands(), env, session);
    expect(session.transcript).toEqual(["c", "b", "a"]);
    expect(arrayItems(args)).toEqual(["a", "b", "c"]);
  });

  it("the engine's listOfValues follows the specification's left-to-right order", () => {
    const { env } = recorderEnv();
    const args = listOfValues(recorderOperands(), env);
    expect(arrayItems(args)).toEqual(["a", "b", "c"]);
    const recorded = lookupVariableValue("recorded", env);
    expect(recorded.tag).toBe("ok");
    if (recorded.tag === "ok" && isArrayValue(recorded.value)) {
      expect(recorded.value.items).toEqual(["a", "b", "c"]);
    }
  });
});
