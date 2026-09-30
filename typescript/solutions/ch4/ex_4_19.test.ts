// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { defineVariableValue, Session } from "../../packages/ch4/src/01-metacircular.js";
import { child, type Env, makeCell } from "../../packages/ch4/src/runtime/env.js";
import type { Completion, GuestError, Outcome } from "../../packages/ch4/src/runtime/errors.js";
import {
  bin,
  type Decl,
  ident,
  num,
  returnStmt,
  type Stmt,
  varDecl,
} from "../../packages/ch4/src/syntax/ast.js";
import { noSpan } from "../../packages/ch4/src/syntax/diagnostics.js";
import { execScanned, scanOutDefinitions } from "./ex_4_16.js";
import { applyRuleSequential, execSimultaneous } from "./ex_4_19.js";

const session = (): { session: Session; env: Env } => {
  const engine = new Session("core");
  const env = engine.globalEnv();
  defineVariableValue("a", 1, env);
  return { session: engine, env };
};

const callFrame = (env: Env): Env => {
  const frame = child(env);
  frame.bindings.set("x", makeCell(10, true));
  return frame;
};

const shownOutcome = (outcome: Outcome): string =>
  outcome.tag === "ok" ? String(outcome.value) : shownError(outcome.error);

const shownError = (error: GuestError): string =>
  error.tag === "tdz-access"
    ? `error:${error.tag}:${error.name}`
    : error.tag === "bad-operand"
      ? `error:${error.tag}:${error.detail}`
      : `error:${error.tag}`;

const shownCompletion = (completion: Completion): string =>
  completion.tag === "normal" || completion.tag === "return"
    ? String(completion.value)
    : completion.tag === "error"
      ? shownError(completion.error)
      : `completion:${completion.tag}`;

const debateBody = (): Array<Decl | Stmt> => [
  varDecl("const", "b", bin("+", ident("a"), ident("x")), null, noSpan),
  varDecl("const", "a", num(5), null, noSpan),
  returnStmt(bin("+", ident("a"), ident("b")), noSpan),
];

describe("exercise 4.19: internal definition scoping", () => {
  it("the sequential rule settles on Alyssa: the sibling read is tdz-access", () => {
    const { session: engine, env } = session();
    expect(shownOutcome(applyRuleSequential(debateBody(), [10], ["x"], env, engine))).toBe(
      "error:tdz-access:a",
    );
  });

  it("the scanned rule answers the same fault", () => {
    const { session: engine, env } = session();
    const frame = callFrame(env);
    expect(shownCompletion(execScanned(scanOutDefinitions(debateBody()), frame, engine))).toBe(
      "error:tdz-access:a",
    );
  });

  it("Eva's rule answers 20", () => {
    const { session: engine, env } = session();
    const frame = callFrame(env);
    expect(shownCompletion(execSimultaneous(debateBody(), frame, engine))).toBe("20");
  });

  it("a force cycle is a circular-definition error", () => {
    const { session: engine, env } = session();
    const frame = callFrame(env);
    const cyclic: Array<Decl | Stmt> = [
      varDecl("const", "a", ident("b"), null, noSpan),
      varDecl("const", "b", ident("a"), null, noSpan),
      returnStmt(ident("a"), noSpan),
    ];
    const completion = execSimultaneous(cyclic, frame, engine);
    expect(shownCompletion(completion)).toBe("error:bad-operand:circular definition: a");
  });
});
