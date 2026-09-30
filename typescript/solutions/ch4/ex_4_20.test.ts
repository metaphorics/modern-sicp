// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { Session } from "../../packages/ch4/src/01-metacircular.js";
import { format } from "../../packages/ch4/src/read.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import {
  bin,
  bool,
  call,
  cond,
  exprStmt,
  ident,
  lam,
  num,
  param,
  returnStmt,
  varDecl,
} from "../../packages/ch4/src/syntax/ast.js";
import { noSpan } from "../../packages/ch4/src/syntax/diagnostics.js";
import {
  evalWithRecursive,
  type RecursiveBinding,
  recursiveNode,
  recursiveToCall,
} from "./ex_4_20.js";

const session = (): { session: Session; env: ReturnType<Session["globalEnv"]> } => {
  const engine = new Session("core");
  return { session: engine, env: engine.globalEnv() };
};

const shown = (outcome: Outcome): string =>
  outcome.tag === "ok" ? format(outcome.value) : `error:${outcome.error.tag}`;

const normalize = (value: object): unknown =>
  JSON.parse(JSON.stringify(value, (key, item) => (key === "span" ? null : item)));

const bindings = (
  ...pairs: ReadonlyArray<readonly [string, ReturnType<typeof lam>]>
): RecursiveBinding[] => pairs.map(([name, init]) => ({ name, init }));

describe("exercise 4.20: recursive bindings as a derived expression", () => {
  it("mutually recursive even?/odd? answers false for 5", () => {
    const even = lam(
      [param("n")],
      [
        exprStmt(
          cond(
            bin("===", ident("n"), num(0)),
            bool(true),
            call(ident("odd"), [bin("-", ident("n"), num(1))]),
          ),
        ),
      ],
    );
    const odd = lam(
      [param("n")],
      [
        exprStmt(
          cond(
            bin("===", ident("n"), num(0)),
            bool(false),
            call(ident("even"), [bin("-", ident("n"), num(1))]),
          ),
        ),
      ],
    );
    const node = recursiveNode(
      bindings(["even", even], ["odd", odd]),
      [exprStmt(call(ident("even"), [num(5)]))],
      noSpan,
    );
    const { session: engine, env } = session();
    expect(shown(evalWithRecursive(node, env, engine))).toBe("false");
  });

  it("factorial 10 by recursive binding is 3628800", () => {
    const fact = lam(
      [param("n")],
      [
        exprStmt(
          cond(
            bin("===", ident("n"), num(0)),
            num(1),
            bin("*", ident("n"), call(ident("fact"), [bin("-", ident("n"), num(1))])),
          ),
        ),
      ],
    );
    const node = recursiveNode(
      bindings(["fact", fact]),
      [exprStmt(call(ident("fact"), [num(10)]))],
      noSpan,
    );
    const { session: engine, env } = session();
    expect(shown(evalWithRecursive(node, env, engine))).toBe("3628800");
  });

  it("a nested recursive binding answers 42 for 21", () => {
    const double = lam([param("n")], [exprStmt(bin("*", num(2), ident("n")))]);
    const inner = recursiveNode(
      bindings(["double", double]),
      [exprStmt(call(ident("double"), [ident("x")]))],
      noSpan,
    );
    const f = lam([param("x")], [returnStmt(recursiveToCall(inner))]);
    const { session: engine, env } = session();
    expect(shown(evalWithRecursive(call(f, [num(21)]), env, engine))).toBe("42");
  });

  it("a plain grouped binding leaves its initializers outside", () => {
    const { session: engine, env } = session();
    const plain = call(lam([param("a"), param("b")], [exprStmt(ident("b"))]), [
      num(1),
      bin("+", ident("a"), num(1)),
    ]);
    expect(shown(evalWithRecursive(plain, env, engine))).toBe("error:unbound-name");
    const rec = recursiveNode(
      [
        { name: "a", init: num(1) },
        { name: "b", init: bin("+", ident("a"), num(1)) },
      ],
      [exprStmt(ident("b"))],
      noSpan,
    );
    expect(shown(evalWithRecursive(rec, env, engine))).toBe("2");
  });

  it("the derivation is the grouped-binding-with-writes shape", () => {
    const node = recursiveNode(
      [
        { name: "a", init: num(1) },
        { name: "b", init: bin("+", ident("a"), num(1)) },
      ],
      [exprStmt(bin("+", ident("a"), ident("b")))],
      noSpan,
    );
    const manual = call(
      lam(
        [],
        [
          varDecl("let", "a", num(1), null, noSpan),
          varDecl("let", "b", bin("+", ident("a"), num(1)), null, noSpan),
          exprStmt(bin("+", ident("a"), ident("b"))),
        ],
      ),
      [],
    );
    expect(normalize(recursiveToCall(node))).toEqual(normalize(manual));
  });
});
