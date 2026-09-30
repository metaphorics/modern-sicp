// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { evaluate, lookupVariableValue, Session } from "../../packages/ch4/src/01-metacircular.js";
import { format, read } from "../../packages/ch4/src/read.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { admitSource } from "../../packages/ch4/src/syntax/check.js";
import { evalApplicationsFirst, evalCallTagged, readCallDialect } from "./ex_4_02.js";

const setup = `
const add = (a: number, b: number): number => a + b;
const mul = (a: number, b: number): number => a * b;
const square = (x: number): number => x * x;
const x = 3;
let n = 1;
`;

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

describe("exercise 4.2: dispatch order and the call-tagged dialect", () => {
  const inputs = ["square(6)", "n = 5", "n", "1 + 2 * 3", 'true ? "yes" : "no"', "missingName"];

  it("the two dispatch orders answer the fixed input suite identically", () => {
    for (const input of inputs) {
      const first = envWith(setup);
      const engine = envWith(setup);
      expect(shown(evalApplicationsFirst(read(input), first.env))).toBe(
        shown(evaluate(read(input), engine.env)),
      );
    }
  });

  it("the reordered order answers the declared values: square(6) is 36 and n = 5 writes 5", () => {
    const { env } = envWith(setup);
    expect(shown(evalApplicationsFirst(read("square(6)"), env))).toBe("36");
    expect(shown(evalApplicationsFirst(read("n = 5"), env))).toBe("5");
    expect(shown(evalApplicationsFirst(read("n"), env))).toBe("5");
    expect(shown(evalApplicationsFirst(read("1 + 2 * 3"), env))).toBe("7");
  });

  it("the declare case binds under both orders: const x = 3; leaves x + 1 as 4", () => {
    const first = envWith("const x = 3;");
    const engine = envWith("const x = 3;");
    expect(shown(evalApplicationsFirst(read("x + 1"), first.env))).toBe("4");
    expect(shown(evaluate(read("x + 1"), engine.env))).toBe("4");
  });

  it("the const-rebind fault category agrees under both orders", () => {
    const first = envWith("const x = 3;");
    const engine = envWith("const x = 3;");
    expect(shown(evalApplicationsFirst(read("x = 4"), first.env))).toBe("error:bad-operand");
    expect(shown(evaluate(read("x = 4"), engine.env))).toBe("error:bad-operand");
  });

  it("call(add, 1, 2) lowers to the ordinary application and answers 3", () => {
    const { env } = envWith(setup);
    expect(shown(evalCallTagged("call(add, 1, 2)", env))).toBe("3");
  });

  it("the plain dialect evaluates add(1, 2) to the same value", () => {
    const { env } = envWith(setup);
    expect(shown(evaluate(read("add(1, 2)"), env))).toBe("3");
  });

  it("lowering is structural: call(square, x) + 1 matches square(x) + 1", () => {
    const tagged = envWith(setup);
    const plain = envWith(setup);
    expect(shown(evalCallTagged("call(square, x) + 1", tagged.env))).toBe("10");
    expect(shown(evaluate(read("square(x) + 1"), plain.env))).toBe("10");
  });

  it("a lambda body keeps the dialect: call((x) => call(mul, x, x), 7) is 49", () => {
    const { env } = envWith(setup);
    expect(shown(evalCallTagged("call((x: number): number => call(mul, x, x), 7)", env))).toBe(
      "49",
    );
  });

  it("a bare application is rejected at read time as unknown-syntax", () => {
    const lowered = readCallDialect("add(1, 2)");
    expect(lowered.tag).toBe("error");
    if (lowered.tag === "error") {
      expect(lowered.error.tag).toBe("unknown-syntax");
    }
  });

  it("read-time rejection happens before any effect", () => {
    const { env } = envWith(`
let hits = 0;
const hit = (v: number): number => {
  hits = hits + 1;
  return v;
};
`);
    const lowered = readCallDialect("hit(1) + hit(2)");
    expect(lowered.tag).toBe("error");
    const hits = lookupVariableValue("hits", env);
    expect(hits.tag).toBe("ok");
    if (hits.tag === "ok") {
      expect(hits.value).toBe(0);
    }
  });
});
