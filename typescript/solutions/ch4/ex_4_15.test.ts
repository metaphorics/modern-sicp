// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { format } from "../../packages/ch4/src/read.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { call, ident, lam, param, returnStmt } from "../../packages/ch4/src/syntax/ast.js";
import { haltingEnv } from "./ex_4_15.js";

/** The observable result: the rendered value, or the fault category and detail. */
const shown = (outcome: Outcome): string => {
  if (outcome.tag === "ok") {
    return format(outcome.value);
  }
  return outcome.error.tag === "bad-operand"
    ? `error:bad-operand:${outcome.error.detail}`
    : `error:${outcome.error.tag}`;
};

const identity = lam(
  [param("u")],
  [returnStmt({ tag: "variable", name: "u", span: { start: 0, end: 0, line: 1, column: 1 } })],
);

describe("exercise 4.15: the halting diagonal, executed", () => {
  it("the true oracle's answer for the diagonal call is false: the run exhausts its fuel", () => {
    const { session, env, spent } = haltingEnv(true, 500);
    const outcome = session.evaluate(call(ident("tryProgram"), [ident("tryProgram")]), env);
    expect(shown(outcome)).toBe("error:bad-operand:out of fuel");
    expect(spent()).toBe(501);
  });

  it("the false oracle's answer is wrong too: the run halts under the limit", () => {
    const { session, env, spent } = haltingEnv(false, 500);
    const outcome = session.evaluate(call(ident("tryProgram"), [identity]), env);
    expect(shown(outcome)).toBe(JSON.stringify("halted"));
    expect(spent() < 500).toBe(true);
  });

  it("under the true oracle a halting procedure still exhausts the fuel", () => {
    const { session, env, spent } = haltingEnv(true, 500);
    const outcome = session.evaluate(call(ident("tryProgram"), [identity]), env);
    expect(shown(outcome)).toBe("error:bad-operand:out of fuel");
    expect(spent()).toBe(501);
  });
});
