// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  defineVariableValue,
  lookupVariableValue,
  Session,
} from "../../packages/ch4/src/01-metacircular.js";
import { format, read } from "../../packages/ch4/src/read.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { ok } from "../../packages/ch4/src/runtime/errors.js";
import { makePrimitive } from "../../packages/ch4/src/runtime/value.js";
import {
  assign,
  bool,
  call,
  ident,
  lam,
  num,
  param,
  returnStmt,
} from "../../packages/ch4/src/syntax/ast.js";
import { admitSource } from "../../packages/ch4/src/syntax/check.js";
import { allOf, allToConditional, anyOf, anyToConditional, evalWithAllAny } from "./ex_4_04.js";

/** A session running one admitted setup unit in its global frame. */
const envWith = (source: string): { session: Session; env: Env } => {
  const admission = admitSource(source);
  if (!admission.ok) {
    throw new Error(`setup source must admit: ${admission.diagnostics[0]?.construct ?? "reject"}`);
  }
  const session = new Session("core");
  const env = session.globalEnv();
  session.execSequence(admission.program, env);
  return { session, env };
};

/** The observable result: the rendered value, or the fault category. */
const shown = (outcome: Outcome): string =>
  outcome.tag === "ok" ? format(outcome.value) : `error:${outcome.error.tag}`;

const hitsAfter = (env: Env): number => {
  const hits = lookupVariableValue("hits", env);
  return hits.tag === "ok" && typeof hits.value === "number" ? hits.value : -1;
};

/** A counting probe: returns its operand and counts its own evaluation. */
const withProbe = (env: Env): { evaluations: () => number } => {
  let calls = 0;
  defineVariableValue(
    "probe",
    makePrimitive("probe", (args) => {
      calls += 1;
      return ok(args[0]);
    }),
    env,
  );
  return { evaluations: () => calls };
};

const probe = (value: boolean): ReturnType<typeof call> => call(ident("probe"), [bool(value)]);

describe("exercise 4.4: all and any as special forms", () => {
  it("empty forms: all() is true and any() is false", () => {
    const { session, env } = envWith("let hits = 0;");
    expect(shown(evalWithAllAny(allOf([]), env, session))).toBe("true");
    expect(shown(evalWithAllAny(anyOf([]), env, session))).toBe("false");
  });

  it("all is true only when every operand is true", () => {
    const { session, env } = envWith("let hits = 0;");
    expect(shown(evalWithAllAny(allOf([bool(true), bool(true), bool(true)]), env, session))).toBe(
      "true",
    );
    expect(shown(evalWithAllAny(allOf([bool(true), bool(false), bool(true)]), env, session))).toBe(
      "false",
    );
    expect(shown(evalWithAllAny(anyOf([bool(false), bool(false), bool(true)]), env, session))).toBe(
      "true",
    );
    expect(shown(evalWithAllAny(anyOf([bool(false), bool(false)]), env, session))).toBe("false");
  });

  it("non-boolean operands are a bad-operand fault, not a value", () => {
    const { session, env } = envWith("let hits = 0;");
    expect(shown(evalWithAllAny(allOf([num(1), num(2), num(3)]), env, session))).toBe(
      "error:bad-operand",
    );
    expect(shown(evalWithAllAny(anyOf([num(7), bool(true)]), env, session))).toBe(
      "error:bad-operand",
    );
  });

  it("effects stop at the decisive operand", () => {
    const { session, env } = envWith("let hits = 0;");
    expect(
      shown(evalWithAllAny(allOf([bool(false), assign(ident("hits"), num(99))]), env, session)),
    ).toBe("false");
    expect(hitsAfter(env)).toBe(0);
    expect(
      shown(evalWithAllAny(anyOf([bool(true), assign(ident("hits"), num(99))]), env, session)),
    ).toBe("true");
    expect(hitsAfter(env)).toBe(0);
  });

  it("every reached operand runs before a fault: the assign runs, then all faults", () => {
    const { session, env } = envWith("let hits = 0;");
    expect(
      shown(evalWithAllAny(allOf([assign(ident("hits"), num(99)), bool(true)]), env, session)),
    ).toBe("error:bad-operand");
    expect(hitsAfter(env)).toBe(99);
  });

  it("evaluation counts stop at the decisive operand", () => {
    const { session, env } = envWith("let hits = 0;");
    const probe1 = withProbe(env);
    expect(
      shown(evalWithAllAny(allOf([probe(true), probe(false), probe(true)]), env, session)),
    ).toBe("false");
    expect(probe1.evaluations()).toBe(2);
    const probe2 = withProbe(env);
    expect(
      shown(evalWithAllAny(anyOf([probe(false), probe(false), probe(true)]), env, session)),
    ).toBe("true");
    expect(probe2.evaluations()).toBe(3);
  });

  it("derived all/any conditionals can be used inside procedure bodies", () => {
    const { session, env } = envWith("let hits = 0;");
    const both = lam(
      [param("a"), param("b")],
      [returnStmt(allToConditional([ident("a"), ident("b")]))],
    );
    const fallback = lam([], [returnStmt(anyToConditional([bool(false), bool(true)]))]);
    expect(shown(evalWithAllAny(call(both, [bool(true), bool(true)]), env, session))).toBe("true");
    expect(shown(evalWithAllAny(call(both, [bool(true), bool(false)]), env, session))).toBe(
      "false",
    );
    expect(shown(evalWithAllAny(call(fallback, []), env, session))).toBe("true");
  });

  it("the derived route reaches the same operands but does not fault", () => {
    const { session, env } = envWith("let hits = 0;");
    expect(shown(evalWithAllAny(allOf([num(1), num(2)]), env, session))).toBe("error:bad-operand");
    expect(shown(session.evaluate(allToConditional([num(1), num(2)]), env))).toBe("false");
    const probe1 = withProbe(env);
    expect(
      shown(session.evaluate(anyToConditional([probe(false), probe(false), probe(true)]), env)),
    ).toBe("true");
    expect(probe1.evaluations()).toBe(3);
  });
});
