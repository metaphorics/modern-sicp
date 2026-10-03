// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { Session } from "../../packages/ch4/src/01-metacircular.js";
import { format, read } from "../../packages/ch4/src/read.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { ok } from "../../packages/ch4/src/runtime/errors.js";
import { admitSource } from "../../packages/ch4/src/syntax/check.js";
import { evalDataDirected, makeFormTable } from "./ex_4_03.js";

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

const setup = `
const square = (x: number): number => x * x;
const classify = (n: number): string => (n < 0 ? "neg" : "pos");
let n = 1;
`;

describe("exercise 4.3: data-directed dispatch", () => {
  it("forms and application share the evaluator: square(6) * square(7) is 1764", () => {
    const { session, env } = envWith(setup);
    expect(shown(evalDataDirected(read("square(6) * square(7)"), env, session))).toBe("1764");
  });

  it("the conditional operation runs its installed form: 1 === 2 ? 111 : 222 is 222", () => {
    const { session, env } = envWith(setup);
    expect(shown(evalDataDirected(read("1 === 2 ? 111 : 222"), env, session))).toBe("222");
  });

  it("assignment writes through the installed operation", () => {
    const { session, env } = envWith(setup);
    expect(shown(evalDataDirected(read("n = 5"), env, session))).toBe("5");
    expect(shown(evalDataDirected(read("n"), env, session))).toBe("5");
  });

  it("assigning a const binding is a bad-operand fault and leaves the value", () => {
    const { session, env } = envWith("const x = 10;");
    expect(shown(evalDataDirected(read("x = 20"), env, session))).toBe("error:bad-operand");
    expect(shown(evalDataDirected(read("x"), env, session))).toBe("10");
  });

  it("an arrow builds a procedure and the application path applies it: 42 and 3", () => {
    const { session, env } = envWith(setup);
    expect(shown(evalDataDirected(read("((x: number): number => x * 2)(21)"), env, session))).toBe(
      "42",
    );
    expect(shown(evalDataDirected(read("(() => { 1; 2; return 3; })()"), env, session))).toBe("3");
  });

  it("nested forms in a procedure body dispatch through the table: classify(-5)", () => {
    const { session, env } = envWith(setup);
    expect(shown(evalDataDirected(read("classify(-5)"), env, session))).toBe(JSON.stringify("neg"));
    expect(shown(evalDataDirected(read("classify(5)"), env, session))).toBe(JSON.stringify("pos"));
  });

  it("an uninstalled call tag runs the application path: square(9) is 81", () => {
    const { session, env } = envWith(setup);
    expect(shown(evalDataDirected(read("square(9)"), env, session))).toBe("81");
  });

  it("an uninstalled data tag runs the ordinary evaluator: [1, 2, 3]", () => {
    const { session, env } = envWith(setup);
    expect(shown(evalDataDirected(read("[1, 2, 3]"), env, session))).toBe("[1, 2, 3]");
  });

  it("one more put installs a new form, and a second put overwrites it", () => {
    const { session, env } = envWith(setup);
    const table = makeFormTable();
    table.put("unary", () => ok(4242));
    expect(shown(evalDataDirected(read("- 1"), env, session, table))).toBe("4242");
  });
});
