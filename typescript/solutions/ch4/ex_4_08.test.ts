// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { Session } from "../../packages/ch4/src/01-metacircular.js";
import { format } from "../../packages/ch4/src/read.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import {
  bin,
  call,
  cond,
  type Expr,
  exprStmt,
  functionDecl,
  ident,
  lam,
  num,
  param,
  returnStmt,
} from "../../packages/ch4/src/syntax/ast.js";
import { noSpan } from "../../packages/ch4/src/syntax/diagnostics.js";
import {
  evalWithNamedLet,
  type LoopBinding,
  letNode,
  namedLetNode,
  namedLetToCall,
} from "./ex_4_08.js";

const envWith = (): { session: Session; env: Env } => {
  const session = new Session("core");
  return { session, env: session.globalEnv() };
};

/** The observable result: the rendered value, or the fault category. */
const shown = (outcome: Outcome): string =>
  outcome.tag === "ok" ? format(outcome.value) : `error:${outcome.error.tag}`;

/** Erases spans so two constructed trees can be compared structurally. */
const normalize = (value: object): unknown =>
  JSON.parse(JSON.stringify(value, (key, item) => (key === "span" ? null : item)));

const bindings = (...pairs: ReadonlyArray<readonly [string, Expr]>): LoopBinding[] =>
  pairs.map(([name, init]) => ({ name, init }));

const loopCall = (name: string, args: ReadonlyArray<Expr>): Expr => call(ident(name), args);

describe("exercise 4.8: named let", () => {
  it("Fibonacci 10 by named let is 55", () => {
    const { session, env } = envWith();
    const node = namedLetNode(
      "loop",
      bindings(["k", num(10)]),
      [
        exprStmt(
          cond(
            bin("<", ident("k"), num(2)),
            ident("k"),
            bin(
              "+",
              loopCall("loop", [bin("-", ident("k"), num(1))]),
              loopCall("loop", [bin("-", ident("k"), num(2))]),
            ),
          ),
        ),
      ],
      noSpan,
    );
    expect(shown(evalWithNamedLet(node, env, session))).toBe("55");
  });

  it("a factorial loop by named let is 120", () => {
    const { session, env } = envWith();
    const node = namedLetNode(
      "loop",
      bindings(["n", num(5)], ["acc", num(1)]),
      [
        exprStmt(
          cond(
            bin("===", ident("n"), num(0)),
            ident("acc"),
            loopCall("loop", [bin("-", ident("n"), num(1)), bin("*", ident("acc"), ident("n"))]),
          ),
        ),
      ],
      noSpan,
    );
    expect(shown(evalWithNamedLet(node, env, session))).toBe("120");
  });

  it("the transformation builds the local named function over the group's names", () => {
    const node = namedLetNode("loop", bindings(["n", num(0)]), [exprStmt(ident("n"))], noSpan);
    const manual = call(
      lam(
        [],
        [
          functionDecl("loop", [param("n")], [exprStmt(ident("n"))]),
          returnStmt(loopCall("loop", [num(0)])),
        ],
      ),
      [],
    );
    expect(normalize(namedLetToCall(node))).toEqual(normalize(manual));
  });

  it("plain let still evaluates in the same evaluator", () => {
    const { session, env } = envWith();
    const node = letNode(
      bindings(["x", num(3)], ["y", num(4)]),
      [exprStmt(bin("+", ident("x"), ident("y")))],
      noSpan,
    );
    expect(shown(evalWithNamedLet(node, env, session))).toBe("7");
  });

  it("a named let inside a procedure body recurs through its name", () => {
    const { session, env } = envWith();
    const inner = namedLetNode(
      "loop",
      bindings(["k", num(10)]),
      [
        exprStmt(
          cond(
            bin("<", ident("k"), num(2)),
            ident("k"),
            bin(
              "+",
              loopCall("loop", [bin("-", ident("k"), num(1))]),
              loopCall("loop", [bin("-", ident("k"), num(2))]),
            ),
          ),
        ),
      ],
      noSpan,
    );
    expect(
      shown(evalWithNamedLet(call(lam([], [returnStmt(namedLetToCall(inner))]), []), env, session)),
    ).toBe("55");
  });
});
