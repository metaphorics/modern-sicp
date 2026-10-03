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
  type Expr,
  exprStmt,
  ident,
  lam,
  num,
  returnStmt,
} from "../../packages/ch4/src/syntax/ast.js";
import { noSpan } from "../../packages/ch4/src/syntax/diagnostics.js";
import {
  evalWithLetStar,
  type LetStarBinding,
  letNode,
  letStarNode,
  letToCall,
  sequentialToNested,
} from "./ex_4_07.js";

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

const bindings = (...pairs: ReadonlyArray<readonly [string, Expr]>): LetStarBinding[] =>
  pairs.map(([name, init]) => ({ name, init }));

describe("exercise 4.7: let* as nested lets", () => {
  it("the book's example returns 39", () => {
    const { session, env } = envWith();
    const node = letStarNode(
      bindings(
        ["x", num(3)],
        ["y", bin("+", ident("x"), num(2))],
        ["z", bin("+", bin("+", ident("x"), ident("y")), num(5))],
      ),
      [exprStmt(bin("*", ident("x"), ident("z")))],
      noSpan,
    );
    expect(shown(evalWithLetStar(node, env, session))).toBe("39");
  });

  it("sequentialToNested peels an outer let and lowers its inner body to a call", () => {
    const node = letStarNode(
      bindings(["x", num(3)], ["y", bin("+", ident("x"), num(2))]),
      [exprStmt(bin("*", ident("x"), ident("y")))],
      noSpan,
    );
    const nested = letNode(
      bindings(["x", num(3)]),
      [
        exprStmt(
          letToCall(
            letNode(
              bindings(["y", bin("+", ident("x"), num(2))]),
              [exprStmt(bin("*", ident("x"), ident("y")))],
              noSpan,
            ),
          ),
        ),
      ],
      noSpan,
    );
    expect(normalize(sequentialToNested(node))).toEqual(normalize(nested));
  });

  it("each initializer sees the previous bindings", () => {
    const { session, env } = envWith();
    const node = letStarNode(
      bindings(["x", num(3)], ["y", bin("+", ident("x"), num(1))]),
      [exprStmt(bin("+", ident("x"), ident("y")))],
      noSpan,
    );
    expect(shown(evalWithLetStar(node, env, session))).toBe("7");
  });

  it("empty bindings leave only the body", () => {
    const { session, env } = envWith();
    const node = letStarNode([], [exprStmt(num(42))], noSpan);
    expect(shown(evalWithLetStar(node, env, session))).toBe("42");
  });

  it("a sequential binding lowered to calls in a procedure body answers 8", () => {
    const { session, env } = envWith();
    const body = lam(
      [],
      [
        returnStmt(
          letToCall(
            sequentialToNested(
              letStarNode(
                bindings(["a", num(2)], ["b", bin("*", ident("a"), num(3))]),
                [exprStmt(bin("+", ident("a"), ident("b")))],
                noSpan,
              ),
            ),
          ),
        ),
      ],
    );
    expect(shown(evalWithLetStar(call(body, []), env, session))).toBe("8");
  });
});
