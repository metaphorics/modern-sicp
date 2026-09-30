// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { Session } from "../../packages/ch4/src/01-metacircular.js";
import { format } from "../../packages/ch4/src/read.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import {
  assign,
  bin,
  call,
  type Expr,
  exprStmt,
  ident,
  lam,
  num,
  param,
  returnStmt,
} from "../../packages/ch4/src/syntax/ast.js";
import { noSpan } from "../../packages/ch4/src/syntax/diagnostics.js";
import { evalWithLet, type LetBinding, letNode, letToCall } from "./ex_4_06.js";

const envWith = (): { session: Session; env: Env } => {
  const session = new Session("core");
  return { session, env: session.globalEnv() };
};

/** The observable result: the rendered value, or the fault category. */
const shown = (outcome: Outcome): string =>
  outcome.tag === "ok" ? format(outcome.value) : `error:${outcome.error.tag}`;

/** Erases spans so two constructed trees can be compared structurally. */
const normalize = (value: unknown): unknown =>
  JSON.parse(JSON.stringify(value, (key, item) => (key === "span" ? null : item)));

const bindings = (...pairs: ReadonlyArray<readonly [string, Expr]>): LetBinding[] =>
  pairs.map(([name, init]) => ({ name, init }));

describe("exercise 4.6: let as a derived expression", () => {
  it("a let evaluates as its combination: x = 3, y = 4 over x + y is 7", () => {
    const { session, env } = envWith();
    const node = letNode(
      bindings(["x", num(3)], ["y", num(4)]),
      [exprStmt(bin("+", ident("x"), ident("y")))],
      noSpan,
    );
    expect(shown(evalWithLet(node, env, session))).toBe("7");
  });

  it("letToCall builds the manual lambda call exactly", () => {
    const node = letNode(
      bindings(["x", num(3)], ["y", num(4)]),
      [exprStmt(bin("+", ident("x"), ident("y")))],
      noSpan,
    );
    const manual = call(
      lam([param("x"), param("y")], [exprStmt(bin("+", ident("x"), ident("y")))]),
      [num(3), num(4)],
    );
    expect(normalize(letToCall(node))).toEqual(normalize(manual));
  });

  it("a lowered let call in a procedure body evaluates through the derived form: 42", () => {
    const { session, env } = envWith();
    const body = lam(
      [],
      [
        returnStmt(
          letToCall(
            letNode(bindings(["x", num(2)]), [exprStmt(bin("*", ident("x"), num(21)))], noSpan),
          ),
        ),
      ],
    );
    expect(shown(evalWithLet(call(body, []), env, session))).toBe("42");
  });

  it("the let body runs as a sequence in the new frame: x = 10 then x is 10", () => {
    const { session, env } = envWith();
    const node = letNode(
      bindings(["x", num(1)]),
      [exprStmt(assign(ident("x"), num(10))), exprStmt(ident("x"))],
      noSpan,
    );
    expect(shown(evalWithLet(node, env, session))).toBe("10");
  });

  it("an outer let body evaluates a manually lowered inner let call", () => {
    const { session, env } = envWith();
    const inner = letNode(
      bindings(["y", num(4)]),
      [exprStmt(bin("+", ident("x"), ident("y")))],
      noSpan,
    );
    const outer = letNode(bindings(["x", num(3)]), [exprStmt(letToCall(inner))], noSpan);
    expect(shown(evalWithLet(outer, env, session))).toBe("7");
  });
});
